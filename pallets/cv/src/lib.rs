#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

use frame_support::{inherent::Vec, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use pallet_utils::{CerID, CvId, RoleId};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	// #[cfg_attr(
	// 	feature = "std",
	// 	serde(bound(serialize = "Account: Serialize, BlockNumber: Serialize, Time: Serialize"))
	// )]
	// #[cfg_attr(
	// 	feature = "std",
	// 	serde(bound(
	// 		deserialize = "Account: Deserialize<'de>, BlockNumber: Deserialize<'de>, Time:
	// Deserialize<'de>" 	))
	// )]
	pub struct Item {
		#[cfg_attr(feature = "std", serde(with = "as_string"))]
		pub item_id: CvId,
		#[cfg_attr(feature = "std", serde(with = "as_string"))]
		pub owner: RoleId,
		pub certificate_id: Option<Vec<CerID>>,
		pub score: Option<u32>,
	}

	impl Item {
		pub fn new(
			id: CvId,
			owner: RoleId,
			certificate_id: Option<Vec<CerID>>,
			score: Option<u32>,
		) -> Self {
			Item { item_id: id, owner, certificate_id, score }
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_utils::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn item)]
	pub type Items<T: Config> = StorageMap<_, Twox64Concat, CvId, Item, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_cv)]
	pub type OwnerCv<T: Config> = StorageMap<_, Twox64Concat, RoleId, Vec<CvId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RevokeSucceed(CvId),
		CreateSucceed(CvId),
		ItemUpdated(CvId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ItemNotFound,
		OwnerNotFound,
		ItemUpdateFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_item(
			origin: OriginFor<T>,
			cv_id: CvId,
			_owner: RoleId,
			_certificated_id: Option<Vec<CerID>>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;
			let new_item: Item = Item::new(cv_id.clone(), _owner.clone(), _certificated_id, None);

			// add cv item to storage along with its id
			<Items<T>>::insert(&cv_id, new_item);

			// track item by account id
			<OwnerCv<T>>::mutate(_owner.clone(), |x| x.push(cv_id.clone()));

			// Emit an event.
			Self::deposit_event(Event::CreateSucceed(cv_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_item(
			origin: OriginFor<T>,
			_item_id: CvId,
			owner: RoleId,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			let item_idx = Self::owner_cv(&owner).iter().position(|x| *x == _item_id);
			ensure!(item_idx != None, Error::<T>::ItemNotFound);
			if let Some(iid) = item_idx {
				<OwnerCv<T>>::mutate(&owner, |x| x.swap_remove(iid));
			}
			<Items<T>>::remove(&_item_id);
			// Emit an event.
			Self::deposit_event(Event::RevokeSucceed(_item_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn update_item(
			origin: OriginFor<T>,
			_item_id: CvId,
			_owner: RoleId,
			_certificated_id: Option<Vec<CerID>>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin)?;

			// find item by id
			ensure!(Items::<T>::contains_key(&_item_id), Error::<T>::ItemNotFound);

			let is_owner = Self::check_permission(&_item_id, _owner.clone());
			// check permission
			ensure!(is_owner, Error::<T>::OwnerNotFound);

			// update item in storage
			Items::<T>::try_mutate(&_item_id, |item| -> DispatchResult {
				if let Some(cv) = item {
					cv.owner = _owner;
					cv.certificate_id = _certificated_id;
					return Ok(())
				} else {
					return Err(Error::<T>::ItemUpdateFailed)?
				}
			})?;

			Self::deposit_event(Event::<T>::ItemUpdated(_item_id));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn check_permission(item_id: &CvId, owner: RoleId) -> bool {
			let item = Items::<T>::get(item_id);
			// if item.is_some() {
			// 	let item = item.unwrap();
			// 	return item.owner == owner
			// }
			// false
			if let Some(cv) = item {
				if cv.owner == owner {
					return true
				} else {
					return false
				}
			} else {
				return false
			}
		}

		pub fn get_cv() -> Vec<Item> {
			Items::<T>::iter_values().collect()
		}
	}
}

#[cfg(feature = "std")]
mod as_string {
	use super::*;
	use serde::{ser::Error, Deserializer, Serializer};

	pub fn serialize<S: Serializer>(bytes: &[u8; 36], serializer: S) -> Result<S::Ok, S::Error> {
		std::str::from_utf8(bytes)
			.map_err(|e| S::Error::custom(format!("Debug buffer contains invalid UTF8 :{}", e)))?
			.serialize(serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 36], D::Error> {
		Ok(String::deserialize(deserializer)?
			.as_bytes()
			.try_into()
			.expect("Slice with incorrect length"))
	}
}
