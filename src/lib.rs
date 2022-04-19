use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod utils;
use utils::log;

mod appcfg;
mod attack;
mod target;

const PATTERN_ERROR_MSG: &str = "Something went wrong";

#[wasm_bindgen]
pub async fn run() {
    let cfg: appcfg::AppCfg = appcfg::load_app_cfg().expect(PATTERN_ERROR_MSG);
    let targets: Vec<target::Target> = target::Target::from_cfg(cfg.dist)
        .await
        .expect(PATTERN_ERROR_MSG);
    attack::start_one(targets)
        .await
        .expect(PATTERN_ERROR_MSG);
}
