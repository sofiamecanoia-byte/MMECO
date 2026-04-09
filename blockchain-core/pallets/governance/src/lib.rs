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
    pub type ProposalOwners<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId, OptionQuery>;

    #[pallet::storage]
    pub type Votes<T> = StorageMap<_, Blake2_128Concat, u32, u32, ValueQuery>;

    #[pallet::storage]
    pub type UserVoted<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, u32,
        Blake2_128Concat, T::AccountId,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalCreated { who: T::AccountId, proposal_id: u32 },
        Voted { voter: T::AccountId, proposal_id: u32 },
    }

    // [PONTO 3 DO CLAUDE - Erros]
    #[pallet::error]
    pub enum Error<T> {
        /// O utilizador já votou nesta proposta.
        AlreadyVoted,
        /// A proposta especificada não existe.
        ProposalNotFound,
        /// Erro de transbordamento de armazenamento.
        StorageOverflow,
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
            let voter = ensure_signed(origin)?;
            
            ensure!(proposal_id <= ProposalCount::<T>::get() && proposal_id > 0, Error::<T>::ProposalNotFound);
            ensure!(!UserVoted::<T>::get(proposal_id, &voter), Error::<T>::AlreadyVoted);
            
            UserVoted::<T>::insert(proposal_id, &voter, true);
            Votes::<T>::mutate(proposal_id, |count| {
                *count = count.saturating_add(1);
            });
            
            Self::deposit_event(Event::Voted { voter, proposal_id });
            Ok(())
        }
    }
}
