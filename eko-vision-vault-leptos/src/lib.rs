mod types;
mod store;
mod pages;

use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use pages::{home::HomePage, create::CreatePage, post::PostPage};

fn get_path() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/".into())
}

fn get_query_param(key: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    params.get(key)
}

#[component]
fn App() -> impl IntoView {
    store::ensure_seeded();

    let path = get_path();
    let path_str = path.trim_end_matches('/').to_string();

    match path_str.as_str() {
        "/create" => view! { <CreatePage /> }.into_any(),
        "/post" => {
            match get_query_param("id") {
                Some(id) => view! { <PostPage id=id /> }.into_any(),
                None => {
                    if let Some(w) = web_sys::window() {
                        let _ = w.location().set_href("/");
                    }
                    view! { <div class="loading">"Redirecting…"</div> }.into_any()
                }
            }
        }
        _ => view! { <HomePage /> }.into_any(),
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    mount_to_body(App);
}
