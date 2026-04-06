#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::BoundedVec;
    use frame_support::traits::ConstU32;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// O tipo de evento que a blockchain vai emitir.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ContributionRecord<AccountId, Balance> {
        pub account: AccountId,
        pub project_id: u32,
        pub amount: Balance,
    }

    #[pallet::storage]
    pub type Contributions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        BoundedVec<ContributionRecord<T::AccountId, u128>, ConstU32<100>>,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Evento disparado quando um novo projeto é criado.
        /// [id_do_criador, id_do_projeto]
        ProjectCreated { creator: T::AccountId, project_id: u32 },
    }

    // Estrutura base de erros para a palete de projetos
    #[pallet::error]
    pub enum Error<T> {
        /// Exemplo: O projeto já existe
        ProjectAlreadyExists,
        /// Exemplo: Limite de projetos atingido
        MaxProjectsReached,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn contribute(
            origin: OriginFor<T>,
            project_id: u32,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Contributions::<T>::mutate(project_id, |list| {
                let _ = list.try_push(ContributionRecord {
                    account: who.clone(),
                    project_id,
                    amount,
                });
            });
            Ok(())
        }
    }
}

