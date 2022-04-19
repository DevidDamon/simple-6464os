use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Target {
    #[serde(deserialize_with = "request_method_from_string")]
    pub method: reqwest::Method,
    #[serde(
        default,
        deserialize_with = "structured_url_from_plain_string",
        rename = "page"
    )]
    pub url: Option<url::Url>,
    #[serde(default)]
    pub ip: Option<std::net::IpAddr>,
    #[serde(default)]
    pub port: Option<i32>,
}

impl Target {
    pub async fn from_cfg(path: String) -> Result<Vec<Self>, ParseTargetError> {
        if Self::is_network(&path) {
            let body = reqwest::get(path).await?.text().await?;
            // crate::log(&body);
            let targets =
                serde_json::from_str::<Vec<Target>>(&body).map_err(ParseTargetError::Parse);
            // crate::log(format!("{:#?}", &targets));
            targets
        } else {
            Err(ParseTargetError::Other("Unimplemented".into()))
        }
    }

    fn is_network(path: &String) -> bool {
        path.starts_with("http")
    }
}

fn structured_url_from_plain_string<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        url::Url::parse(&s)
            .map(Option::Some)
            .map_err(serde::de::Error::custom)
    }
}

fn request_method_from_string<'de, D>(deserializer: D) -> Result<reqwest::Method, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    reqwest::Method::from_bytes(s.to_uppercase().as_bytes()).map_err(serde::de::Error::custom)
}

#[derive(Debug)]
pub enum ParseTargetError {
    Other(String),
    Reqwest(reqwest::Error),
    Parse(serde_json::Error),
}

impl From<reqwest::Error> for ParseTargetError {
    fn from(e: reqwest::Error) -> ParseTargetError {
        ParseTargetError::Reqwest(e)
    }
}

impl From<serde_json::Error> for ParseTargetError {
    fn from(e: serde_json::Error) -> ParseTargetError {
        ParseTargetError::Parse(e)
    }
}

impl std::fmt::Display for ParseTargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseTargetError::*;

        write!(
            f,
            "{}",
            match self {
                Reqwest(s) => format!("Network error {s}"),
                Parse(s) => format!("Parse error {s}"),
                Other(s) => format!("Uncategory error {s}"),
            }
        )
    }
}
