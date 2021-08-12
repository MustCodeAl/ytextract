//! Videos.
//!
//! Videos are identified by a unique [`Id`], and can be queried with a
//! [`Client`](crate::Client).
//!
//! Once you have a [`Video`] you can use [`Video::streams`] to get its
//! [`Streams`](crate::Stream). These contain URLs to download videos,
//! along with metadata about dimensions and fps.
//!
//! # Example
//!
//! ```rust
//! # #[async_std::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ytextract::Client::new().await?;
//!
//! let video = client.video("nI2e-J6fsuk".parse()?).await?;
//!
//! println!("Title: {}", video.title());
//! # Ok(())
//! # }
//! ```

use crate::{youtube::player_response::PlayerResponse, Client, Stream, Thumbnail};

use serde_json::Value;

use std::{sync::Arc, time::Duration};

/// A Video.
///
/// For more information see the [crate level documentation](crate::video)
#[derive(Clone)]
pub struct Video {
    initial_data: Value,
    player_response: PlayerResponse,
    client: Arc<Client>,
}

impl Video {
    pub(crate) async fn get(client: Arc<Client>, id: Id) -> crate::Result<Self> {
        let player_response = client.api.player(id).await?.into_std()?;

        Ok(Self {
            initial_data: client.api.next(id).await?,
            client,
            player_response,
        })
    }

    /// The title of a [`Video`].
    pub fn title(&self) -> &str {
        &self.player_response.video_details.title
    }

    /// The [`Id`] of a [`Video`].
    pub fn id(&self) -> Id {
        self.player_response.video_details.video_id
    }

    /// The [`Duration`] of a [`Video`].
    pub fn duration(&self) -> Duration {
        self.player_response.video_details.length_seconds
    }

    /// The keyword/tags of a [`Video`].
    pub fn keywords(&self) -> &Vec<String> {
        &self.player_response.video_details.keywords
    }

    /// The [`Channel`] of a [`Video`].
    pub fn channel(&self) -> Channel<'_> {
        Channel {
            client: Arc::clone(&self.client),
            id: self.player_response.video_details.channel_id,
            name: &self.player_response.video_details.author,
        }
    }

    /// The description of a [`Video`].
    pub fn description(&self) -> &str {
        &self.player_response.video_details.short_description
    }

    /// The amount of views a [`Video`] received.
    pub fn views(&self) -> u64 {
        self.player_response.video_details.view_count
    }

    /// The [`Ratings`] a [`Video`] received.
    pub fn ratings(&self) -> Ratings {
        if self.player_response.video_details.allow_ratings {
            let fixed_tooltip = self.initial_data["contents"]["twoColumnWatchNextResults"]
                ["results"]["results"]["contents"]
                .as_array()
                .expect("InitialData contents was not an array")
                .iter()
                .find_map(|v| v.get("videoPrimaryInfoRenderer"))
                .expect("InitialData contents did not have a videoPrimaryInfoRenderer")
                ["sentimentBar"]["sentimentBarRenderer"]["tooltip"]
                .as_str()
                .expect("sentimentBar tooltip was not a string")
                .replace(',', "");
            let (likes, dislikes) = fixed_tooltip
                .split_once(" / ")
                .expect("sentimentBar tooltip did not have a '/'");

            let likes = likes
                .parse()
                .expect("Likes we not parsable as a unsigned integer");
            let dislikes = dislikes
                .parse()
                .expect("Dislikes we not parsable as a unsigned integer");

            Ratings::Allowed { likes, dislikes }
        } else {
            Ratings::NotAllowed
        }
    }

    /// If a [`Video`] is live (e.g. a Livestream) or if it was live in the
    /// past.
    pub fn live(&self) -> bool {
        self.player_response.video_details.is_live_content
    }

    /// The [`Thumbnails`](Thumbnail) of a [`Video`]
    pub fn thumbnails(&self) -> &Vec<Thumbnail> {
        &self.player_response.video_details.thumbnail.thumbnails
    }

    /// If a [`Video`] is age-restricted.
    pub fn age_restricted(&self) -> bool {
        !self.microformat().is_family_safe
    }

    fn microformat(&self) -> &crate::youtube::player_response::PlayerMicroformatRenderer {
        &self.player_response.microformat.player_microformat_renderer
    }

    /// If a [`Video`] is unlisted.
    pub fn unlisted(&self) -> bool {
        self.microformat().is_unlisted
    }

    /// The category a [`Video`] belongs in.
    pub fn category(&self) -> &str {
        &self.microformat().category
    }

    /// The date a [`Video`] was published.
    pub fn publish_date(&self) -> chrono::NaiveDate {
        self.microformat().publish_date
    }

    /// The date a [`Video`] was uploaded.
    pub fn upload_date(&self) -> chrono::NaiveDate {
        self.microformat().upload_date
    }

    /// The [`Streams`](Stream) of a [`Video`]
    pub async fn streams(&self) -> crate::Result<impl Iterator<Item = Stream>> {
        crate::stream::get(
            Arc::clone(&self.client),
            self.id(),
            self.player_response.streaming_data.clone(),
        )
        .await
    }
}

impl std::fmt::Debug for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Video")
            .field("id", &self.id())
            .field("title", &self.title())
            .field("duration", &self.duration())
            .field("keywords", &self.keywords())
            .field("channel", &self.channel())
            .field("description", &self.description())
            .field("views", &self.views())
            .field("ratings", &self.ratings())
            .field("live", &self.live())
            .field("thumbnails", &self.thumbnails())
            .field("age_restricted", &self.age_restricted())
            .field("unlisted", &self.unlisted())
            .field("category", &self.category())
            .field("publish_date", &self.publish_date())
            .field("upload_date", &self.upload_date())
            .finish()
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Video {}

/// The uploader of a video
pub struct Channel<'a> {
    client: Arc<Client>,
    id: crate::channel::Id,
    name: &'a str,
}

impl<'a> Channel<'a> {
    /// The [`Id`](crate::channel::Id) of a [`Channel`]
    pub fn id(&self) -> crate::channel::Id {
        self.id
    }

    /// The name of a [`Channel`]
    pub fn name(&self) -> &str {
        self.name
    }

    /// Refetch the channel to get more information
    pub async fn upgrade(&self) -> crate::Result<crate::Channel> {
        self.client.channel(self.id).await
    }
}

impl<'a> std::fmt::Debug for Channel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

impl<'a> PartialEq for Channel<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'a> Eq for Channel<'a> {}

/// Ratings on a video
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ratings {
    /// Rating is allowed
    Allowed {
        /// The amount of likes a [`Video`] received
        likes: u64,
        /// The amount of dislikes a [`Video`] received
        dislikes: u64,
    },

    /// Rating is not allowed
    NotAllowed,
}

define_id! {
    11,
    "An Id describing a [`Video`]",
    [
        "https://www.youtube.com/watch?v=",
        "https://youtu.be/",
        "https://www.youtube.com/embed/",
    ]
}
