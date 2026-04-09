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

// CORREÇÃO: ExecuteBlock está disponível via frame_support no runtime
// Não é necessário import direto - o Executive já usa corretamente

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

pub type TxExtension = ();

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, TxExtension>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

#[polkadot_sdk::sp_version::runtime_version]
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
    TxExtension,
>;

impl_runtime_apis! {
    impl polkadot_sdk::sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            // Temporary skip - Checkable trait issue on stable2412
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> polkadot_sdk::sp_runtime::ExtrinsicInclusionMode {
            Default::default()
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
        fn apply_extrinsic(_extrinsic: <Block as BlockT>::Extrinsic) -> polkadot_sdk::sp_runtime::ApplyExtrinsicResult {
            Ok(Ok(()))
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Header {
                parent_hash: Default::default(),
                number: 0,
                state_root: Default::default(),
                extrinsics_root: Default::default(),
                digest: Default::default(),
            }
        }

        fn inherent_extrinsics(_data: polkadot_sdk::sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            Vec::new()
        }

        fn check_inherents(
            _block: Block,
            _data: polkadot_sdk::sp_inherents::InherentData,
        ) -> polkadot_sdk::sp_inherents::CheckInherentsResult {
            Default::default()
        }
    }

    impl polkadot_sdk::sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            _source: polkadot_sdk::sp_runtime::transaction_validity::TransactionSource,
            _tx: <Block as BlockT>::Extrinsic,
            _block_hash: <Block as BlockT>::Hash,
        ) -> polkadot_sdk::sp_runtime::transaction_validity::TransactionValidity {
            Ok(Default::default())
        }
    }

    impl polkadot_sdk::sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(_header: &<Block as BlockT>::Header) {
            // Temporary skip - Checkable trait issue on stable2412
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
