#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_karaoke::KaraokeError;
    use sp_karaoke::INHERENT_IDENTIFIER;
    use sp_karaoke::InherentType;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn line)]
    pub type Line<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO: set weight that makes sense
        #[pallet::weight(10_000)]
        pub fn set(origin: OriginFor<T>, song_line : u32) -> DispatchResult {
            ensure_none(origin)?;
            Line::<T>::put(song_line);
            Ok(())
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = KaraokeError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data = data.get_data::<InherentType>(&INHERENT_IDENTIFIER)
                .expect("Song line not correctly encoded")
                .expect("Song line data must be provided");

            Some(Call::set { song_line : data.0 })
        }

        fn is_inherent(call : &Self::Call) -> bool {
            matches!(call, Call::set{ .. })
        }
    }
}
