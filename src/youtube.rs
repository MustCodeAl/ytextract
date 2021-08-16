use std::ops::Deref;

use serde::Deserialize;

pub mod browse;
pub mod innertube;
pub mod player_response;
pub mod tv_config;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNameRuns {
    pub runs: Vec<ChannelNameRun>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNameRun {
    pub text: String,
    pub navigation_endpoint: NavigationEndpoint,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEndpoint {
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: crate::channel::Id,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Text {
    SimpleText(SimpleText),
    Runs(Runs),
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Runs {
    pub runs: Vec<TitleRun>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TitleRun {
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub metadata_badge_renderer: MetadataBadgeRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeRenderer {
    pub style: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub thumbnails: Vec<crate::Thumbnail>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationEndpoint {
    pub continuation_command: ContinuationCommand,
}

#[derive(Debug, Deserialize, Clone)]
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
