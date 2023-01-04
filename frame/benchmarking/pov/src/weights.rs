
//! Autogenerated weights for frame_benchmarking_pallet_pov
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `oty-parity`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --dev
// --pallet
// frame-benchmarking-pallet-pov
// --extrinsic
// 
// --steps
// 50
// --repeat
// 20
// --template=.maintain/frame-weight-template.hbs
// --output=frame/benchmarking/pov/src/weights.rs
// --default-pov-mode
// max-encoded-len

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for frame_benchmarking_pallet_pov.
pub trait WeightInfo {
	fn storage_single_value_read() -> Weight;
	fn storage_single_value_read_twice() -> Weight;
	fn storage_single_value_write() -> Weight;
	fn storage_single_value_kill() -> Weight;
	fn storage_1m_map_read_one_value_two_additional_layers() -> Weight;
	fn storage_1m_map_read_one_value_three_additional_layers() -> Weight;
	fn storage_1m_map_read_one_value_four_additional_layers() -> Weight;
	fn storage_map_read_per_component(n: u32, m: u32, ) -> Weight;
	fn storage_1m_map_one_entry_repeated_read(n: u32, ) -> Weight;
	fn storage_1m_map_multiple_entry_repeated_read(n: u32, ) -> Weight;
	fn storage_1m_double_map_read_per_component(n: u32, ) -> Weight;
	fn storage_value_bounded_read() -> Weight;
	fn storage_value_unbounded_read() -> Weight;
	fn measured_storage_value_read_linear_size(l: u32, ) -> Weight;
	fn mel_storage_value_read_linear_size(l: u32, ) -> Weight;
	fn storage_value_bounded_and_unbounded_read() -> Weight;
	fn storage_map_unbounded_read() -> Weight;
	fn emit_event() -> Weight;
	fn noop() -> Weight;
}

