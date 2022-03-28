#![allow(clippy::enum_variant_names)]

use std::ops::Deref;

use serde::Deserialize;

pub mod browse;
pub mod innertube;
pub mod next;
pub mod player_response;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Runs<T> {
    pub runs: Vec<T>,
}

pub type ChannelNameRuns = Runs<ChannelNameRun>;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNameRun {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: crate::channel::Id,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Text {
    SimpleText(SimpleText),
    Runs(TitleRuns),
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    pub simple_text: String,
}

impl Deref for SimpleText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.simple_text
    }
}

pub type TitleRuns = Runs<TitleRun>;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TitleRun {
    pub text: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub metadata_badge_renderer: MetadataBadgeRenderer,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer {
    pub style: String,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub thumbnails: Vec<crate::Thumbnail>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemRenderer {
    pub continuation_endpoint: ContinuationEndpoint,
}

impl ContinuationItemRenderer {
    pub fn get(&self) -> String {
        self.continuation_endpoint
            .continuation_command
            .token
            .clone()
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationEndpoint {
    pub continuation_command: ContinuationCommand,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationCommand {
    pub token: String,
}

pub fn parse_subscribers(value: &str) -> Option<u64> {
    let last = value.chars().last()?;
    if last.is_numeric() {
        value.parse().ok()
    } else {
        let val = &value[..value.len() - 1];
        let val: f64 = val.parse().ok()?;
        let mul = match last {
            'K' => 1_000.0,
            'M' => 1_000_000.0,
            modifier => unimplemented!("Unknown modifier '{}'", modifier),
        };

        Some((val * mul) as u64)
    }
}

/// Parse a video length in the format `HH:MM:SS`
pub fn parse_length(value: &str) -> std::time::Duration {
    std::time::Duration::from_secs(value.split(':').rev().enumerate().fold(0, |acc, (i, s)| {
        let s: u64 = s.parse().unwrap();
        let mul = 60u64.pow(i as u32);
        acc + (s * mul)
    }))
}

/// Parse a video upload data in the format `[Premiered |Premires ]<MONTH_NAME> <DAY>, <YEAR>`
pub fn parse_date(value: &str) -> Option<chrono::NaiveDate> {
    const PREFIXES: &[&str] = &["Premiered ", "Premieres "];

    let value = PREFIXES
        .iter()
        .find_map(|x| value.strip_prefix(x))
        .unwrap_or(value);

    chrono::NaiveDate::parse_from_str(value, "%b %e, %Y").ok()
}

/// Strips the various possible domains of a youtube URL
pub fn strip_url_prefix(url: &str) -> &str {
    const PREFIXES: &[&str] = &[
        "https://www.youtube.com/",
        "https://m.youtube.com/",
        "https://youtube.com/",
        "https://youtu.be/",
    ];

    PREFIXES
        .iter()
        .find_map(|prefix| url.strip_prefix(prefix))
        .unwrap_or(url)
}
