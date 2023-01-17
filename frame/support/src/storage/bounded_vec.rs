// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Traits, types and structs to support putting a bounded vector into storage, as a raw value, map
//! or a double map.

use crate::{
	storage::{StorageDecodeLength, StorageTryAppend},
	traits::Get,
};
use crate::pallet_prelude::ValueQuery;
pub use sp_runtime::{BoundedSlice, BoundedVec};

impl<T, S> StorageDecodeLength for BoundedVec<T, S> {}

impl<T, S: Get<u32>> StorageTryAppend<T> for BoundedVec<T, S> {
	fn bound() -> usize {
		S::get() as usize
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::{bounded_vec, traits::ConstU32, Twox128};
	use sp_io::TestExternalities;

	#[crate::storage_alias]
	type Foo = StorageValue<Prefix, BoundedVec<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type ValueFoo = StorageValue<Prefix, BoundedVec<u32, ConstU32<7>>, ValueQuery>;

	#[crate::storage_alias]
	type FooMap = StorageMap<Prefix, Twox128, u32, BoundedVec<u32, ConstU32<7>>>;

	#[crate::storage_alias]
	type FooDoubleMap =
		StorageDoubleMap<Prefix, Twox128, u32, Twox128, u32, BoundedVec<u32, ConstU32<7>>>;

	use codec::Encode;

	#[test]
	fn decoding_with_shorter_bound_fails() {
		TestExternalities::default().execute_with(|| {
			// First we put a value in there and verify that it is there.
			let short: BoundedVec<u32, ConstU32<7>> = bounded_vec![1; 7];
			let long: BoundedVec<u32, ConstU32<8>> = bounded_vec![1; 8];
			Foo::put(&short);
			assert_eq!(Foo::get(), Some(short));

			// put into the raw storage of Foo
			crate::storage::unhashed::put_raw(&Foo::hashed_key(), &long.encode());
			// decode with a shorter bound
			assert!(Foo::get().is_none());
		});
	}

	#[test]
	fn decoding_with_shorter_bound_fails2() {
		TestExternalities::default().execute_with(|| {
			// First we put a value in there and verify that it is there.
			let short: BoundedVec<u32, ConstU32<7>> = bounded_vec![1; 7];
			let long: BoundedVec<u32, ConstU32<8>> = bounded_vec![1; 8];
			ValueFoo::put(&short);
			assert_eq!(ValueFoo::get(), short);

			// put into the raw storage of Foo
			crate::storage::unhashed::put_raw(&ValueFoo::hashed_key(), &long.encode());
			// decode with a shorter bound
			assert_eq!(ValueFoo::get(), BoundedVec::<u32, ConstU32<7>>::default());
		});
	}

	#[test]
	fn decode_len_works() {
		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			Foo::put(bounded);
			assert_eq!(Foo::decode_len().unwrap(), 3);
		});

		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			FooMap::insert(1, bounded);
			assert_eq!(FooMap::decode_len(1).unwrap(), 3);
			assert!(FooMap::decode_len(0).is_none());
			assert!(FooMap::decode_len(2).is_none());
		});

		TestExternalities::default().execute_with(|| {
			let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3];
			FooDoubleMap::insert(1, 1, bounded);
			assert_eq!(FooDoubleMap::decode_len(1, 1).unwrap(), 3);
			assert!(FooDoubleMap::decode_len(2, 1).is_none());
			assert!(FooDoubleMap::decode_len(1, 2).is_none());
			assert!(FooDoubleMap::decode_len(2, 2).is_none());
		});
	}
}
