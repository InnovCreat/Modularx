mod parser;
use parser::{parse_term_tree, Term};

use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

type AppState = Arc<RwLock<HashMap<String, Term>>>;

#[derive(Deserialize)]
struct QueryParams {
    #[serde(default)]
    search: Option<String>,
    #[serde(default)]
    domain: Option<String>,
    #[serde(default)]
    intent: Option<String>,
}

#[tokio::main]
async fn main() {
    let terms = parse_term_tree("static/VERITAS_HORTUS.tree").unwrap_or_else(|e| {
        eprintln!("Erreur chargement termes: {}", e);
        std::process::exit(1);
    });

    let state: AppState = Arc::new(RwLock::new(terms));

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/terms", get(get_all_terms))
        .route("/api/query", get(query_terms))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Veritas Hortus — Term Tree Explorer                    ║");
    println!("║  275+ termes • 7 dimensions • 639 Hz                    ║");
    println!("║  http://localhost:3000                                   ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> Html<String> {
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>index.html introuvable dans static/</h1>".to_string()),
    }
}

async fn get_all_terms(State(state): State<AppState>) -> Json<Vec<Term>> {
    let terms = state.read().await;
    let mut list: Vec<Term> = terms.values().cloned().collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    Json(list)
}

async fn query_terms(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<Vec<Term>> {
    let terms = state.read().await;
    let mut result: Vec<Term> = terms.values().cloned().collect();

    if let Some(s) = params.search {
        let s = s.to_lowercase();
        result.retain(|t| t.name.to_lowercase().contains(&s));
    }
    if let Some(d) = params.domain {
        if d != "All" {
            result.retain(|t| t.domain == d);
        }
    }
    if let Some(i) = params.intent {
        if i != "All" {
            result.retain(|t| t.intent == i);
        }
    }

    result.sort_by(|a, b| a.name.cmp(&b.name));
    Json(result)
}
