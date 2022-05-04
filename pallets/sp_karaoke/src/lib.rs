#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::{Decode, Encode};
use sp_inherents::{InherentData, InherentIdentifier,   IsFatalError};
use alloc::vec::*;

pub const INHERENT_IDENTIFIER : InherentIdentifier = *b"karaoke0";
pub type InherentType = Vec<u8>;

pub const SONG_LEN : u32 = 4;
//#[derive(Encode, Decode)]
//pub struct SongLine(pub String);

#[derive(Encode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode, thiserror::Error))]
#[cfg_attr(feature = "std", error("Karaoke Error"))]
pub struct KaraokeError {
    err_type : u8,
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
        Self { song_lines : vec!["If you like to gamble, I tell you I'm your man".as_bytes().to_vec(),
                            "You win some, lose some, all the same to me".as_bytes().to_vec(),
                            "The pleasure is to play, makes no difference what you say".as_bytes().to_vec(),
                            "I don't share your greed, the only card I need is the Ace of Spades".as_bytes().to_vec()] }
    }
}


#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(&self, inherent_data: &mut InherentData)
        -> Result<(), sp_inherents::Error> {
            inherent_data.put_data(INHERENT_IDENTIFIER, &self.song_lines)
        }

    // TODO: Real implementation
    async fn try_handle_error(
        &self,
        _identifier : & InherentIdentifier,
        _error : &[u8],
        ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}

