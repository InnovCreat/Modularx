use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::http::{Request, Response};
use crate::store::{self, Store, Tip};
use crate::templates;

pub fn handle(mut stream: TcpStream, store: Arc<Mutex<Store>>) {
    if let Some(req) = Request::parse(&mut stream) {
        route(&req, store).send(&mut stream);
    }
}

fn route(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/")              => Response::html(templates::index_page()),
        ("GET", "/create")        => Response::html(templates::create_page()),
        ("GET", "/post")          => page_post(req),
        ("GET", "/api/posts")     => api_get_posts(req, store),
        ("POST", "/api/posts")    => api_create_post(req, store),
        ("POST", "/api/vote")     => api_vote(req, store),
        ("POST", "/api/tip")      => api_tip(req, store),
        ("POST", "/api/views")    => api_views(req, store),
        _                         => Response::not_found(),
    }
}

fn page_post(req: &Request) -> Response {
    let id = req.query.get("id").cloned().unwrap_or_default();
    if id.is_empty() {
        return Response::redirect("/");
    }
    Response::html(templates::post_page(&id))
}

fn api_get_posts(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    let s = store.lock().unwrap();
    if let Some(id) = req.query.get("id") {
        match s.get(id) {
            Some(post) => Response::json(store::serialize_post(post)),
            None => Response::not_found(),
        }
    } else {
        Response::json(store::serialize_posts(s.get_all()))
    }
}

fn api_create_post(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    let username = store::json_extract_str(&req.body, "username")
        .unwrap_or_else(|| "@anon".into());
    let text = match store::json_extract_str(&req.body, "text") {
        Some(t) if !t.trim().is_empty() => t,
        _ => return Response::bad_request("text required"),
    };

    let mut s = store.lock().unwrap();
    let post = s.create(username, text);
    Response::json(store::serialize_post(&post))
}

fn api_vote(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    let id = match store::json_extract_str(&req.body, "id") {
        Some(id) => id,
        None => return Response::bad_request("id required"),
    };
    let dir = store::json_extract_i64(&req.body, "dir")
        .unwrap_or(1)
        .clamp(-1, 1);

    let mut s = store.lock().unwrap();
    match s.vote(&id, dir) {
        Some(score) => Response::json(format!(r#"{{"score":{}}}"#, score)),
        None => Response::not_found(),
    }
}

fn api_tip(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    let id = match store::json_extract_str(&req.body, "id") {
        Some(id) => id,
        None => return Response::bad_request("id required"),
    };
    let amount = store::json_extract_u32(&req.body, "amount").unwrap_or(1);
    let currency = store::json_extract_str(&req.body, "currency")
        .unwrap_or_else(|| "USD".into());
    let from = store::json_extract_str(&req.body, "from")
        .unwrap_or_else(|| "anon".into());

    let tip = Tip { amount, currency, from };
    let mut s = store.lock().unwrap();
    if s.add_tip(&id, tip) {
        Response::json(r#"{"ok":true}"#.to_string())
    } else {
        Response::not_found()
    }
}

fn api_views(req: &Request, store: Arc<Mutex<Store>>) -> Response {
    let id = match store::json_extract_str(&req.body, "id") {
        Some(id) => id,
        None => return Response::bad_request("id required"),
    };
    store.lock().unwrap().increment_views(&id);
    Response::json(r#"{"ok":true}"#.to_string())
}
