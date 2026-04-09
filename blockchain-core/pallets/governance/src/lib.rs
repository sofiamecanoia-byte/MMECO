#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[polkadot_sdk::frame_support::pallet]
pub mod pallet {
    use polkadot_sdk::frame_support::pallet_prelude::*;
    use polkadot_sdk::frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn proposal_owners)]
    pub type ProposalOwners<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T> = StorageMap<_, Blake2_128Concat, u32, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalCreated { who: T::AccountId, proposal_id: u32 },
        Voted { who: T::AccountId, proposal_id: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        StorageOverflow,
        ProposalNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::default())]
        pub fn create_proposal(origin: OriginFor<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let new_id = ProposalCount::<T>::get().checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
            
            ProposalCount::<T>::put(new_id);
            ProposalOwners::<T>::insert(new_id, sender.clone());
            
            Self::deposit_event(Event::ProposalCreated { who: sender, proposal_id: new_id });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::default())]
        pub fn vote(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(proposal_id <= ProposalCount::<T>::get(), Error::<T>::ProposalNotFound);

            let current_votes = Votes::<T>::get(proposal_id);
            let new_votes = current_votes.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
            Votes::<T>::insert(proposal_id, new_votes);

            Self::deposit_event(Event::Voted { who: sender, proposal_id });
            Ok(())
        }
    }
}
