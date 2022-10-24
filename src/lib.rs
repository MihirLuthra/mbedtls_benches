mod test_rng;

use mbedtls::pk::{Pk, EcGroup, EcGroupId, ECDSA_MAX_LEN};
use mbedtls::rng::Random;
use mbedtls::hash::{Type as MdType, MdInfo};
use test_rng::TestRng;

pub fn create_ec_from_curve(curve: EcGroupId) -> Pk {
    let ec_group = EcGroup::new(curve).unwrap();
    Pk::generate_ec(&mut TestRng, ec_group).unwrap()
}

pub fn create_rsa_from_size(size: u32) -> Pk {
    const EXPONENT: u32 = 0x10001;
    Pk::generate_rsa(&mut TestRng, size, EXPONENT).unwrap()
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
    TestRng.random(&mut data).unwrap();

    let mut sig = vec![0u8; sig_buffer_size];

    let res_len = pk.sign(md_type, &data, &mut sig, &mut TestRng).unwrap();

    sig.truncate(res_len);
}
