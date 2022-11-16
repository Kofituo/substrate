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

//! Various basic types for use in the Nfts pallet.

use super::*;
use crate::macros::*;
use codec::EncodeLike;
use enumflags2::{bitflags, BitFlags};
use frame_support::{
	pallet_prelude::{BoundedVec, MaxEncodedLen},
	traits::Get,
};
use scale_info::{build::Fields, meta_type, Path, Type, TypeInfo, TypeParameter};

pub(super) type DepositBalanceOf<T, I = ()> =
	<<T as Config<I>>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;
pub(super) type CollectionDetailsFor<T, I> =
	CollectionDetails<<T as SystemConfig>::AccountId, DepositBalanceOf<T, I>>;
pub(super) type ApprovalsOf<T, I = ()> = BoundedBTreeMap<
	<T as SystemConfig>::AccountId,
	Option<<T as SystemConfig>::BlockNumber>,
	<T as Config<I>>::ApprovalsLimit,
>;
pub(super) type ItemDepositOf<T, I> =
	ItemDeposit<DepositBalanceOf<T, I>, <T as SystemConfig>::AccountId>;
pub(super) type ItemDetailsFor<T, I> =
	ItemDetails<<T as SystemConfig>::AccountId, ItemDepositOf<T, I>, ApprovalsOf<T, I>>;
pub(super) type BalanceOf<T, I = ()> =
	<<T as Config<I>>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;
pub(super) type ItemPrice<T, I = ()> = BalanceOf<T, I>;
pub(super) type ItemTipOf<T, I = ()> = ItemTip<
	<T as Config<I>>::CollectionId,
	<T as Config<I>>::ItemId,
	<T as SystemConfig>::AccountId,
	BalanceOf<T, I>,
>;
pub(super) type CollectionConfigFor<T, I = ()> = CollectionConfig<
	BalanceOf<T, I>,
	<T as SystemConfig>::BlockNumber,
	<T as Config<I>>::CollectionId,
>;

pub trait Incrementable {
	fn increment(&self) -> Self;
	fn initial_value() -> Self;
}
impl_incrementable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectionDetails<AccountId, DepositBalance> {
	/// Collection's owner.
	pub(super) owner: AccountId,
	/// The total balance deposited for the all storage associated with this collection.
	/// Used by `destroy`.
	pub(super) total_deposit: DepositBalance,
	/// The total number of outstanding items of this collection.
	pub(super) items: u32,
	/// The total number of outstanding item metadata of this collection.
	pub(super) item_metadatas: u32,
	/// The total number of attributes for this collection.
	pub(super) attributes: u32,
}

/// Witness data for the destroy transactions.
#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DestroyWitness {
	/// The total number of outstanding items of this collection.
	#[codec(compact)]
	pub items: u32,
	/// The total number of items in this collection that have outstanding item metadata.
	#[codec(compact)]
	pub item_metadatas: u32,
	/// The total number of attributes for this collection.
	#[codec(compact)]
	pub attributes: u32,
}

impl<AccountId, DepositBalance> CollectionDetails<AccountId, DepositBalance> {
	pub fn destroy_witness(&self) -> DestroyWitness {
		DestroyWitness {
			items: self.items,
			item_metadatas: self.item_metadatas,
			attributes: self.attributes,
		}
	}
}

/// Information concerning the ownership of a single unique item.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct ItemDetails<AccountId, Deposit, Approvals> {
	/// The owner of this item.
	pub(super) owner: AccountId,
	/// The approved transferrer of this item, if one is set.
	pub(super) approvals: Approvals,
	/// The amount held in the pallet's default account for this item. Free-hold items will have
	/// this as zero.
	pub(super) deposit: Deposit,
}

/// Information about the reserved item deposit.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ItemDeposit<DepositBalance, AccountId> {
	/// A depositor account.
	pub(super) account: AccountId,
	/// An amount that gets reserved.
	pub(super) amount: DepositBalance,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(StringLimit))]
