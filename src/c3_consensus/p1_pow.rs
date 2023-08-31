//! Proof of Work provides security to the blockchain by requiring block authors
//! to expend a real-world scarce resource, namely energy, in order to author a valid block.
//!
//! This is the same logic we implemented previously. Here we re-implement it in the
//! generic consensus framework that we will use throughout the rest of the chapter.

use super::{Consensus, Header};

/// A Proof of Work consensus engine. This is the same consensus logic that we
/// implemented in the previous chapter. Here we simply re-implement it in the
/// consensus framework that will be used throughout this chapter.
pub struct PoW {
	threshold: u64,
}

impl Consensus for PoW {
	type Digest = u64;

	/// Check that the provided header's hash is below the required threshold.
	/// This does not rely on the parent digest at all.
	fn validate(&self, _: &Self::Digest, header: &Header<Self::Digest>) -> bool {
		todo!("Exercise 1")
	}

	/// Mine a new PoW seal for the partial header provided.
	/// This does not rely on the parent digest at all.
	fn seal(&self, _: &Self::Digest, partial_header: Header<()>) -> Option<Header<Self::Digest>> {
		todo!("Exercise 2")
	}
}

/// Create a PoW consensus engine that has a difficulty threshold such that roughly 1 in 100 blocks
/// with randomly drawn nonces will be valid. That is: the threshold should be u64::max_value() /
/// 100.
pub fn moderate_difficulty_pow() -> impl Consensus {
	todo!("Exercise 3")
}
