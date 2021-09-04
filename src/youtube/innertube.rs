use serde::Serialize;

use crate::youtube::player_response;

const DUMP: bool = false;
const BASE_URL: &str = "https://youtubei.googleapis.com/youtubei/v1";
const API_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

const CONTEXT_WEB: Context<'static> = Context {
    client: Client {
        hl: "en",
        gl: "US",
        client_name: "WEB",
        client_version: "2.20210622.10.0",
    },
};

const CONTEXT_ANDROID: Context<'static> = Context {
    client: Client {
        hl: "en",
        gl: "US",
        client_name: "ANDROID",
        client_version: "16.05",
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

pub enum Next {
    Video(crate::video::Id),
    Continuation(String),
}

#[derive(Clone)]
pub struct Api {
    pub(crate) http: reqwest::Client,
}

impl Api {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
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
            .header("X-Goog-Api-Key", API_KEY)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        if DUMP {
            let _ = std::fs::create_dir(endpoint);
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

        self.get("player", request, CONTEXT_ANDROID).await
    }

    pub async fn player(&self, id: crate::video::Id) -> crate::Result<player_response::Result> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            video_id: crate::video::Id,
        }

        let request = Request { video_id: id };

        self.get("player", request, CONTEXT_ANDROID).await
    }

    pub async fn next<T: serde::de::DeserializeOwned>(&self, next: Next) -> crate::Result<T> {
        match next {
            Next::Video(video_id) => {
                #[derive(Debug, Serialize)]
                #[serde(rename_all = "camelCase")]
                struct Request {
                    video_id: crate::video::Id,
                }

                let request = Request { video_id };

                self.get("next", request, CONTEXT_WEB).await
            }
            Next::Continuation(continuation) => {
                #[derive(Debug, Serialize)]
                #[serde(rename_all = "camelCase")]
                struct Request {
                    continuation: String,
                }

                let request = Request { continuation };

                self.get("next", request, CONTEXT_WEB).await
            }
        }
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

                return self.get("browse", request, CONTEXT_WEB).await;
            }
        };

        self.get("browse", request, CONTEXT_WEB).await
    }
}
