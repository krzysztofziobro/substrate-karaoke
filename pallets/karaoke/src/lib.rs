#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

extern crate alloc;
pub use pallet::*;

use codec::{Decode, Encode};
use sp_inherents::{InherentData, InherentIdentifier,   IsFatalError};
use alloc::vec::*;

pub const INHERENT_IDENTIFIER : InherentIdentifier = *b"karaoke0";
pub type InherentType = Vec<u8>;

pub const SONG_LEN : u32 = 19;
//const SONG : [&'static str; SONG_LEN as usize] = ["a", "b", "c", "d"];
const SONG : [&'static str; SONG_LEN as usize] =
["If you like to gamble, I tell you I'm your man",
"You win some, lose some, all the same to me",
"The pleasure is to play, makes no difference what you say",
"I don't share your greed, the only card I need is the Ace of Spades",
"The Ace of Spades",
"Playing for the high one, dancing with the devil",
"Going with the flow, it's all a game to me",
"Seven or eleven, snake eyes watching you",
"Double up or quit, double stake or split, the Ace of Spades",
"The Ace of Spades",
"You know I'm born to lose, and gambling's for fools",
"But that's the way I like it baby",
"I don't wanna live for ever",
"And don't forget the joker!",
"Pushing up the ante, I know you gotta see me",
"Read 'em and weep, the dead man's hand again",
"I see it in your eyes, take one look and die",
"The only thing you see, you know it's gonna be the Ace of Spades",
"The Ace of Spades"];

#[derive(Encode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode, thiserror::Error))]
#[cfg_attr(feature = "std", error("Karaoke Error"))]
pub enum KaraokeError {
    KaraokeError,
}

impl IsFatalError for KaraokeError {
    fn is_fatal_error(&self) -> bool {
        return false;
    }
}

#[cfg(feature = "std")]
pub struct InherentDataProvider {
    song_lines : Vec<InherentType>,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    pub fn new() -> Self {
        Self { song_lines : SONG.iter().map(|&s| s.as_bytes().to_vec()).collect() }
    }
}


#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(&self, inherent_data: &mut InherentData)
        -> Result<(), sp_inherents::Error> {
            inherent_data.put_data(INHERENT_IDENTIFIER, &self.song_lines)
        }

    async fn try_handle_error(
        &self,
        _identifier : & InherentIdentifier,
        _error : &[u8],
        ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}


#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Event that should be emitted when song line extrinsic is added
        SongLineSet(InherentType),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn line)]
    // Line counter that provides context for choosing a song line
    pub type Line<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((T::DbWeight::get().reads_writes(1, 1) + 10_000, DispatchClass::Mandatory))]
        pub fn update_karaoke_inherent(origin: OriginFor<T>, song_line : InherentType) -> DispatchResult {
            // Check unsigned
            ensure_none(origin)?;

            // update line counter
            let mut ind = Line::<T>::get();
            ind = (ind + 1) % SONG_LEN;
            Line::<T>::put(ind);

            // emit an event
            Self::deposit_event(Event::SongLineSet(song_line));
            Ok(())
        }
    }

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
        type Error = KaraokeError;
        const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

        fn create_inherent(data: &InherentData) -> Option<Self::Call> {
            // extract data created by InherentDataProvider
            let data = data.get_data::<Vec<InherentType>>(&INHERENT_IDENTIFIER)
                .expect("Song not correctly encoded")
                .expect("Song data must be provided");

            // get value of line counter from storage
            let line_num = Line::<T>::get() as usize;

            // create inherent
            Some(Call::update_karaoke_inherent { song_line : data[line_num].clone() })
        }

        fn is_inherent(call : &Self::Call) -> bool {
            matches!(call, Call::update_karaoke_inherent{ .. })
        }
    }
}
