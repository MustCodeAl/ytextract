use std::ops::Deref;

use super::{
    parse_date, parse_subscribers, Badge, ChannelNameRuns, ContinuationItemRenderer, SimpleText,
    Text, Thumbnails, TitleRun,
};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub contents: Contents,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_watch_next_results: TwoColumnWatchNextResults,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnWatchNextResults {
    pub results: Results,
    pub secondary_results: Option<SecondaryResults>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results {
    pub results: Results2,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Results2 {
    pub contents: Vec<Content>,
}

impl Results2 {
    pub fn primary(&self) -> &VideoPrimaryInfoRenderer {
        self.contents
            .iter()
            .find_map(|x| match x {
                Content::VideoPrimaryInfoRenderer(ret) => Some(ret),
                _ => None,
            })
            .expect("VideoPrimaryInfoRenderer was not found")
    }

    pub fn secondary(&self) -> &VideoSecondaryInfoRenderer {
        self.contents
            .iter()
            .find_map(|x| match x {
                Content::VideoSecondaryInfoRenderer(ret) => Some(ret),
                _ => None,
            })
            .expect("VideoSecondaryInfoRenderer was not found")
    }

    pub fn comments(&self) -> Option<&ContinuationItemRenderer> {
        self.contents
            .iter()
            .find_map(|x| match x {
                Content::ItemSectionRenderer(ret) => Some(ret),
                _ => None,
            })
            .and_then(|x| x.contents.0.continuation_item_renderer.as_ref())
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Content {
    VideoPrimaryInfoRenderer(VideoPrimaryInfoRenderer),
    VideoSecondaryInfoRenderer(VideoSecondaryInfoRenderer),
    ItemSectionRenderer(ItemSectionRenderer),
    #[serde(other)]
    Other,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPrimaryInfoRenderer {
    pub video_actions: VideoActions,
    #[serde(default)]
    pub super_title_link: SuperTitleLink,
    pub date_text: SimpleText,
}

impl VideoPrimaryInfoRenderer {
    pub fn date(&self) -> chrono::NaiveDate {
        let date_str = self
            .date_text
            .deref()
            .trim_start_matches("Streamed live on ");

        parse_date(date_str).expect("Unable to parse date")
    }

    pub fn likes(&self) -> Option<u64> {
        // `like this video along with 4,457 other people` or `I like this`
        let label = &self
            .video_actions
            .menu_renderer
            .like_button()?
            .accessibility
            .as_ref()?
            .label;

        let (_, likes) = label.split_once("like this video along with ")?;
        let (likes, _) = likes.split_once(" other people")?;

        let likes = likes.replace(',', "");

        let likes = likes
            .parse()
            .expect("Likes we not parsable as a unsigned integer");

        Some(likes)
    }

    pub fn hashtags(&self) -> impl Iterator<Item = &str> {
        self.super_title_link.runs.iter().map(|x| x.text.as_str())
    }
}
#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoActions {
    pub menu_renderer: MenuRenderer,
}

#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuRenderer {
    pub top_level_buttons: Vec<TopLevelButton>,
}

impl MenuRenderer {
    fn like_button(&self) -> Option<&ToggleButtonRenderer> {
        self.top_level_buttons.iter().find_map(|x| match x {
            TopLevelButton::ToggleButtonRenderer(ref button) => Some(button),
            TopLevelButton::ButtonRenderer {} => None,
            TopLevelButton::DownloadButtonRenderer {} => None,
            TopLevelButton::SegmentedLikeDislikeButtonRenderer {} => None,
        })
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TopLevelButton {
    ToggleButtonRenderer(ToggleButtonRenderer),
    DownloadButtonRenderer {},
    ButtonRenderer {},
    SegmentedLikeDislikeButtonRenderer {},
}

#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToggleButtonRenderer {
    pub accessibility: Option<AccessibilityData>,
}

#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SuperTitleLink {
    pub runs: Vec<TitleRun>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSecondaryInfoRenderer {
    pub owner: Owner,
    pub metadata_row_container: MetadataRowContainer,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub video_owner_renderer: VideoOwnerRenderer,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOwnerRenderer {
    pub thumbnail: Thumbnails,
    pub subscriber_count_text: Option<SimpleText>,
}
impl VideoOwnerRenderer {
    pub fn subscribers(&self) -> Option<u64> {
        self.subscriber_count_text.as_ref().map(|x| {
            parse_subscribers(
                x.simple_text
                    .split_once(' ')
                    .expect("no space in subscriber_count_text")
                    .0,
            )
            .expect("Unable to parse subscriber count")
        })
    }

    pub fn thumbnails(&self) -> &Vec<crate::Thumbnail> {
        &self.thumbnail.thumbnails
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowContainer {
    pub metadata_row_container_renderer: MetadataRowContainerRenderer,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowContainerRenderer {
    pub collapsed_item_count: i64,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer {
    pub contents: (Content2,),
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub continuation_item_renderer: Option<ContinuationItemRenderer>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryResults {
    pub secondary_results: SecondaryResults2,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SecondaryResults2 {
    WithChips {
        #[serde(default)]
        results: Vec<SecondaryResultsItem>,
    },
    WithoutChips {
        #[serde(default)]
        results: Vec<SectionItem>,
    },
}

impl SecondaryResults2 {
    pub fn items(&self) -> Option<impl Iterator<Item = SectionItem>> {
        match self {
            SecondaryResults2::WithChips { results } => results.iter().find_map(|x| match x {
                SecondaryResultsItem::ItemSectionRenderer(r) => {
                    Some(r.contents.clone().into_iter())
                }
                SecondaryResultsItem::Other => None,
            }),
            SecondaryResults2::WithoutChips { results } => Some(results.clone().into_iter()),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecondaryResultsItem {
    ItemSectionRenderer(SecondaryItemSectionRenderer),
    #[serde(other)]
    Other,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryItemSectionRenderer {
    #[serde(default)]
    pub contents: Vec<SectionItem>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionItem {
    CompactVideoRenderer(CompactVideoRenderer),
    CompactPlaylistRenderer(CompactPlaylistRenderer),
    CompactRadioRenderer(CompactRadioRenderer),
    CompactMovieRenderer(CompactMovieRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer),
    #[serde(other)]
    Other,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactVideoRenderer {
    pub video_id: crate::video::Id,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    // channel name
    pub short_byline_text: ChannelNameRuns,
    pub view_count_text: Option<Text>,
    pub length_text: Option<SimpleText>,

    #[serde(default)]
    pub owner_badges: Vec<Badge>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactPlaylistRenderer {
    pub playlist_id: String,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    // channel name
    pub short_byline_text: Text<SimpleText, super::Runs<OptionalChannelNameRun>>,
    // length
    pub video_count_short_text: SimpleText,

    #[serde(default)]
    pub owner_badges: Vec<Badge>,
}

impl CompactPlaylistRenderer {
    pub fn channel_name(&self) -> &str {
        match &self.short_byline_text {
            Text::SimpleText(s) => &s.simple_text,
            Text::Runs(r) => &r.runs[0].text,
        }
    }

    pub fn channel_id(&self) -> Option<crate::channel::Id> {
        match &self.short_byline_text {
            Text::SimpleText(_) => None,
            Text::Runs(r) => r.runs[0]
                .navigation_endpoint
                .clone()
                .map(|x| x.browse_endpoint.browse_id),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactRadioRenderer {
    pub playlist_id: String,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactMovieRenderer {
    pub video_id: crate::video::Id,
    pub thumbnail: Thumbnails,
    pub title: SimpleText,
    pub length_text: SimpleText,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionalChannelNameRun {
    pub text: String,
    pub navigation_endpoint: Option<super::NavigationEndpoint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continuation {
    pub on_response_received_endpoints: Vec<OnResponseReceivedEndpoint>,
}

impl Continuation {
    pub fn into_videos(self) -> impl Iterator<Item = SectionItem> {
        self.on_response_received_endpoints
            .into_iter()
            .find_map(|x| {
                x.append_continuation_items_action
                    .map(|x| x.continuation_items.into_iter())
            })
            .unwrap_or_else(|| vec![].into_iter())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnResponseReceivedEndpoint {
    #[serde(default)]
    pub append_continuation_items_action: Option<AppendContinuationItemsAction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendContinuationItemsAction {
    pub continuation_items: Vec<SectionItem>,
}
