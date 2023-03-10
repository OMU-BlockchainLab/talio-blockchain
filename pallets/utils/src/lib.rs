#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

use frame_support::inherent::Vec;

pub type CvId = [u8;36];
pub type TypeID = u32;
pub type CerID = [u8;16];
pub type UnixEpoch = u64;
pub type String = Vec<u8>;

pub type RoleId = [u8; 36];

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system as system;
	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};
	use sp_runtime::RuntimeDebug;
	use sp_std::prelude::*;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(
		feature = "std",
		serde(bound(serialize = "Account: Serialize, BlockNumber: Serialize, Time: Serialize"))
	)]
	#[cfg_attr(
		feature = "std",
		serde(bound(
			deserialize = "Account: Deserialize<'de>, BlockNumber: Deserialize<'de>, Time: Deserialize<'de>"
		))
	)]
	pub struct WhoAndWhen<Account, BlockNumber, Time> {
		pub account: Account,
		pub block: BlockNumber,
		pub time: Time,
	}

	impl<Account, BlockNumber, Time> WhoAndWhen<Account, BlockNumber, Time> {
		pub fn new(account: Account, block: BlockNumber, time: Time) -> Self {
			WhoAndWhen { account, block, time }
		}
	}

	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum User<AccountId> {
		Account(AccountId),
	}

	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Role {
		Organization,
		SysMan,
		User,
	}

	// Role status should be confirm by sysman
	// Everyone can register their role system, but can not allow until they confirmed by sysman
	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum RoleStatus {
		Active,
		Revoked,
		Pending,
	}

	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum CerStatus {
		Validated,
		Unvalidated,
	}

	// impl Default for Status {
	// 	fn default() -> Self {
	// 		Self::Active
	// 	}
	// }

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Deserialize))]
	#[cfg_attr(feature = "std", serde(tag = "contentType", content = "contentId"))]
	pub enum Content {
		/// No content.
		None,
		/// A raw vector of bytes.
		Raw(Vec<u8>),
		/// IPFS CID v0 of content.
		#[allow(clippy::upper_case_acronyms)]
		IPFS(Vec<u8>),
		/// Hypercore protocol (former DAT) id of content.
		Hyper(Vec<u8>),
	}

	impl From<Content> for Vec<u8> {
		fn from(content: Content) -> Vec<u8> {
			match content {
				Content::None => Vec::new(),
				Content::Raw(vec_u8) => vec_u8,
				Content::IPFS(vec_u8) => vec_u8,
				Content::Hyper(vec_u8) => vec_u8,
			}
		}
	}

	impl Default for Content {
		fn default() -> Self {
			Self::None
		}
	}

	impl Content {
		pub fn is_none(&self) -> bool {
			self == &Self::None
		}

		pub fn is_some(&self) -> bool {
			!self.is_none()
		}

		pub fn is_ipfs(&self) -> bool {
			matches!(self, Self::IPFS(_))
		}
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account is blocked in a given space.
		AccountIsBlocked,
		/// Content is blocked in a given space.
		ContentIsBlocked,
		/// Post is blocked in a given space.
		PostIsBlocked,
		/// IPFS CID is invalid.
		InvalidIpfsCid,
		/// `Raw` content type is not yet supported.
		RawContentTypeNotSupported,
		/// `Hyper` content type is not yet supported.
		HypercoreContentTypeNotSupported,
		/// Space handle is too short.
		HandleIsTooShort,
		/// Space handle is too long.
		HandleIsTooLong,
		/// Space handle contains invalid characters.
		HandleContainsInvalidChars,
		/// Content type is `None`.
		ContentIsEmpty,
	}
}
