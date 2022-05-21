#![allow(dead_code)]
use crate::appcfg;
use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn log<'a, K>(s: K)
where
    K: Into<String>,
{
    web_sys::console::log_1(&JsValue::from_str(&s.into()));
}

pub fn any_other_user_agent(cfg: &appcfg::AppCfg) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let s_agent = rng.gen_range(0, cfg.agents.len());

    if let Some(agent) = cfg.agents.get(s_agent) {
        return agent.to_string();
    } else {
        any_other_user_agent(cfg)
    }
}

pub fn clear_page_cache() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    format!("{}", rng.gen::<u64>())
}
