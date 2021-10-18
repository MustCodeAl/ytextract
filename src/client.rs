use crate::{
    channel, playlist, stream, video, youtube::innertube::Api, Channel, Playlist, Stream, Video,
};

/// A Client capable of interacting with YouTube
///
/// Note: This structure already uses an [`Arc`](std::sync::Arc) internally, so
///       it does not need to be wrapped again.
#[allow(missing_debug_implementations)]
#[derive(Clone, Default)]
pub struct Client {
    pub(crate) api: Api,
}

impl Client {
    /// Create a new [`Client`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a [`Video`] identified by a [`Id`](video::Id)
    pub async fn video(&self, id: video::Id) -> crate::Result<Video> {
        Video::get(self.clone(), id).await
    }

    /// Get the [`Stream`]s of a [`Video`] identified by a [`Id`](video::Id)
    pub async fn streams(&self, id: video::Id) -> crate::Result<impl Iterator<Item = Stream>> {
        stream::get(self.clone(), id).await
    }

    /// Get a [`Playlist`] identified by a [`Id`](playlist::Id)
    pub async fn playlist(&self, id: playlist::Id) -> crate::Result<Playlist> {
        Playlist::get(self.clone(), id).await
    }

    /// Get a [`Channel`] identified by a [`Id`](channel::Id)
    pub async fn channel(&self, id: channel::Id) -> crate::Result<Channel> {
        Channel::get(self.clone(), id).await
    }
}
