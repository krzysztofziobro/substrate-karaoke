#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_inherents::{InherentData, InherentIdentifier,   IsFatalError};

pub const INHERENT_IDENTIFIER : InherentIdentifier = *b"karaoke0";
pub type InherentType = SongLine;

#[derive(Encode, Decode)]
pub struct SongLine(pub u32);

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
    song_line : InherentType,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    pub fn new(song_line : InherentType) -> Self {
        Self { song_line }
    }
}


#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(&self, inherent_data: &mut InherentData)
        -> Result<(), sp_inherents::Error> {
            inherent_data.put_data(INHERENT_IDENTIFIER, &self.song_line)
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

