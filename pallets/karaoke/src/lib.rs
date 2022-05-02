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
    use sp_karaoke::SONG_LEN;

    extern crate alloc;
    use alloc::vec::*;

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
        #[pallet::weight((10_000, DispatchClass::Mandatory))]
        pub fn update_karaoke_inherent(origin: OriginFor<T>, _song_line : [u8; 100]) -> DispatchResult {
            ensure_none(origin)?;
            let mut ind = Line::<T>::get();
            ind = (ind + 1) % SONG_LEN;
            Line::<T>::put(ind);
            Ok(())
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = KaraokeError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            let data = data.get_data::<Vec<InherentType>>(&INHERENT_IDENTIFIER)
                .expect("Song line not correctly encoded")
                .expect("Song line data must be provided");

            let line_num = Line::<T>::get() as usize;

            let mut line_data : [u8; 100] = [0; 100];
            for i in 0..data[line_num].len() {
                line_data[i] = data[line_num][i];
            }

            Some(Call::update_karaoke_inherent { song_line : line_data })
        }

        fn is_inherent(call : &Self::Call) -> bool {
            matches!(call, Call::update_karaoke_inherent{ .. })
        }
    }
}
