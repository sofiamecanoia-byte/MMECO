#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

extern crate alloc;

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use alloc::vec::Vec;
use polkadot_sdk::sp_std::borrow::Cow;
use polkadot_sdk::sp_api::impl_runtime_apis;
use polkadot_sdk::sp_runtime::{
    generic,
    traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    MultiSignature,
};
use polkadot_sdk::sp_version::RuntimeVersion;
use polkadot_sdk::frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU16, ConstU32, ConstU64, Everything},
};

// CORREÇÃO CRÍTICA PARA E0599 (stable2412)
use polkadot_sdk::frame_executive::traits::ExecuteBlock;

pub use pallet_reputation;
pub use pallet_projects;
pub use pallet_governance;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Hash = polkadot_sdk::sp_core::H256;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Nonce = u32;

pub type TxExtension = (
    polkadot_sdk::frame_system::CheckNonZeroSender<Runtime>,
    polkadot_sdk::frame_system::CheckSpecVersion<Runtime>,
    polkadot_sdk::frame_system::CheckTxVersion<Runtime>,
    polkadot_sdk::frame_system::CheckGenesis<Runtime>,
    polkadot_sdk::frame_system::CheckEra<Runtime>,
    polkadot_sdk::frame_system::CheckNonce<Runtime>,
    polkadot_sdk::frame_system::CheckWeight<Runtime>,
);

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, TxExtension>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("moral-money"),
    impl_name: Cow::Borrowed("moral-money"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: polkadot_sdk::sp_version::create_apis_vec!([]),
    transaction_version: 1,
    system_version: 1,
};

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

construct_runtime!(
    pub struct Runtime {
        System: polkadot_sdk::frame_system = 0,
        Timestamp: polkadot_sdk::pallet_timestamp = 1,
        Balances: polkadot_sdk::pallet_balances = 2,
        Aura: polkadot_sdk::pallet_aura = 3,
        Grandpa: polkadot_sdk::pallet_grandpa = 4,
        Reputation: pallet_reputation = 5,
        Projects: pallet_projects = 6,
        Governance: pallet_governance = 7,
    }
);

impl polkadot_sdk::frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = RuntimeTask;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = polkadot_sdk::sp_runtime::traits::AccountIdLookup<AccountId, ()>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU32<256>;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = polkadot_sdk::pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

impl polkadot_sdk::pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<3000>;
    type WeightInfo = ();
}

impl polkadot_sdk::pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = polkadot_sdk::frame_system::Pallet<Runtime>;
    type WeightInfo = ();
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<50>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = (); 
}

impl polkadot_sdk::pallet_aura::Config for Runtime {
    type AuthorityId = polkadot_sdk::sp_consensus_aura::sr25519::AuthorityId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = polkadot_sdk::frame_support::traits::ConstBool<false>;
    type SlotDuration = polkadot_sdk::pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl polkadot_sdk::pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = polkadot_sdk::sp_core::Void;
    type EquivocationReportSystem = ();
}

impl pallet_reputation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_projects::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

type Executive = polkadot_sdk::frame_executive::Executive<
    Runtime,
    Block,
    polkadot_sdk::frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

impl_runtime_apis! {
    impl polkadot_sdk::sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> polkadot_sdk::sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl polkadot_sdk::sp_api::Metadata<Block> for Runtime {
        fn metadata() -> polkadot_sdk::sp_core::OpaqueMetadata {
            polkadot_sdk::sp_core::OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<polkadot_sdk::sp_core::OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl polkadot_sdk::sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> polkadot_sdk::sp_runtime::ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: polkadot_sdk::sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: polkadot_sdk::sp_inherents::InherentData,
        ) -> polkadot_sdk::sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl polkadot_sdk::sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: polkadot_sdk::sp_runtime::transaction_validity::TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> polkadot_sdk::sp_runtime::transaction_validity::TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl polkadot_sdk::sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl polkadot_sdk::sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
            Default::default()
        }

        fn decode_session_keys(
            _encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, polkadot_sdk::sp_core::crypto::KeyTypeId)>> {
            None
        }
    }

    impl polkadot_sdk::sp_consensus_aura::AuraApi<Block, polkadot_sdk::sp_consensus_aura::sr25519::AuthorityId> for Runtime {
        fn slot_duration() -> polkadot_sdk::sp_consensus_aura::SlotDuration {
            polkadot_sdk::sp_consensus_aura::SlotDuration::from_millis(6000)
        }

        fn authorities() -> Vec<polkadot_sdk::sp_consensus_aura::sr25519::AuthorityId> {
            polkadot_sdk::pallet_aura::Authorities::<Runtime>::get().into_inner()
        }
    }

    impl polkadot_sdk::sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> polkadot_sdk::sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> polkadot_sdk::sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: polkadot_sdk::sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                BlockNumber,
            >,
            _key_owner_proof: polkadot_sdk::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: polkadot_sdk::sp_consensus_grandpa::SetId,
            _authority_id: polkadot_sdk::sp_consensus_grandpa::AuthorityId,
        ) -> Option<polkadot_sdk::sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            None
        }
    }
}
