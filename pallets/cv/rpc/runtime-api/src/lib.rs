#![cfg_attr(not(feature = "std"), no_std)]

use pallet_cv::Item;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait CvApi where Item: sp_api::Decode,

	{
		fn get_cv() -> Vec<Item>;
	}
}
