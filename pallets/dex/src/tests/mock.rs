use crate as pallet_dex;
use crate::AssetBalanceOf;
use codec::Compact;
use frame_support::assert_ok;
use frame_support::pallet_prelude::*;
use frame_support::traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, ConstU64};
use frame_support::{parameter_types, PalletId};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::{sp_std, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, FixedU128,
};
use sp_std::prelude::*;
use std::cell::RefCell;

type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;
pub type AssetId = u32;
pub type AccountId = u64;
pub const MIN_LIQUIDITY: u128 = 1000;
pub type NativeBalance = <Test as crate::Config>::NativeBalance;
pub type Fungibles = <Test as crate::Config>::Fungibles;

pub const DOT: AssetId = 100;
pub const USDC: AssetId = 101;
pub const ADMIN: AccountId = 1;
pub const ALICE: AccountId = 2;
pub const BOB: AccountId = 3;
pub const MINT_BALANCE: u128 = 1;

parameter_types! {
	pub const MemeSwapPallet: PalletId = PalletId(*b"MeMeSwap");
	pub const TokenDecimals: u32 = 10;
	pub const MinimumLiquidity: u32 = 1000;
}

thread_local! {
	pub static ENDOWED_BALANCES: RefCell<Vec<(AssetId, AccountId, Balance)>> = RefCell::new(Vec::new());
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Dex: pallet_dex,
	}
);

impl frame_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type MaxHolds = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<100>;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = ConstU128<10>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_dex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = Balances;
	type Fungibles = Assets;
	type PalletId = MemeSwapPallet;
	type TokenDecimals = TokenDecimals;
	type MinimumLiquidity = MinimumLiquidity;
}

pub struct ExtBuilder {
	endowed_balances: Vec<(AssetId, AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		ENDOWED_BALANCES.with(|v| {
			v.borrow_mut().clear();
		});
		Self { endowed_balances: vec![] }
	}
}

impl ExtBuilder {
	pub fn with_endowed_balances(mut self, balances: Vec<(AssetId, AccountId, Balance)>) -> Self {
		self.endowed_balances = balances;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		let mut assets = vec![];
		for (asset_id, account_id, balance) in self.endowed_balances.clone().into_iter() {
			assets.push((asset_id, ADMIN, true, MINT_BALANCE));
		}

		pallet_assets::GenesisConfig::<Test> {
			assets,
			metadata: vec![],
			accounts: self.endowed_balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

pub(crate) fn create_and_mint(asset: u32, user: u64, amount: u128) -> Result<(), &'static str> {
	assert_ok!(Assets::force_create(
		RuntimeOrigin::root(),
		Compact::from(asset),
		ADMIN,
		true,
		MINT_BALANCE
	));
	assert_ok!(Assets::mint(RuntimeOrigin::signed(ADMIN), Compact::from(asset), user, amount));
	Ok(())
}

pub(super) fn expand_to_decimals(n: u128) -> u128 {
	n * 10u128.pow(10u32)
}

pub(super) fn decimals_to_numeric(n: u128) -> u128 {
	FixedU128::from_inner(n).div(10u128.pow(10u32).into()).into_inner()
}
