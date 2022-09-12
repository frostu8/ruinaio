pub mod app;
pub mod node;
pub mod menu;

use reqwest::Client;

pub use app::App;

#[derive(Clone, Debug)]
pub struct Context {
    api_client: Client,
}

impl PartialEq for Context {
    fn eq(&self, _other: &Context) -> bool {
        true
    }
}

pub fn origin() -> String {
    web_sys::window().unwrap().origin()
}

