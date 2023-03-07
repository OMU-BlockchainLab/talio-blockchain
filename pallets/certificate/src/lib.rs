#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{pallet_prelude::*, traits::{UnixTime, Randomness}};
use frame_system::pallet_prelude::*;
use pallet_account::EnsureRole;
use pallet_utils::{CerID, CerStatus, Role, RoleId};
use scale_info::TypeInfo;
use sp_runtime::traits::SaturatedConversion;
use sp_std::vec::Vec;
use sp_io::hashing::blake2_128;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	//#[scale_info(bounds(), skip_type_params(T))]
	pub struct Certificate {
		pub owner: RoleId,
		pub org: Option<RoleId>,
		pub score: Option<u32>,
		pub is_public: bool,
		pub created_date: u64,
		pub expired_date: Option<u64>,
		pub status: CerStatus,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_account::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CheckRole: EnsureRole<Self>;
		type UnixTime: UnixTime;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>; 
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn certificate_by_id)]
	pub type CertificateById<T> = StorageMap<_, Twox64Concat, CerID, Certificate, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_certificates)]
	pub type OwnerCertificates<T: Config> =
		StorageMap<_, Twox64Concat, RoleId, Vec<CerID>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CertificateCreated(CerID),
		CertificateRevoked(CerID),
		CertificateValidateSuccess(CerID),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoPermission,
		NotOwnerCertificate,
		NotExistCertificate,
		OrgNotFound,
		WrongOrgToValidateCer,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_certificate(
			origin: OriginFor<T>,
			owner: RoleId,
			org: Option<RoleId>,
			expired_date: Option<u64>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// T::CheckRole::ensure_org(&who)?;
			//ensure_root(origin)?;
			let now = T::UnixTime::now().as_millis().saturated_into::<u64>();
			let cid = Self::gen_random();
			let new_cer = Certificate {
				owner: owner.clone(),
				org,
				score: None,
				is_public: true,
				expired_date,
				created_date: now,
				status: CerStatus::Unvalidated,
			};
			<CertificateById<T>>::insert(&cid, new_cer);
			<OwnerCertificates<T>>::mutate(&owner, |cers| {
				cers.push(cid);
			});

			Self::deposit_event(Event::CertificateCreated(cid));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_certificate(
			origin: OriginFor<T>,
			cid: CerID,
			owner: RoleId,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			//ensure_root(origin)?;
			let owner_cers = Self::owner_certificates(&owner);
			ensure!(owner_cers.contains(&cid), Error::<T>::NotOwnerCertificate);
			<CertificateById<T>>::remove(&cid);
			<OwnerCertificates<T>>::mutate(&owner, |cers| {
				cers.retain(|x| x != &cid);
			});
			Self::deposit_event(Event::CertificateRevoked(cid));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn validate_certificate(
			origin: OriginFor<T>,
			role_id: RoleId,
			cid: CerID,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			//chẹck role is organization
			T::CheckRole::ensure_role(&role_id, Role::Organization)?;
			// check organization is validated
			T::CheckRole::ensure_validated(&role_id)?;
			if let Some(mut cer) = CertificateById::<T>::get(&cid) {
				if cer.org.is_none() {
					return Err(Error::<T>::OrgNotFound)?
				} else {
					if cer.org == Some(role_id) {
						// modify unvalidated into validated
						cer.status = CerStatus::Validated;
						CertificateById::<T>::insert(&cid, cer);
					} else {
						return Err(Error::<T>::WrongOrgToValidateCer)?
					}
				}
			} else {
				return Err(Error::<T>::NotExistCertificate)?
			}
			Self::deposit_event(Event::CertificateValidateSuccess(cid));
			Ok(())
		}
	}
}



impl<T:Config> Pallet<T>{

	fn gen_random() -> [u8;16]{

		let random = T::Randomness::random(&b"cert"[..]).0;
		// Create randomness payload
		let payload = (
			random,
			<frame_system::Pallet<T>>::extrinsic_index().unwrap_or_default(),
			<frame_system::Pallet<T>>::block_number(),
		);

		return payload.using_encoded(blake2_128)
	}
}