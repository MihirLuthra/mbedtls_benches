use structopt::StructOpt;
use super::operation::OperationType;
use mbedtls::{hash::Type as MdType, pk::EcGroupId};
use strum::{EnumString, Display};

#[derive(Debug, StructOpt)]
#[structopt(name = "mbedtls_benches", about = "Perform mbedtls benchmarks")]
pub struct Args {
    /// Thread count.
    #[structopt(short, long)]
    pub thread_count: u32,

    /// Number of operations per thread.
    #[structopt(short, long)]
    pub num_ops: u32,

    #[structopt(short, long)]
    /// Type of operation. Available operations: [Sign] - case insensitive
    pub operation_type: OperationType,

    #[structopt(short, long)]
    /// Key type. Available: [RSA, ECDSA] - case insensitive
    pub key_type: KeyType,

    #[structopt(short, long, parse(try_from_str = str_to_md_type), default_value = "sha256")]
    /// Key type. Available: [SHA256, SHA384, SHA512] - case insensitive
    pub md_type: MdType,

    #[structopt(short, long, parse(try_from_str = str_to_ec_group_id), default_value = "nistp256")]
    /// Ec curve. Available: [secp192r1, secp224r1, secp256r1, secp384r1, secp521r1] - case
    /// insensitive
    pub curve: EcGroupId,

    #[structopt(short, long, default_value = "2048")]
    pub key_size: u32,
}

fn str_to_md_type(s: &str) -> Result<MdType, String> {
    Ok(match s.to_lowercase().as_str() {
        "sha256" => MdType::Sha256,
        "sha384" => MdType::Sha384,
        "sha512" => MdType::Sha512,
        _ => return Err(format!("unsupported MdType: {}", s)),
    })
}

fn str_to_ec_group_id(s: &str) -> Result<EcGroupId, String> {
    Ok(match s.to_lowercase().as_str() {
        "secp192r1" | "nistp192" => EcGroupId::SecP192R1,
        "secp224r1" | "nistp224" => EcGroupId::SecP224R1,
        "secp256r1" | "nistp256" => EcGroupId::SecP256R1,
        "secp384r1" | "nistp384" => EcGroupId::SecP384R1,
        "secp521r1" | "nistp521" => EcGroupId::SecP521R1,
        _ => return Err(format!("unsupported curve: {}", s)),
    })
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum KeyType {
    Rsa,
    Ecdsa,
}
