//! Recommended/Related items of a video.

use crate::youtube::{
    self,
    next::{
        CompactMovieRenderer, CompactPlaylistRenderer, CompactRadioRenderer, CompactVideoRenderer,
    },
    parse_length,
};

use std::fmt::Debug;

/// A related Video
#[derive(Clone)]
pub struct Video(pub(super) CompactVideoRenderer, pub(super) crate::Client);

impl Video {
    /// The [`Id`](super::Id) of this video.
    pub fn id(&self) -> super::Id {
        self.0.video_id
    }

    /// The title of this video.
    pub fn title(&self) -> &str {
        &self.0.title
    }

    /// The [`Thumbnails`](crate::Thumbnail) of this video.
    pub fn thumbnails(&self) -> impl Iterator<Item = &crate::Thumbnail> {
        self.0.thumbnail.thumbnails.iter()
    }

    /// The amount of views this video has.
    pub fn views(&self) -> Option<u64> {
        let s: &str = match self.0.view_count_text.as_ref()? {
            // "<VIEWS> views"
            crate::youtube::Text::SimpleText(simple) => {
                simple
                    .simple_text
                    .split_once(' ')
                    .expect("No space in view_count_text")
                    .0
            }
            // ["<VIEWS>", ..]
            crate::youtube::Text::Runs(runs) => &runs.runs[0].text,
        };

        Some(s.replace(',', "").parse().expect("Views were not parsable"))
    }

    /// The length of this video. [`None`] if this video is a livestream.
    pub fn length(&self) -> Option<std::time::Duration> {
        self.0.length_text.as_deref().map(parse_length)
    }

    /// The [`Channel`] that uploaded this video.
    pub fn channel(&self) -> Channel<'_> {
        Channel {
            id: Some(
                self.0.short_byline_text.runs[0]
                    .navigation_endpoint
                    .browse_endpoint
                    .browse_id,
            ),
            name: &self.0.short_byline_text.runs[0].text,
            badges: &self.0.owner_badges,
            client: &self.1,
        }
    }

    /// Refetch this video for more information.
    pub async fn upgrade(&self) -> crate::Result<crate::Video> {
        self.1.video(self.id()).await
    }

    /// Get the [`Streams`](crate::Stream) for this video.
    pub async fn streams(&self) -> crate::Result<impl Iterator<Item = crate::Stream>> {
        self.1.streams(self.id()).await
    }
}

impl std::fmt::Debug for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Video")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("thumbnails", &self.thumbnails().collect::<Vec<_>>())
            .field("views", &self.views())
            .field("length", &self.length())
            .field("channel", &self.channel())
            .finish()
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Video {}

/// A related Playlist
#[derive(Clone)]
pub struct Playlist(pub(super) CompactPlaylistRenderer, pub(super) crate::Client);

impl Playlist {
    /// The [`Id`](crate::playlist::Id) of this playlist.
    pub fn id(&self) -> crate::playlist::Id {
        crate::playlist::Id(self.0.playlist_id.clone())
    }

    /// The title of this playlist.
    pub fn title(&self) -> &str {
        &self.0.title
    }

    /// The [`Thumbnails`](crate::Thumbnail) of this playlist.
    pub fn thumbnails(&self) -> impl Iterator<Item = &crate::Thumbnail> {
        self.0.thumbnail.thumbnails.iter()
    }

    /// The [`Channel`] that uploaded this playlist.
    pub fn channel(&self) -> Channel<'_> {
        Channel {
            id: self.0.short_byline_text.runs[0]
                .navigation_endpoint
                .clone()
                .map(|x| x.browse_endpoint.browse_id),
            name: &self.0.short_byline_text.runs[0].text,
            badges: &self.0.owner_badges,
            client: &self.1,
        }
    }

    /// Refetch this playlist for more information.
    pub async fn upgrade(&self) -> crate::Result<crate::Playlist> {
        self.1.playlist(self.id()).await
    }
}

impl Debug for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Playlist")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("thumbnails", &self.thumbnails().collect::<Vec<_>>())
            .field("channel", &self.channel())
            .finish()
    }
}

impl PartialEq for Playlist {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Playlist {}

/// A related Radio
#[derive(Clone)]
pub struct Radio(pub(super) CompactRadioRenderer, pub(super) crate::Client);

impl Radio {
    /// The [`Id`](crate::playlist::Id) of this radio.
    pub fn id(&self) -> crate::playlist::Id {
        crate::playlist::Id(self.0.playlist_id.clone())
    }

    /// The title of this radio.
    pub fn title(&self) -> &str {
        &self.0.title
    }

    /// The [`Thumbnails`](crate::Thumbnail) of this playlist.
    pub fn thumbnails(&self) -> impl Iterator<Item = &crate::Thumbnail> {
        self.0.thumbnail.thumbnails.iter()
    }

    /// Refetch this radio for more information.
    pub async fn upgrade(&self) -> crate::Result<crate::Playlist> {
        self.1.playlist(self.id()).await
    }
}

impl Debug for Radio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Radio")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("thumbnails", &self.thumbnails().collect::<Vec<_>>())
            .finish()
    }
}

impl PartialEq for Radio {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Radio {}

/// A related Movie
#[derive(Clone)]
pub struct Movie(pub(super) CompactMovieRenderer, pub(super) crate::Client);

impl Movie {
    /// The [`Id`](super::Id) of this movie.
    pub fn id(&self) -> super::Id {
        self.0.video_id
    }

    /// The title of this movie.
    pub fn title(&self) -> &str {
        &self.0.title
    }

    /// The [`Thumbnails`](crate::Thumbnail) of this movie.
    pub fn thumbnails(&self) -> impl Iterator<Item = &crate::Thumbnail> {
        self.0.thumbnail.thumbnails.iter()
    }

    /// The length of this movie.
    pub fn length(&self) -> std::time::Duration {
        parse_length(&self.0.length_text)
    }

    /// Refetch this video for more information.
    pub async fn upgrade(&self) -> crate::Result<crate::Video> {
        self.1.video(self.id()).await
    }
}

impl std::fmt::Debug for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Movie")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("thumbnails", &self.thumbnails().collect::<Vec<_>>())
            .field("length", &self.length())
            .finish()
    }
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Movie {}

/// The uploader of a [`Related`](super::Related) item
#[derive(Clone)]
pub struct Channel<'a> {
    id: Option<crate::channel::Id>,
    name: &'a str,
    badges: &'a Vec<youtube::Badge>,
    client: &'a crate::Client,
}

impl<'a> Channel<'a> {
    /// The [`Id`](crate::channel::Id) of this channel.
    pub fn id(&self) -> Option<crate::channel::Id> {
        self.id
    }

    /// The name of this channel.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The [`Badges`](crate::channel::Badge) that this channel has.
    pub fn badges(&self) -> impl Iterator<Item = crate::channel::Badge> + '_ {
        self.badges.iter().map(crate::channel::Badge::from)
    }
}

impl<'a> std::fmt::Debug for Channel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("badges", &self.badges().collect::<Vec<_>>())
            .finish()
    }
}

impl<'a> PartialEq for Channel<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'a> Eq for Channel<'a> {}
