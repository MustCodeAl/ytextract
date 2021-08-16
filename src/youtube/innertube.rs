use serde::Serialize;

use crate::youtube::{
    player_response,
    tv_config::{Config, TvConfig},
};

const DUMP: bool = false;

const BASE_URL: &str = "https://youtubei.googleapis.com/youtubei/v1";

const TV_CONFIG_URL: &str = "https://www.youtube.com/tv_config?action_get_config=true";

const CONTEXT: Context<'static> = Context {
    client: Client {
        hl: "en",
        gl: "US",
        client_name: "WEB",
        client_screen: None,
        client_version: "2.20210622.10.0",
    },
};

const CONTEXT_EMBEDDED: Context<'static> = Context {
    client: Client {
        hl: "en",
        gl: "US",
        client_name: "WEB",
        client_screen: Some("EMBED"),
        client_version: "2.20210622.10.0",
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Context<'a> {
    client: Client<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Client<'a> {
    hl: &'a str,
    gl: &'a str,
    client_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_screen: Option<&'a str>,
    client_version: &'a str,
}

pub enum ChannelPage {
    About,
}

pub enum Browse {
    Playlist(crate::playlist::Id),
    Channel {
        id: crate::channel::Id,
        page: ChannelPage,
    },
    Continuation(String),
}

pub struct Api {
    pub(crate) config: Config,
    pub(crate) http: reqwest::Client,
}

impl Api {
    pub async fn new() -> crate::Result<Self> {
        let http = reqwest::Client::new();
        let tv_config = http
            .get(TV_CONFIG_URL)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let tv_config = tv_config
            .lines()
            .nth(1)
            .expect("tv_config did not have a second line");

        let tv_config: TvConfig =
            serde_json::from_str(tv_config).expect("tv_config was invalid json");

        Ok(Self {
            config: tv_config
                .web_player_context_config
                .web_player_context_config_id_living_room_watch,
            http,
        })
    }

    async fn get<T: serde::de::DeserializeOwned, R: Serialize>(
        &self,
        endpoint: &'static str,
        request: R,
        context: Context<'static>,
    ) -> crate::Result<T> {
        #[derive(Serialize)]
        struct Request<R: Serialize> {
            context: Context<'static>,
            #[serde(flatten)]
            request: R,
        }

        let request = Request { context, request };

        let response = self
            .http
            .post(format!("{}/{}", BASE_URL, endpoint))
            .header("X-Goog-Api-Key", &self.config.innertube_api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        if DUMP {
            use std::time::SystemTime;
            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("TIME");
            std::fs::write(
                &format!("{}/{}.json", endpoint, time.as_millis()),
                &response,
            )
            .expect("Write");
        }

        match serde_json::from_str(&response) {
            Ok(ok) => Ok(ok),
            Err(err) => {
                eprintln!("Unable to parse JSON: {}", err);
                eprintln!(
                    "This is a bug. Please report it at https://github.com/ATiltedTree/ytextract"
                );
                panic!("Encountered fatal error: {}. Please report this.", err);
            }
        }
    }

    pub async fn streams(
        &self,
        id: crate::video::Id,
    ) -> crate::Result<player_response::StreamResult> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            video_id: crate::video::Id,
        }

        let request = Request { video_id: id };

        let res: player_response::StreamResult = self.get("player", request, CONTEXT).await?;

        match res.into_std() {
            Ok(streaming_data) => Ok(player_response::StreamResult::Ok { streaming_data }),
            Err(crate::Error::Youtube(crate::error::Youtube::AgeRestricted)) => {
                let request = Request { video_id: id };
                self.get("player", request, CONTEXT_EMBEDDED).await
            }
            Err(err) => Err(err),
        }
    }

    pub async fn player(&self, id: crate::video::Id) -> crate::Result<player_response::Result> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            video_id: crate::video::Id,
        }

        let request = Request { video_id: id };

        self.get("player", request, CONTEXT).await
    }

    pub async fn next(&self, id: crate::video::Id) -> crate::Result<serde_json::Value> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            video_id: crate::video::Id,
        }

        let request = Request { video_id: id };

        self.get("next", request, CONTEXT).await
    }

    pub async fn browse<T: serde::de::DeserializeOwned>(&self, browse: Browse) -> crate::Result<T> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            browse_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            params: Option<String>,
        }

        let request = match browse {
            Browse::Playlist(id) => Request {
                browse_id: format!("VL{}", id),
                params: Some(base64::encode([0xc2, 0x06, 0x02, 0x08, 0x00])),
            },
            Browse::Channel { id, page } => Request {
                browse_id: format!("{}", id),
                params: match page {
                    ChannelPage::About => Some(base64::encode(b"\x12\x05about")),
                },
            },
            Browse::Continuation(continuation) => {
                #[derive(Debug, Serialize)]
                #[serde(rename_all = "camelCase")]
                struct Request {
                    continuation: String,
                }

                let request = Request { continuation };

                return self.get("browse", request, CONTEXT).await;
            }
        };

        self.get("browse", request, CONTEXT).await
    }
}
