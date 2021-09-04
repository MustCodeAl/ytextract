use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum StreamResult {
    Ok {
        #[serde(rename = "streamingData")]
        streaming_data: StreamingData,
    },
    Error {
        #[serde(rename = "playabilityStatus")]
        playability_status: PlayabilityStatus,
    },
}

impl StreamResult {
    pub fn into_std(self) -> crate::Result<StreamingData> {
        match self {
            Self::Error { playability_status } => {
                Err(crate::Error::Youtube(playability_status.as_error()))
            }
            Self::Ok { streaming_data } => Ok(streaming_data),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Result {
    Ok(PlayerResponse),
    Error {
        #[serde(rename = "playabilityStatus")]
        playability_status: PlayabilityStatus,
    },
}

impl Result {
    pub fn into_std(self) -> crate::Result<PlayerResponse> {
        match self {
            Self::Error { playability_status } => {
                Err(crate::Error::Youtube(playability_status.as_error()))
            }
            Self::Ok(ok) => Ok(ok),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub video_details: VideoDetails,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub title: String,
    pub video_id: crate::video::Id,
    #[serde_as(as = "serde_with::DurationSeconds<String>")]
    pub length_seconds: Duration,

    #[serde(default)]
    pub keywords: Vec<String>,
    pub channel_id: crate::channel::Id,

    pub author: String,
    pub short_description: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub view_count: u64,

    pub allow_ratings: bool,

    pub is_private: bool,
    pub is_live_content: bool,

    pub thumbnail: Thumbnail,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub thumbnails: Vec<crate::Thumbnail>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde(default)]
    pub adaptive_formats: Vec<Format>,
}

#[derive(Deserialize, Clone)]
pub struct Format {
    #[serde(flatten)]
    pub base: CommonFormat,
    #[serde(flatten)]
    pub ty: FormatType,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonFormat {
    pub url: Url,
    pub quality: Quality,
    pub projection_type: String,
    pub mime_type: String,
    #[serde_as(as = "serde_with::TimestampMilliSeconds<String>")]
    pub last_modified: DateTime<Utc>,
    pub itag: u64,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(default)]
    pub content_length: Option<u64>,
    pub bitrate: u64,
    pub average_bitrate: Option<u64>,
    #[serde_as(as = "Option<serde_with::DurationMilliSeconds<String>>")]
    #[serde(default, rename = "approxDurationMs")]
    pub duration: Option<Duration>,

    pub init_range: Option<Range>,
    pub index_range: Option<Range>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum FormatType {
    Audio(AudioFormat),
    Video(VideoFormat),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoFormat {
    pub width: u64,
    pub height: u64,
    pub fps: u64,
    pub quality_label: String,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioFormat {
    pub loudness_db: Option<f64>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub audio_sample_rate: u64,
    pub audio_quality: String,
    pub audio_channels: u64,
}

#[serde_with::serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub start: u64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub end: u64,
}

/// The quality of a Stream
#[allow(missing_docs)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Quality {
    Tiny,
    Small,
    Medium,
    Large,

    HD720,
    HD1080,
    HD1440,
    HD2160,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: String,
}

impl PlayabilityStatus {
    fn as_error(&self) -> crate::error::Youtube {
        use crate::error::Youtube;

        eprintln!("{}", &self.reason);

        match self.reason.as_str() {
            "This video is unavailable." => Youtube::NotFound,
            "This video is no longer available because the YouTube account associated with this video has been terminated." => Youtube::AccountTerminated,
            "This video has been removed by the uploader" => Youtube::RemovedByUploader,
            "This video has been removed for violating YouTube's policy on nudity or sexual content." => Youtube::NudityOrSexualContentViolation,
            "This video is private." => Youtube::Private,
            "This video has been removed for violating YouTube's Terms of Service." => Youtube::TermsOfServiceViolation,
            "This video requires payment to watch." => Youtube::PurchaseRequired,
            "This video may be inappropriate for some users." => Youtube::AgeRestricted,
            copyright if copyright.starts_with("This video is no longer available due to a copyright claim by") => {
                let who = copyright.strip_prefix("This video is no longer available due to a copyright claim by ").unwrap().strip_suffix('.').unwrap();
                Youtube::CopyrightClaim {
                    claiment: who.to_string()
                }
            }
            unknown => unimplemented!("Unknown error reason: `{}`", unknown),
        }
    }
}
