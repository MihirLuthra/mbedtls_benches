use mbedtls::rng::{RngCallback, RngCallbackMut};
use mbedtls_sys::types::raw_types::c_void;
use mbedtls_sys::types::size_t;
use std::os::raw::{c_uchar, c_int};

pub mod small_rng {
    use std::cell::UnsafeCell;
    use once_cell::sync::Lazy;
    use rand::rngs::SmallRng;
    use rand::{SeedableRng, RngCore};
    use super::*;

    thread_local! {
        pub static SMALL_RNG: Lazy<UnsafeCell<SmallRng>> = Lazy::new(|| {
            fn seed() {}
            let seed = seed as *const fn() as u64;
            let rng = rand::rngs::SmallRng::seed_from_u64(seed);
            UnsafeCell::new(rng)
        });
    }

    fn fill_random_bytes(buffer: &mut [u8]) {
        unsafe { SMALL_RNG.with(|r| r.get().as_mut().unwrap().fill_bytes(buffer)) }
    }

    pub struct TestRng;

    impl RngCallbackMut for TestRng {
        unsafe extern "C" fn call_mut(_user_data: *mut c_void, data: *mut c_uchar, len: size_t) -> c_int where Self: Sized {
            fill_random_bytes(core::slice::from_raw_parts_mut(data, len));
            0
        }

        fn data_ptr_mut(&mut self) ->  *mut c_void {
            self as *mut _ as *mut _
        }
    }

    impl RngCallback for TestRng {
        unsafe extern "C" fn call(_user_data: *mut c_void, data: *mut c_uchar, len: size_t) -> c_int where Self: Sized {
            fill_random_bytes(core::slice::from_raw_parts_mut(data, len));
            0
        }

        fn data_ptr(&self) ->  *mut c_void {
            self as *const _ as *mut _
        }
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub use small_rng::TestRng;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use mbedtls::rng::Rdrand as TestRng;
