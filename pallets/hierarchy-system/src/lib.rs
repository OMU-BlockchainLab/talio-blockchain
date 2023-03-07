#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_utils::{RoleId, Role};
use sp_std::{borrow::ToOwned, vec, vec::Vec};
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod hierarchy;

#[cfg(feature = "std")]
use serde::{ser::Error as SerdeError, Deserialize, Deserializer, Serialize, Serializer};

pub use hierarchy::*;
use pallet_account::EnsureRole;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_account::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CheckRole: EnsureRole<Self>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn node_list)]
	pub type Node<T: Config> = StorageValue<_, Vec<RoleId>, ValueQuery>;

	// map parent => vec<child>
	#[pallet::storage]
	#[pallet::getter(fn tree)]
	pub(super) type Tree<T: Config> =
		StorageMap<_, Blake2_128Concat, RoleId, Vec<RoleId>, ValueQuery>;

	// relation ship with parent
	#[pallet::storage]
	#[pallet::getter(fn parents)]
	pub(super) type Parents<T: Config> =
		StorageMap<_, Blake2_128Concat, RoleId, RoleId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sys_role)]
	pub type SysRole<T: Config> = StorageValue<_, Vec<RoleId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn role_layer)]
	pub type RoleLayer<T: Config> = StorageMap<_, Blake2_128Concat, RoleId, u8, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ApproveSysMan(Vec<RoleId>),
		RevokeSysMan(Vec<RoleId>),
		ApproveOrg(Vec<RoleId>),
		RevokeOrg(Vec<RoleId>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotParent,
		ExistSysMan,
		NotFoundSysMan,
		NotSysMan,
		InvalidApprove,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub root_hierarchy: Option<UserId>,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> GenesisConfig {
			Self { root_hierarchy: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			let role_root_id = self.root_hierarchy.as_ref().unwrap();
			// set up hierarchy sysman for predefined account
			Pallet::<T>::initialize_hierarchy(role_root_id.0);


		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn approve_sysman(
			origin: OriginFor<T>,
			sys: RoleId,
			new_sysmans: Vec<RoleId>,
		) -> DispatchResult {
			//ensure_root(origin)?;
			let _ = ensure_signed(origin)?;
			Self::ensure_validated_sysman(&sys)?;
			

			for sysman_child in new_sysmans.iter() {
				
				// Check account is user or not
				T::CheckRole::ensure_role(&sysman_child,Role::SysMan)?;
				T::CheckRole::ensure_unvalidated(&sysman_child)?;
				Self::add_sub_node(sys,sysman_child);

				// Update new sysman into system
				Self::update_validated_sysman(sysman_child.clone())?;
				// validate role
				pallet_account::Pallet::<T>::validate_role(&sysman_child)?;

				// update map account id => role_id

				//<Role<T>>::insert(&sys, role_child_id);
			}

			Self::deposit_event(Event::ApproveSysMan(new_sysmans));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_sysman(
			origin: OriginFor<T>,
			sysman: RoleId,
			revoking_sysman: RoleId,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			Self::ensure_validated_sysman(&sysman)?;

			ensure!(Self::is_parent(sysman, &revoking_sysman), Error::<T>::NotParent);
			let childs_of_sysman = Self::iter_child(&revoking_sysman);

			// Remove sysman
			// Node
			Node::<T>::mutate(|node_list| {
				node_list.retain(|node| node != &revoking_sysman);
			});
			// Tree
			Tree::<T>::mutate(&sysman, |childs| {
				childs.retain(|child| child != &revoking_sysman);
			});
			// Parent
			Parents::<T>::remove(&revoking_sysman);

			// if sysman have own childs, we should remove it
			if !childs_of_sysman.is_empty() {
				for item in childs_of_sysman.iter() {
					Parents::<T>::remove(item);
				}
				Tree::<T>::remove(&revoking_sysman);

				// Update node info when remove child of sysman
				let diff = Self::find_difference(Self::node_list(), childs_of_sysman);
				Node::<T>::put(diff);
			}
			Self::remove_validated_sysman(&revoking_sysman)?;

			// unvalidate sysman role
			pallet_account::Pallet::<T>::unvalidate_role(&revoking_sysman)?;
			Self::deposit_event(Event::RevokeSysMan(vec![revoking_sysman]));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn approve_org(origin: OriginFor<T>, sys: RoleId, organizations: Vec<RoleId>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// Ensure real sysman that validated by parent sysman
			Self::ensure_validated_sysman(&sys)?;

			for id in organizations.iter() {

				T::CheckRole::ensure_role(&id,Role::Organization)?;

				// Ensure org registered but not validated
				T::CheckRole::ensure_unvalidated(id)?;

				// validate role
				pallet_account::Pallet::<T>::validate_role(id)?;
			}

			Self::deposit_event(Event::ApproveOrg(organizations));
			Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn revoke_org(origin: OriginFor<T>, sys: RoleId, orgs: Vec<RoleId>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			// Ensure organization


			// Ensure real sysman that validated by parent sysman
			Self::ensure_validated_sysman(&sys)?;

			// unvalidate org role
			for org in orgs.iter() {
				pallet_account::Pallet::<T>::unvalidate_role(org)?;
			}

			Self::deposit_event(Event::RevokeOrg(orgs));
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn initialize_hierarchy(role_root_id: RoleId) {
		Self::add_root_node(role_root_id);
		// insert role_id => layer 
		<RoleLayer<T>>::insert(&role_root_id, 1);

		// insert root => id
		//let sys_mans = pallet_account::Pallet::<T>::sys_role();
		let store_sysmans = vec![role_root_id];
		// for item in sys_mans.iter() {
		// 	let (user_id, account)  = item;
		// 	Self::add_sub_node(role_root_id, root.clone(), user_id.0, &account);
		// 	<Role<T>>::insert(account, user_id.0);
		// 	store_sysmans.push(account.to_owned());

		// }
		SysRole::<T>::put(store_sysmans);
	}

	fn find_difference(
		old_node_list: Vec<RoleId>,
		node_child_list: Vec<RoleId>,
	) -> Vec<RoleId> {
		let mut diff: Vec<RoleId> = Vec::new();
		let mut v_other: Vec<RoleId> = node_child_list.into_iter().collect();

		for e1 in old_node_list {
			if let Some(pos) = v_other.iter().position(|e2| e1 == *e2) {
				v_other.remove(pos);
			} else {
				diff.push(e1);
			}
		}
		diff.append(&mut v_other);
		diff
	}
	pub fn update_validated_sysman(who: RoleId) -> DispatchResult {
		let mut current_sysman = Self::sys_role();
		if current_sysman.contains(&who) {
			return Err(Error::<T>::ExistSysMan)?
		} else {
			current_sysman.push(who);
		}
		SysRole::<T>::put(current_sysman);
		Ok(())
	}

	pub fn remove_validated_sysman(who: &RoleId) -> DispatchResult {
		SysRole::<T>::try_mutate(|sysmans| -> DispatchResult {
			if !sysmans.contains(&who) {
				return Err(Error::<T>::NotFoundSysMan)?
			}
			sysmans.retain(|sysman| sysman != who);
			Ok(())
		})?;
		Ok(())
	}

	fn ensure_validated_sysman(who: &RoleId) -> DispatchResult {
		let sysmans = Self::sys_role();

		if sysmans.contains(who) {
			return Ok(())
		} else {
			return Err(Error::<T>::NotSysMan)?
		}
	}
}


pub struct UserId([u8; 36]);

#[cfg(feature = "std")]
impl Serialize for UserId {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		std::str::from_utf8(&self.0)
			.map_err(|e| S::Error::custom(format!("Debug buffer contains invalid UTF8 :{}", e)))?
			.serialize(serializer)
	}
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for UserId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;

		let bytes: [u8;36] = s.as_bytes().try_into().expect("Slice with incorrect length");

		Ok(UserId::from(bytes))
		// Ok(String::deserialize(deserializer)?
		// 	.as_bytes()
		// 	.try_into()
		// 	.expect("Slice with incorrect length"))
	}
}

#[cfg(feature = "std")]
impl From<[u8;36]> for UserId {
	fn from(item: [u8;36]) -> Self {
        UserId(item)
    }
}
