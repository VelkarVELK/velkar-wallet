use velkar_consensus_core::constants::*;
use velkar_consensus_core::network::NetworkType;
use separator::{Separatable, separated_float, separated_int, separated_uint_with_output};

#[inline]
pub fn sompi_to_velkar(sompi: u64) -> f64 {
    sompi as f64 / SOMPI_PER_VELKAR as f64
}

#[inline]
pub fn velkar_to_sompi(velkar: f64) -> u64 {
    (velkar * SOMPI_PER_VELKAR as f64) as u64
}

#[inline]
pub fn sompi_to_velkar_string(sompi: u64) -> String {
    sompi_to_velkar(sompi).separated_string()
}

#[inline]
pub fn sompi_to_velkar_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_velkar(sompi)))
}

pub fn velkar_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "VELK",
        NetworkType::Testnet => "TVELK",
        NetworkType::Simnet => "SVELK",
        NetworkType::Devnet => "DVELK",
    }
}

#[inline]
pub fn sompi_to_velkar_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let vlk = sompi_to_velkar_string(sompi);
    let suffix = velkar_suffix(network_type);
    format!("{vlk} {suffix}")
}

