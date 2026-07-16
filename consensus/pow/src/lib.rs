// public for benchmarks
#[doc(hidden)]
pub mod matrix;
#[cfg(feature = "wasm32-sdk")]
pub mod wasm;
#[doc(hidden)]
pub mod xoshiro;

use std::cmp::max;

use argon2::{Algorithm, Argon2, Params, Version};
use crate::matrix::Matrix;
use velkar_consensus_core::{BlockLevel, hashing, header::Header};
use velkar_hashes::{KHeavyHash, PowHash, ProofOfWorkHash};
use velkar_math::Uint256;

const ARGON_MEMORY_KIB: u32 = 8 * 1024;
const ARGON_TIME_COST: u32 = 1;
const ARGON_LANES: u32 = 1;

/// State is an intermediate data structure with pre-computed values to speed up mining.
pub struct State {
    pub(crate) matrix: Matrix,
    pub(crate) target: Uint256,
    // PRE_POW_HASH || TIME || 32 zero byte padding; without NONCE
    pub(crate) hasher: PowHash,
}

impl State {
    #[inline]
    pub fn new(header: &Header) -> Self {
        let target = Uint256::from_compact_target_bits(header.bits);
        // Zero out the time and nonce.
        let pre_pow_hash = hashing::header::hash_override_nonce_time(header, 0, 0);
        // PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
        let hasher = PowHash::new(pre_pow_hash, header.timestamp);
        let matrix = Matrix::generate(pre_pow_hash);

        Self { matrix, target, hasher }
    }

    #[inline]
    #[must_use]
    /// PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
    pub fn calculate_pow(&self, nonce: u64) -> Uint256 {
        // VelkarHash:
        // 1) cSHAKE-based pre-PoW hash stage
        // 2) matrix heavy-hash stage
        // 3) extra cSHAKE heavy-hash stage
        // 4) final domain-separated squeeze and word-mixing
        let stage1 = self.hasher.clone().finalize_with_nonce(nonce);
        let stage2 = self.matrix.heavy_hash(stage1);
        let stage3 = KHeavyHash::hash(stage2);
        let stage4 = memory_hard_hash(stage3, stage1, nonce, self.target);

        let mut final_hasher = ProofOfWorkHash::new();
        final_hasher.write(stage4.as_bytes());
        final_hasher.write(nonce.to_le_bytes());
        final_hasher.write(self.target.to_le_bytes());
        let stage5 = final_hasher.finalize();

        let mut words = stage5.to_le_u64();
        words[0] ^= stage1.to_le_u64()[0].rotate_left(17);
        words[1] ^= stage2.to_le_u64()[1].rotate_right(11);
        words[2] ^= stage3.to_le_u64()[2].rotate_left(7);
        words[3] ^= stage4.to_le_u64()[3].rotate_left(13) ^ nonce.rotate_left(13);

        Uint256::from_le_bytes(velkar_hashes::Hash::from_le_u64(words).as_bytes())
    }

    #[inline]
    #[must_use]
    pub fn check_pow(&self, nonce: u64) -> (bool, Uint256) {
        let pow = self.calculate_pow(nonce);
        // The pow hash must be less or equal than the claimed target.
        (pow <= self.target, pow)
    }
}

pub fn calc_block_level(header: &Header, max_block_level: BlockLevel) -> BlockLevel {
    let (block_level, _) = calc_block_level_check_pow(header, max_block_level);
    block_level
}

pub fn calc_block_level_check_pow(header: &Header, max_block_level: BlockLevel) -> (BlockLevel, bool) {
    if header.parents_by_level.is_empty() {
        return (max_block_level, true); // Genesis has the max block level
    }

    let state = State::new(header);
    let (passed, pow) = state.check_pow(header.nonce);
    let block_level = calc_level_from_pow(pow, max_block_level);
    (block_level, passed)
}

pub fn calc_level_from_pow(pow: Uint256, max_block_level: BlockLevel) -> BlockLevel {
    let signed_block_level = max_block_level as i64 - pow.bits() as i64;
    max(signed_block_level, 0) as BlockLevel
}

fn memory_hard_hash(stage3: velkar_hashes::Hash, stage1: velkar_hashes::Hash, nonce: u64, target: Uint256) -> velkar_hashes::Hash {
    let params = Params::new(ARGON_MEMORY_KIB, ARGON_TIME_COST, ARGON_LANES, Some(32)).expect("valid argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut password = [0u8; 72];
    password[..32].copy_from_slice(&stage3.as_bytes());
    password[32..64].copy_from_slice(&target.to_le_bytes());
    password[64..72].copy_from_slice(&nonce.to_le_bytes());

    let stage1_bytes = stage1.as_bytes();
    let stage3_bytes = stage3.as_bytes();
    let mut salt = [0u8; 16];
    for i in 0..16 {
        salt[i] = stage1_bytes[i] ^ stage3_bytes[i + 16];
    }

    let mut out = [0u8; 32];
    argon2.hash_password_into(&password, &salt, &mut out).expect("argon2 output buffer has the requested size");
    velkar_hashes::Hash::from_bytes(out)
}