/// Weights for frame_benchmarking_pallet_pov using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Pov Value (r:1 w:0)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `499`
		// Minimum execution time: 2_582 nanoseconds.
		Weight::from_parts(2_716_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Value (r:1 w:0)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_read_twice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `499`
		// Minimum execution time: 4_243 nanoseconds.
		Weight::from_parts(4_658_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Value (r:0 w:1)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_write() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 520 nanoseconds.
		Weight::from_ref_time(788_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Pov Value (r:0 w:1)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 490 nanoseconds.
		Weight::from_ref_time(535_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_two_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1275`
		//  Estimated: `3750`
		// Minimum execution time: 7_956 nanoseconds.
		Weight::from_parts(9_030_000, 3750)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_three_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1544`
		//  Estimated: `4019`
		// Minimum execution time: 8_771 nanoseconds.
		Weight::from_parts(10_028_000, 4019)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_four_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2044`
		//  Estimated: `4519`
		// Minimum execution time: 12_680 nanoseconds.
		Weight::from_parts(13_557_000, 4519)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:100 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// Storage: Pov Map16M (r:100 w:0)
	/// Proof: Pov Map16M (max_values: Some(16000000), max_size: Some(36), added: 3006, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	fn storage_map_read_per_component(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `515 + n * (188 ±0) + m * (188 ±0)`
		//  Estimated: `0 + m * (2511 ±0) + n * (3006 ±0)`
		// Minimum execution time: 272_284 nanoseconds.
		Weight::from_ref_time(169_894_842)
			// Standard Error: 19_162
			.saturating_add(Weight::from_ref_time(1_382_375).saturating_mul(n.into()))
			// Standard Error: 19_162
			.saturating_add(Weight::from_ref_time(1_459_384).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_proof_size(2511).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(3006).saturating_mul(n.into()))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn storage_1m_map_one_entry_repeated_read(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `2511`
		// Minimum execution time: 51 nanoseconds.
		Weight::from_parts(6_006_906, 2511)
			// Standard Error: 3_265
			.saturating_add(Weight::from_ref_time(398_091).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:100 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn storage_1m_map_multiple_entry_repeated_read(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147 + n * (40 ±0)`
		//  Estimated: `0 + n * (2511 ±0)`
		// Minimum execution time: 40 nanoseconds.
		Weight::from_ref_time(12_820_709)
			// Standard Error: 37_042
			.saturating_add(Weight::from_ref_time(5_849_901).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2511).saturating_mul(n.into()))
	}
	/// Storage: Pov DoubleMap1M (r:1024 w:0)
	/// Proof: Pov DoubleMap1M (max_values: Some(1000000), max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn storage_1m_double_map_read_per_component(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `21938 + n * (57 ±0)`
		//  Estimated: `0 + n * (2543 ±0)`
		// Minimum execution time: 468 nanoseconds.
		Weight::from_ref_time(101_899_233)
			// Standard Error: 4_574
			.saturating_add(Weight::from_ref_time(2_559_688).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2543).saturating_mul(n.into()))
	}
	/// Storage: Pov BoundedValue (r:1 w:0)
	/// Proof: Pov BoundedValue (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	fn storage_value_bounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `528`
		// Minimum execution time: 1_715 nanoseconds.
		Weight::from_parts(1_815_000, 528)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov UnboundedValue (r:1 w:0)
	/// Proof Skipped: Pov UnboundedValue (max_values: Some(1), max_size: None, mode: Measured)
	fn storage_value_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 1_701 nanoseconds.
		Weight::from_parts(1_798_000, 604)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov LargeValue (r:1 w:0)
	/// Proof: Pov LargeValue (max_values: Some(1), max_size: Some(4194308), added: 4194803, mode: Measured)
	/// The range of component `l` is `[0, 4194304]`.
	fn measured_storage_value_read_linear_size(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + l * (1 ±0)`
		//  Estimated: `666 + l * (1 ±0)`
		// Minimum execution time: 2_652 nanoseconds.
		Weight::from_parts(2_738_000, 666)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(304).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(l.into()))
	}
	/// Storage: Pov LargeValue (r:1 w:0)
	/// Proof: Pov LargeValue (max_values: Some(1), max_size: Some(4194308), added: 4194803, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 4194304]`.
	fn mel_storage_value_read_linear_size(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + l * (1 ±0)`
		//  Estimated: `4194803`
		// Minimum execution time: 2_634 nanoseconds.
		Weight::from_parts(2_671_000, 4194803)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(305).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Pov UnboundedValue (r:1 w:0)
	/// Proof Skipped: Pov UnboundedValue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Pov BoundedValue (r:1 w:0)
	/// Proof: Pov BoundedValue (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	fn storage_value_bounded_and_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1132`
		// Minimum execution time: 1_926 nanoseconds.
		Weight::from_parts(2_135_000, 1132)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: Pov UnboundedMap (r:1 w:0)
	/// Proof Skipped: Pov UnboundedMap (max_values: None, max_size: None, mode: Measured)
	fn storage_map_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2584`
		// Minimum execution time: 1_856 nanoseconds.
		Weight::from_parts(1_941_000, 2584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn emit_event() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_192 nanoseconds.
		Weight::from_ref_time(3_279_000)
	}
	fn noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_075 nanoseconds.
		Weight::from_ref_time(1_137_000)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Pov Value (r:1 w:0)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `499`
		// Minimum execution time: 2_582 nanoseconds.
		Weight::from_parts(2_716_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Value (r:1 w:0)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_read_twice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `499`
		// Minimum execution time: 4_243 nanoseconds.
		Weight::from_parts(4_658_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Value (r:0 w:1)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_write() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 520 nanoseconds.
		Weight::from_ref_time(788_000)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Pov Value (r:0 w:1)
	/// Proof: Pov Value (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn storage_single_value_kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 490 nanoseconds.
		Weight::from_ref_time(535_000)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_two_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1275`
		//  Estimated: `3750`
		// Minimum execution time: 7_956 nanoseconds.
		Weight::from_parts(9_030_000, 3750)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_three_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1544`
		//  Estimated: `4019`
		// Minimum execution time: 8_771 nanoseconds.
		Weight::from_parts(10_028_000, 4019)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: Measured)
	fn storage_1m_map_read_one_value_four_additional_layers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2044`
		//  Estimated: `4519`
		// Minimum execution time: 12_680 nanoseconds.
		Weight::from_parts(13_557_000, 4519)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:100 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// Storage: Pov Map16M (r:100 w:0)
	/// Proof: Pov Map16M (max_values: Some(16000000), max_size: Some(36), added: 3006, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	fn storage_map_read_per_component(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `515 + n * (188 ±0) + m * (188 ±0)`
		//  Estimated: `0 + m * (2511 ±0) + n * (3006 ±0)`
		// Minimum execution time: 272_284 nanoseconds.
		Weight::from_ref_time(169_894_842)
			// Standard Error: 19_162
			.saturating_add(Weight::from_ref_time(1_382_375).saturating_mul(n.into()))
			// Standard Error: 19_162
			.saturating_add(Weight::from_ref_time(1_459_384).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_proof_size(2511).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(3006).saturating_mul(n.into()))
	}
	/// Storage: Pov Map1M (r:1 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn storage_1m_map_one_entry_repeated_read(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `2511`
		// Minimum execution time: 51 nanoseconds.
		Weight::from_parts(6_006_906, 2511)
			// Standard Error: 3_265
			.saturating_add(Weight::from_ref_time(398_091).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov Map1M (r:100 w:0)
	/// Proof: Pov Map1M (max_values: Some(1000000), max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 100]`.
	fn storage_1m_map_multiple_entry_repeated_read(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147 + n * (40 ±0)`
		//  Estimated: `0 + n * (2511 ±0)`
		// Minimum execution time: 40 nanoseconds.
		Weight::from_ref_time(12_820_709)
			// Standard Error: 37_042
			.saturating_add(Weight::from_ref_time(5_849_901).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2511).saturating_mul(n.into()))
	}
	/// Storage: Pov DoubleMap1M (r:1024 w:0)
	/// Proof: Pov DoubleMap1M (max_values: Some(1000000), max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1024]`.
	fn storage_1m_double_map_read_per_component(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `21938 + n * (57 ±0)`
		//  Estimated: `0 + n * (2543 ±0)`
		// Minimum execution time: 468 nanoseconds.
		Weight::from_ref_time(101_899_233)
			// Standard Error: 4_574
			.saturating_add(Weight::from_ref_time(2_559_688).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(2543).saturating_mul(n.into()))
	}
	/// Storage: Pov BoundedValue (r:1 w:0)
	/// Proof: Pov BoundedValue (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	fn storage_value_bounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `528`
		// Minimum execution time: 1_715 nanoseconds.
		Weight::from_parts(1_815_000, 528)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov UnboundedValue (r:1 w:0)
	/// Proof Skipped: Pov UnboundedValue (max_values: Some(1), max_size: None, mode: Measured)
	fn storage_value_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 1_701 nanoseconds.
		Weight::from_parts(1_798_000, 604)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov LargeValue (r:1 w:0)
	/// Proof: Pov LargeValue (max_values: Some(1), max_size: Some(4194308), added: 4194803, mode: Measured)
	/// The range of component `l` is `[0, 4194304]`.
	fn measured_storage_value_read_linear_size(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + l * (1 ±0)`
		//  Estimated: `666 + l * (1 ±0)`
		// Minimum execution time: 2_652 nanoseconds.
		Weight::from_parts(2_738_000, 666)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(304).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_proof_size(1).saturating_mul(l.into()))
	}
	/// Storage: Pov LargeValue (r:1 w:0)
	/// Proof: Pov LargeValue (max_values: Some(1), max_size: Some(4194308), added: 4194803, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 4194304]`.
	fn mel_storage_value_read_linear_size(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174 + l * (1 ±0)`
		//  Estimated: `4194803`
		// Minimum execution time: 2_634 nanoseconds.
		Weight::from_parts(2_671_000, 4194803)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(305).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Pov UnboundedValue (r:1 w:0)
	/// Proof Skipped: Pov UnboundedValue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Pov BoundedValue (r:1 w:0)
	/// Proof: Pov BoundedValue (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	fn storage_value_bounded_and_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1132`
		// Minimum execution time: 1_926 nanoseconds.
		Weight::from_parts(2_135_000, 1132)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: Pov UnboundedMap (r:1 w:0)
	/// Proof Skipped: Pov UnboundedMap (max_values: None, max_size: None, mode: Measured)
	fn storage_map_unbounded_read() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `2584`
		// Minimum execution time: 1_856 nanoseconds.
		Weight::from_parts(1_941_000, 2584)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn emit_event() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_192 nanoseconds.
		Weight::from_ref_time(3_279_000)
	}
	fn noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_075 nanoseconds.
		Weight::from_ref_time(1_137_000)
	}
}