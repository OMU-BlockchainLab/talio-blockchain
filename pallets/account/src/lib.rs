#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_utils::{Role, RoleId, RoleStatus};
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(bounds(), skip_type_params(T))]
	pub struct Account {
		pub role: Role,
		pub status: RoleStatus,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn account_storage)]
	pub type AccountStorage<T: Config> = StorageMap<_, Twox64Concat, RoleId, Account, OptionQuery>;


	// map role id => wallet
	#[pallet::storage]
	#[pallet::getter(fn role_account)]
	pub type RoleAccount<T: Config> = StorageMap<_, Blake2_128Concat, RoleId, T::AccountId, OptionQuery>;

	//userA -> Normal User
	// userB -> Org
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountRegisted([u8; 36]),

		AccountUpdated([u8; 36]),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account is already Registered
		AlreadyRegistered,
		NotExactRole,
		/// Account is not Registered
		AccountNotRegistered,
		ActiveRole,
		CanNotRevoke,
		UserNotNeedValidate,
		RoleNotValidate,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// register account
		// only root
		#[pallet::weight(10_000)]
		pub fn register_account(origin: OriginFor<T>, id: RoleId, role: Role) -> DispatchResult {
			ensure_root(origin)?;

			if let Some(_) = <AccountStorage<T>>::get(&id) {
				return Err(Error::<T>::AlreadyRegistered)?
			} else {
				<AccountStorage<T>>::insert(&id, Account { role, status: RoleStatus::Pending });
			}

			// Return a successful DispatchResultWithPostInfo
			Self::deposit_event(Event::AccountRegisted(id));
			Ok(())
		}

		// Only update their role
		// only root
		#[pallet::weight(10_000)]
		pub fn update_role(origin: OriginFor<T>, id: RoleId, role: Role) -> DispatchResult {
			ensure_root(origin)?;

			<AccountStorage<T>>::try_mutate(&id, |acc| {
				if let Some(account) = acc {
					account.role = role;
				} else {
					return Err(Error::<T>::AccountNotRegistered)
				}
				Ok(())
			})?;

			Ok(())
		}

		// update wallet in blockchain
		#[pallet::weight(10_000)]
		pub fn update_account(origin: OriginFor<T>, id: RoleId, wallet: T::AccountId) -> DispatchResult{
			ensure_root(origin)?;

			<RoleAccount<T>>::mutate(&id, |account_id|{
				*account_id = Some(wallet);
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn validate_role(id: &RoleId) -> DispatchResult {
		<AccountStorage<T>>::mutate(id, |acc| {
			if let Some(account) = acc {
				account.status = RoleStatus::Active;
			}
		});
		Ok(())
	}
	pub fn unvalidate_role(id: &RoleId) -> DispatchResult {
		<AccountStorage<T>>::try_mutate(id, |acc| -> DispatchResult {
			if let Some(account) = acc {
				if account.status == RoleStatus::Active {
					account.status = RoleStatus::Revoked;
					return Ok(())
				} else {
					return Err(Error::<T>::CanNotRevoke)?
				}
			} else {
				return Err(Error::<T>::AccountNotRegistered)?
			}
		})?;
		Ok(())
	}
}

pub trait EnsureRole<T: Config> {
	fn ensure_role(id: &RoleId, role: Role) -> DispatchResult;
	fn ensure_unvalidated(id: &RoleId) -> DispatchResult;
	fn ensure_validated(id: &RoleId) -> DispatchResult;
}

impl<T: Config> EnsureRole<T> for Pallet<T> {
	fn ensure_role(id: &RoleId, role: Role) -> DispatchResult {
		if let Some(account) = <AccountStorage<T>>::get(id) {
			if account.role == role {
				Ok(())
			} else {
				return Err(Error::<T>::NotExactRole)?
			}
		} else {
			return Err(Error::<T>::AccountNotRegistered)?
		}
	}
	fn ensure_unvalidated(id: &RoleId) -> DispatchResult {
		if let Some(account) = <AccountStorage<T>>::get(id) {
			if account.role == Role::User {
				return Err(Error::<T>::UserNotNeedValidate)?
			} else {
				if account.status == RoleStatus::Pending || account.status == RoleStatus::Revoked {
					Ok(())
				} else {
					return Err(Error::<T>::ActiveRole)?
				}
			}
		} else {
			return Err(Error::<T>::AccountNotRegistered)?
		}
	}

	fn ensure_validated(id: &RoleId) -> DispatchResult {
		if let Some(account) = <AccountStorage<T>>::get(id) {
			if account.status == RoleStatus::Active {
				Ok(())
			} else {
				return Err(Error::<T>::RoleNotValidate)?
			}
		} else {
			return Err(Error::<T>::AccountNotRegistered)?
		}
	}
}
