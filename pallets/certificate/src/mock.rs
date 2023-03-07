use crate as pallet_certificate;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, GenesisBuild},
};
use frame_system as system;
pub use pallet_utils::{CerStatus, Role};
use sp_core::H256;
use sp_runtime::{
	offchain::Timestamp,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Certificate: pallet_certificate::{Pallet, Call, Storage, Event<T>},
		Account: pallet_account::{Pallet, Call, Storage, Event<T>},
		Hierarchy: hierarchy_system::{Pallet, Call,Config<T>, Storage, Event<T>},

	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl pallet_certificate::Config for Test {
	type Event = Event;
	type CheckRole = Account;
	type UnixTime = Timestamp;
}
impl pallet_account::Config for Test {
	type Event = Event;
}

impl hierarchy_system::Config for Test {
	type Event = Event;
	type CheckRole = Account;
}

pub const ROOT: u64 = 0;
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	hierarchy_system::GenesisConfig::<Test> {
		root_hierarchy: Some(ROOT),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
