use crate::testnet_fixtures;
use node_primitives::Balance;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use scv_node::{
	currency::SCV,
	opaque::{Block, SessionKeys},
	AccountId, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, GenesisConfig,
	HierarchyConfig, IndicesConfig, SessionConfig, Signature, StakerStatus, StakingConfig,
	SudoConfig, SystemConfig, WASM_BINARY,
};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use hierarchy_system::UserId;
const DEFAULT_PROTOCOL_ID: &str = "scv";

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

pub(crate) fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				// vec![(
				// 	get_account_id_from_seed::<sr25519::Public>("Alice"),
				// 	root_authority.clone(),
				// )],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
		Default::default(),
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				// vec![(
				// 	get_account_id_from_seed::<sr25519::Public>("Alice"),
				// 	root_authority.clone(),
				// )],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
		Default::default(),
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	// sys_man: Vec<(AccountId, SysManAccount<scv_node::Runtime>)>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: u128 = 100 * SCV;
	const STASH: u128 = 2 * SCV;
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)) // stash, controller, balance, status
				.collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(scv_node::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: Default::default(),
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key.clone()),
		},
		transaction_payment: Default::default(),
		treasury: Default::default(),
		hierarchy: HierarchyConfig {
			root_hierarchy: Some(UserId::from([0u8;36])),
			// sys_mans: vec![
			// 	(UserId::from([1u8;36]),get_account_id_from_seed::<sr25519::Public>("Bob")),
			// 	(UserId::from([2u8;36]),get_account_id_from_seed::<sr25519::Public>("Charlie")),
			// ],
		},
	}
}

// Configure initial storage state for FRAME modules.
fn cv_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	//sys_man: Vec<(AccountId, SysManAccount<scv_node::Runtime>)>,
	_enable_println: bool,
) -> GenesisConfig {
	const STASH: u128 = 10 * SCV;

	let allocation_account: Vec<(AccountId, Balance)> =
		endowed_accounts.iter().map(|x| (x.0.clone(), x.1.clone())).collect();
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: allocation_account,
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)) // stash, controller, balance, status
				.collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(scv_node::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: Default::default(),
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		treasury: Default::default(),
		hierarchy: HierarchyConfig {
			root_hierarchy: Some(UserId::from([0u8;36])),
			// sys_mans: vec![
			// 	(UserId::from([1u8;36]),get_account_id_from_seed::<sr25519::Public>("Bob")),
			// 	(UserId::from([2u8;36]),get_account_id_from_seed::<sr25519::Public>("Charlie")),
			// ],
		},
	}
}

pub fn cv_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"SCV Testnet",
		"SCV_testnet",
		ChainType::Live,
		move || {
			cv_testnet_genesis(
				wasm_binary,
				testnet_fixtures::get_cv_initial_authorities(),
				/* Sudo Account */
				testnet_fixtures::get_testnet_root_key(),
				testnet_fixtures::get_endownment_account(),
				true,
			)
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(cv_properties()),
		Default::default(),
	))
}

pub fn cv_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "SCV".into());

	properties
}
