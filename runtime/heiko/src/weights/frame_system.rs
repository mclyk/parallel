
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-88-3-164`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("heiko-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parallel
// benchmark
// pallet
// --chain=heiko-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=frame_system
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/heiko/src/weights/frame_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(b: u32, ) -> Weight {
		// Minimum execution time: 6_842 nanoseconds.
		Weight::from_ref_time(4_467_938)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(534).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		// Minimum execution time: 22_622 nanoseconds.
		Weight::from_ref_time(19_476_218)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(2_294).saturating_mul(b.into()))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		// Minimum execution time: 14_493 nanoseconds.
		Weight::from_ref_time(14_783_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		// Minimum execution time: 6_691 nanoseconds.
		Weight::from_ref_time(6_790_000)
			// Standard Error: 2_840
			.saturating_add(Weight::from_ref_time(1_092_534).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		// Minimum execution time: 6_843 nanoseconds.
		Weight::from_ref_time(6_880_000)
			// Standard Error: 898
			.saturating_add(Weight::from_ref_time(751_055).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[0, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		// Minimum execution time: 9_746 nanoseconds.
		Weight::from_ref_time(9_918_000)
			// Standard Error: 1_372
			.saturating_add(Weight::from_ref_time(1_453_284).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
}
