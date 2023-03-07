use hex_literal::hex;
use node_primitives::{AccountId, Balance};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use scv_node::currency::SCV;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

use sp_core::crypto::UncheckedInto;
use sp_finality_grandpa::AuthorityId as GrandpaId;

/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	//sudo key: 5HTSDGMrfHCjBs27BzXc1fomyo9NfUUHwLXLhFCsfz9kHUza
	return hex!["ee83256d5fd8a4fc41b418d2ae84be2448a7458ce4ad8e88a545e47715a9e000"].into()
}

const AUTHORITY_ENDOWMENT: Balance = 100_000_000 * SCV; //100_000_000 SCV

const ROOT_ENDOWMENT: Balance = 500_000_000 * SCV; //500_000_000 SCV

const TEAM_ENDOWMENT: Balance = 100_000_000 * SCV; //100_000_000 SCV

const CONTROLLER_ENDOWMENT: Balance = 50 * SCV;
const STASH_ENDOWMENT: Balance = 50 * SCV;

pub fn get_cv_initial_authorities(
) -> Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)> {
	return vec![
		(
			hex!["88e115f542393f09c5fccc2fbc9fc4f46afb6bc702ecdc2c97f09f659785d969"].into(),
			hex!["1cc27871f0752a36c440f54709d1d142ce0616e463abb029a34caa5838d2436b"].into(),
			hex!["a2245539d0fd497b73c2ca928cb963466118ccdfae7770b0cb01e3b162e3176c"]
				.unchecked_into(),
			hex!["a400aab0ddce43835fafbb38a9c9c53796b2af3658221f84f8d04ac2faf8c753"]
				.unchecked_into(),
			hex!["a2245539d0fd497b73c2ca928cb963466118ccdfae7770b0cb01e3b162e3176c"]
				.unchecked_into(),
			hex!["a2245539d0fd497b73c2ca928cb963466118ccdfae7770b0cb01e3b162e3176c"]
				.unchecked_into(),
		),
		(
			//STASH
			hex!["b231993d3e549f827c4b57e8b72c2f27c5362e4d3540ff3ab8e66eae92eee748"].into(),
			//CONTROLLER
			hex!["76cea3eccea733bf142118ef0e143ccac611b6f9b11092a93a4f46cb1027a516"].into(),
			hex!["18c31f136bd4e7d942de1a291cf11bab8a9d93fc38036a7951b49c2017166c39"]
				.unchecked_into(),
			hex!["c841ca88cf40f53ad0930d9a4a4c9d93c33c100a193f7f59b300174996c248f1"]
				.unchecked_into(),
			hex!["18c31f136bd4e7d942de1a291cf11bab8a9d93fc38036a7951b49c2017166c39"]
				.unchecked_into(),
			hex!["18c31f136bd4e7d942de1a291cf11bab8a9d93fc38036a7951b49c2017166c39"]
				.unchecked_into(),
		),
	]
}

pub fn get_endownment_account() -> Vec<(AccountId, Balance)> {
	return vec![
		(
			//STASH
			hex!["88e115f542393f09c5fccc2fbc9fc4f46afb6bc702ecdc2c97f09f659785d969"].into(),
			STASH_ENDOWMENT,
		),
		(
			//Controller
			hex!["1cc27871f0752a36c440f54709d1d142ce0616e463abb029a34caa5838d2436b"].into(),
			CONTROLLER_ENDOWMENT,
		),
		(
			//STASH
			hex!["b231993d3e549f827c4b57e8b72c2f27c5362e4d3540ff3ab8e66eae92eee748"].into(),
			STASH_ENDOWMENT,
		),
		(
			//Controller
			hex!["76cea3eccea733bf142118ef0e143ccac611b6f9b11092a93a4f46cb1027a516"].into(),
			CONTROLLER_ENDOWMENT,
		),
		(
			//team endownment
			hex!["a69d88301a87898468fce00f26b0f607d4d7c0ea8ee25b0a247e1b821974d84b"].into(),
			TEAM_ENDOWMENT,
		),
		(
			hex!["a2245539d0fd497b73c2ca928cb963466118ccdfae7770b0cb01e3b162e3176c"].into(),
			AUTHORITY_ENDOWMENT,
		),
		(
			hex!["18c31f136bd4e7d942de1a291cf11bab8a9d93fc38036a7951b49c2017166c39"].into(),
			AUTHORITY_ENDOWMENT,
		),
		(
			hex!["ee83256d5fd8a4fc41b418d2ae84be2448a7458ce4ad8e88a545e47715a9e000"].into(),
			ROOT_ENDOWMENT,
		),
	]
}