#[codec(mel_bound(DepositBalance: MaxEncodedLen))]
pub struct CollectionMetadata<DepositBalance, StringLimit: Get<u32>> {
	/// The balance deposited for this metadata.
	///
	/// This pays for the data stored in this struct.
	pub(super) deposit: DepositBalance,
	/// General information concerning this collection. Limited in length by `StringLimit`. This
	/// will generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedVec<u8, StringLimit>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(StringLimit))]
#[codec(mel_bound(DepositBalance: MaxEncodedLen))]
pub struct ItemMetadata<DepositBalance, StringLimit: Get<u32>> {
	/// The balance deposited for this metadata.
	///
	/// This pays for the data stored in this struct.
	pub(super) deposit: DepositBalance,
	/// General information concerning this item. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedVec<u8, StringLimit>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ItemTip<CollectionId, ItemId, AccountId, Amount> {
	/// A collection of the item.
	pub(super) collection: CollectionId,
	/// An item of which the tip is send for.
	pub(super) item: ItemId,
	/// A sender of the tip.
	pub(super) receiver: AccountId,
	/// An amount the sender is willing to tip.
	pub(super) amount: Amount,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
pub struct PendingSwap<CollectionId, ItemId, ItemPriceWithDirection, Deadline> {
	/// A collection of the item user wants to receive.
	pub(super) desired_collection: CollectionId,
	/// An item user wants to receive.
	pub(super) desired_item: Option<ItemId>,
	/// A price for the desired `item` with the direction.
	pub(super) price: Option<ItemPriceWithDirection>,
	/// An optional deadline for the swap.
	pub(super) deadline: Deadline,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PriceDirection {
	Send,
	Receive,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PriceWithDirection<Amount> {
	/// An amount.
	pub(super) amount: Amount,
	/// A direction (send or receive).
	pub(super) direction: PriceDirection,
}

/// Support for up to 64 user-enabled features on a collection.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum CollectionSetting {
	/// Items in this collection are transferable.
	TransferableItems,
	/// The metadata of this collection can be modified.
	UnlockedMetadata,
	/// Attributes of this collection can be modified.
	UnlockedAttributes,
	/// The supply of this collection can be modified.
	UnlockedMaxSupply,
	/// When this isn't set then the deposit is required to hold the items of this collection.
	DepositRequired,
}

/// Wrapper type for `BitFlags<CollectionSetting>` that implements `Codec`.
#[derive(Clone, Copy, PartialEq, Eq, Default, RuntimeDebug)]
pub struct CollectionSettings(pub BitFlags<CollectionSetting>);

impl CollectionSettings {
	pub fn all_enabled() -> Self {
		Self(BitFlags::EMPTY)
	}
	pub fn get_disabled(&self) -> BitFlags<CollectionSetting> {
		self.0
	}
	pub fn is_disabled(&self, setting: CollectionSetting) -> bool {
		self.0.contains(setting)
	}
	pub fn from_disabled(settings: BitFlags<CollectionSetting>) -> Self {
		Self(settings)
	}
}

impl_codec_bitflags!(CollectionSettings, u64, CollectionSetting);

/// Mint type. Can the NFT be create by anyone, or only the creator of the collection,
/// or only by wallets that already hold an NFT from a certain collection?
/// The ownership of a privately minted NFT is still publicly visible.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum MintType<CollectionId> {
	/// Only an `Issuer` could mint items.
	Issuer,
	/// Anyone could mint items.
	Public,
	/// Only holders of items in specified collection could mint new items.
	HolderOf(CollectionId),
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MintSettings<Price, BlockNumber, CollectionId> {
	/// Whether anyone can mint or if minters are restricted to some subset.
	pub(super) mint_type: MintType<CollectionId>,
	/// An optional price per mint.
	pub(super) price: Option<Price>,
	/// When the mint starts.
	pub(super) start_block: Option<BlockNumber>,
	/// When the mint ends.
	pub(super) end_block: Option<BlockNumber>,
	/// Default settings each item will get during the mint.
	pub(super) default_item_settings: ItemSettings,
}

impl<Price, BlockNumber, CollectionId> Default for MintSettings<Price, BlockNumber, CollectionId> {
	fn default() -> Self {
		Self {
			mint_type: MintType::Issuer,
			price: None,
			start_block: None,
			end_block: None,
			default_item_settings: ItemSettings::all_enabled(),
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct MintWitness<ItemId> {
	/// Provide the id of the item in a required collection.
	pub owner_of_item: ItemId,
}

#[derive(
	Clone, Copy, Decode, Default, Encode, MaxEncodedLen, PartialEq, RuntimeDebug, TypeInfo,
)]
pub struct CollectionConfig<Price, BlockNumber, CollectionId> {
	/// Collection's settings.
	pub(super) settings: CollectionSettings,
	/// Collection's max supply.
	pub(super) max_supply: Option<u32>,
	/// Default settings each item will get during the mint.
	pub(super) mint_settings: MintSettings<Price, BlockNumber, CollectionId>,
}

impl<Price, BlockNumber, CollectionId> CollectionConfig<Price, BlockNumber, CollectionId> {
	pub fn is_setting_enabled(&self, setting: CollectionSetting) -> bool {
		!self.settings.is_disabled(setting)
	}
	pub fn has_disabled_setting(&self, setting: CollectionSetting) -> bool {
		self.settings.is_disabled(setting)
	}
	pub fn enable_setting(&mut self, setting: CollectionSetting) {
		self.settings.0.remove(setting);
	}
	pub fn disable_setting(&mut self, setting: CollectionSetting) {
		self.settings.0.insert(setting);
	}
}

/// Support for up to 64 user-enabled features on an item.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum ItemSetting {
	/// This item is transferable.
	Transferable,
	/// The metadata of this item can be modified.
	UnlockedMetadata,
	/// Attributes of this item can be modified.
	UnlockedAttributes,
}

/// Wrapper type for `BitFlags<ItemSetting>` that implements `Codec`.
#[derive(Clone, Copy, PartialEq, Eq, Default, RuntimeDebug)]
pub struct ItemSettings(pub BitFlags<ItemSetting>);

impl ItemSettings {
	pub fn all_enabled() -> Self {
		Self(BitFlags::EMPTY)
	}
	pub fn get_disabled(&self) -> BitFlags<ItemSetting> {
		self.0
	}
	pub fn is_disabled(&self, setting: ItemSetting) -> bool {
		self.0.contains(setting)
	}
	pub fn from_disabled(settings: BitFlags<ItemSetting>) -> Self {
		Self(settings)
	}
}

impl_codec_bitflags!(ItemSettings, u64, ItemSetting);

#[derive(
	Encode, Decode, Default, PartialEq, RuntimeDebug, Clone, Copy, MaxEncodedLen, TypeInfo,
)]
pub struct ItemConfig {
	/// Item's settings.
	pub(super) settings: ItemSettings,
}

impl ItemConfig {
	pub fn is_setting_enabled(&self, setting: ItemSetting) -> bool {
		!self.settings.is_disabled(setting)
	}
	pub fn has_disabled_setting(&self, setting: ItemSetting) -> bool {
		self.settings.is_disabled(setting)
	}
	pub fn has_disabled_settings(&self) -> bool {
		!self.settings.get_disabled().is_empty()
	}
	pub fn enable_setting(&mut self, setting: ItemSetting) {
		self.settings.0.remove(setting);
	}
	pub fn disable_setting(&mut self, setting: ItemSetting) {
		self.settings.0.insert(setting);
	}
}

/// Support for up to 64 system-enabled features on a collection.
#[bitflags]
#[repr(u64)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum PalletFeature {
	/// Enable/disable trading operations.
	Trading,
	/// Allow/disallow setting attributes.
	Attributes,
	/// Allow/disallow transfer approvals.
	Approvals,
	/// Allow/disallow atomic items swap.
	Swaps,
}

