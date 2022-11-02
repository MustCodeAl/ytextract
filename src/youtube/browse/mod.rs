use super::Text;
use serde::Deserialize;

pub mod channel;
pub mod playlist;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Result<T> {
    Error { alerts: (Alert,) },
    Ok(T),
}

impl<T> Result<T> {
    pub fn into_std(self) -> crate::Result<T> {
        match self {
            Self::Error { alerts } => {
                assert_eq!(alerts.0.alert_renderer.r#type, "ERROR");

                Err(crate::Error::Youtube(crate::error::Youtube(
                    alerts.0.alert_renderer.text(),
                )))
            }
            Self::Ok(ok) => Ok(ok),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub alert_renderer: AlertRenderer,
}

#[derive(Deserialize)]
pub struct AlertRenderer {
    pub r#type: String,
    pub text: Text,
}

impl AlertRenderer {
    fn text(self) -> String {
        match self.text {
            Text::SimpleText(simple_text) => simple_text.simple_text,
            Text::Runs(mut runs) => runs.runs.swap_remove(0).text,
        }
    }
}
