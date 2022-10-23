use std::cell::UnsafeCell;

use mbedtls::pk::{Pk, EcGroup, EcGroupId, ECDSA_MAX_LEN};
use mbedtls::rng::Rdrand;
use mbedtls::hash::{Type as MdType, MdInfo};
use once_cell::sync::Lazy;
use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};

thread_local! {
    pub static SMALL_RNG: Lazy<UnsafeCell<SmallRng>> = Lazy::new(|| {
        fn seed() {}
        let seed = seed as *const fn() as u64;
        let rng = rand::rngs::SmallRng::seed_from_u64(seed);
        UnsafeCell::new(rng)
    });
}

pub fn fill_random_bytes(buffer: &mut [u8]) {
    unsafe { SMALL_RNG.with(|r| r.get().as_mut().unwrap().fill_bytes(buffer)) }
}

pub fn create_ec_from_curve(curve: EcGroupId) -> Pk {
    let ec_group = EcGroup::new(curve).unwrap();
    Pk::generate_ec(&mut Rdrand, ec_group).unwrap()
}

pub fn create_rsa_from_size(size: u32) -> Pk {
    const EXPONENT: u32 = 0x10001;
    Pk::generate_rsa(&mut Rdrand, size, EXPONENT).unwrap()
}

pub fn dummy_ec_sign(pk: &mut Pk, md_type: MdType) {
    dummy_sign(pk, ECDSA_MAX_LEN, md_type)
}

pub fn dummy_rsa_sign(pk: &mut Pk, md_type: MdType) {
    dummy_sign(pk, pk.len() / 8, md_type)
}

pub fn dummy_sign(pk: &mut Pk, sig_buffer_size: usize, md_type: MdType) {
    let md_info = Into::<Option<MdInfo>>::into(md_type).expect("unsupported MdType");
    let data_size = md_info.size();

    let mut data = vec![0; data_size];
    fill_random_bytes(&mut data);

    let mut sig = vec![0u8; sig_buffer_size];

    let res_len = pk.sign(md_type, &data, &mut sig, &mut Rdrand).unwrap();

    sig.truncate(res_len);
}