/// Wrapper type for `BitFlags<PalletFeature>` that implements `Codec`.
#[derive(Default, RuntimeDebug)]
pub struct PalletFeatures(pub BitFlags<PalletFeature>);

impl PalletFeatures {
	pub fn all_enabled() -> Self {
		Self(BitFlags::EMPTY)
	}
	pub fn from_disabled(features: BitFlags<PalletFeature>) -> Self {
		Self(features)
	}
	pub fn is_enabled(&self, feature: PalletFeature) -> bool {
		!self.0.contains(feature)
	}
}
impl_codec_bitflags!(PalletFeatures, u64, PalletFeature);

/// Support for up to 8 different roles for collections.
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, RuntimeDebug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum CollectionRole {
	/// Can mint items.
	Issuer,
	/// Can freeze items.
	Freezer,
	/// Can thaw items, force transfers and burn items from any account.
	Admin,
}

/// A wrapper type that implements `Codec`.
#[derive(Clone, Copy, PartialEq, Eq, Default, RuntimeDebug)]
pub struct CollectionRoles(pub BitFlags<CollectionRole>);

impl CollectionRoles {
	pub fn none() -> Self {
		Self(BitFlags::EMPTY)
	}
	pub fn has_role(&self, role: CollectionRole) -> bool {
		self.0.contains(role)
	}
	pub fn add_role(&mut self, role: CollectionRole) {
		self.0.insert(role);
	}
	pub fn max_roles() -> u8 {
		let all: BitFlags<CollectionRole> = BitFlags::all();
		all.len() as u8
	}
}
impl_codec_bitflags!(CollectionRoles, u8, CollectionRole);