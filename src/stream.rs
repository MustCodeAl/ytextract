//! Streams of a YouTube video
//!
//! # Example
//!
//! ```rust
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ytextract::Client::new();
//!
//! let streams = client.streams("nI2e-J6fsuk".parse()?).await?;
//!
//! for stream in streams {
//!     println!("Duration: {:?}", stream.duration())
//! }
//!
//! # Ok(())
//! # }
//! ```

mod audio;
mod common;
mod video;

pub use self::audio::Stream as Audio;
pub use self::common::Stream as Common;
pub use self::video::Stream as Video;
use crate::{youtube::player_response::FormatType, Client};

pub(crate) async fn get(
    client: Client,
    id: crate::video::Id,
) -> crate::Result<impl Iterator<Item = Stream>> {
    let player_response = client.api.streams(id).await?.into_std()?;

    // TODO: DashManifest/HlsManifest
    Ok(player_response
        .streaming_data
        .adaptive_formats
        .into_iter()
        .map(move |stream| Stream::new(stream, client.clone())))
}

/// A Stream of a YouTube video
#[derive(Clone)]
pub enum Stream {
    /// A Stream exclusively containing [`Audio`] data
    Audio(Audio),
    /// A Stream exclusively containing [`Video`] data
    Video(Video),
}

impl Stream {
    pub(crate) fn new(format: crate::youtube::player_response::Format, client: Client) -> Self {
        match format.ty {
            FormatType::Audio(audio) => Self::Audio(Audio {
                common: Common {
                    format: format.base,
                    client,
                },
                audio,
            }),
            FormatType::Video(video) => Self::Video(Video {
                common: Common {
                    format: format.base,
                    client,
                },
                video,
            }),
        }
    }

    /// Returns `true` if the stream is [`Self::Audio`].
    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio(..))
    }

    /// Returns `true` if the stream is [`Self::Video`].
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video(..))
    }
}

impl std::ops::Deref for Stream {
    type Target = Common;

    fn deref(&self) -> &Self::Target {
        match self {
            Stream::Audio(audio) => &audio.common,
            Stream::Video(video) => &video.common,
        }
    }
}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Stream");

        match self {
            Stream::Audio(audio) => {
                audio.common.debug(&mut debug);
                audio.debug(&mut debug);
            }
            Stream::Video(video) => {
                video.common.debug(&mut debug);
                video.debug(&mut debug);
            }
        }
        debug.finish()?;

        Ok(())
    }
}
