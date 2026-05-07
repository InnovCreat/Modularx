use js_sys::Date;
use web_sys::window;

use crate::types::{Post, Tip, parse_posts, ser_posts};

const STORAGE_KEY: &str = "evv_posts";
const SEEDED_KEY: &str = "evv_seeded";

fn storage() -> Option<web_sys::Storage> {
    window()?.local_storage().ok()?
}

pub fn load_posts() -> Vec<Post> {
    let Some(s) = storage() else { return vec![] };
    let json = match s.get_item(STORAGE_KEY) {
        Ok(Some(v)) if !v.is_empty() => v,
        _ => return vec![],
    };
    parse_posts(&json)
}

fn save_posts(posts: &[Post]) {
    if let Some(s) = storage() {
        let _ = s.set_item(STORAGE_KEY, &ser_posts(posts));
    }
}

pub fn get_post(id: &str) -> Option<Post> {
    load_posts().into_iter().find(|p| p.id == id)
}

pub fn create_post(username: &str, text: &str) -> Post {
    let avatar = username
        .chars()
        .find(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".into());

    let post = Post {
        id: format!("p{}", Date::now() as u64),
        username: username.to_string(),
        avatar,
        text: text.to_string(),
        timestamp: Date::now(),
        score: 0,
        views: 0,
        tips: vec![],
    };

    let mut posts = load_posts();
    posts.insert(0, post.clone());
    save_posts(&posts);
    post
}

pub fn vote(id: &str, dir: i64) -> i64 {
    let mut posts = load_posts();
    if let Some(p) = posts.iter_mut().find(|p| p.id == id) {
        p.score += dir;
        let score = p.score;
        save_posts(&posts);
        return score;
    }
    0
}

pub fn add_tip(id: &str, tip: Tip) {
    let mut posts = load_posts();
    if let Some(p) = posts.iter_mut().find(|p| p.id == id) {
        p.tips.push(tip);
        save_posts(&posts);
    }
}

pub fn increment_views(id: &str) {
    let mut posts = load_posts();
    if let Some(p) = posts.iter_mut().find(|p| p.id == id) {
        p.views += 1;
        save_posts(&posts);
    }
}

pub fn ensure_seeded() {
    let Some(s) = storage() else { return };
    if s.get_item(SEEDED_KEY).ok().flatten().is_some() {
        return;
    }
    let now = Date::now();
    let posts = vec![
        Post {
            id: "seed-1".into(),
            username: "@merkaba_mind".into(),
            avatar: "M".into(),
            text: "The Fibonacci spiral isn't just a pattern in nature — it's a compression algorithm the universe uses to fold infinite complexity into finite form. Every nautilus shell, every galaxy arm, every tree branch is a lossy render of the same source code. What if we built cities the same way?".into(),
            timestamp: now - 7_200_000.0,
            score: 142, views: 891,
            tips: vec![
                Tip { amount: 5, currency: "USD".into(), from: "@nova_lens".into() },
                Tip { amount: 1, currency: "USD".into(), from: "anon".into() },
            ],
        },
        Post {
            id: "seed-2".into(),
            username: "@nova_lens".into(),
            avatar: "N".into(),
            text: "Unpopular take: most AI art tools are training people to be worse at prompting reality. When you can hallucinate any image instantly, you stop learning how to see. The friction in photography — light, timing, positioning — was never a bug. It was the curriculum.".into(),
            timestamp: now - 18_000_000.0,
            score: 89, views: 534,
            tips: vec![Tip { amount: 1, currency: "USD".into(), from: "@echo_field".into() }],
        },
        Post {
            id: "seed-3".into(),
            username: "@prism_arc".into(),
            avatar: "P".into(),
            text: "Sound at 639 Hz — the frequency of connection and relationships — causes water molecules to form hexagonal lattice structures. We're 60% water. Every conversation you have is a geometry lesson your body is silently taking. Choose your frequencies.".into(),
            timestamp: now - 39_600_000.0,
            score: 217, views: 1204,
            tips: vec![
                Tip { amount: 5, currency: "USD".into(), from: "@merkaba_mind".into() },
                Tip { amount: 20, currency: "USD".into(), from: "anon".into() },
            ],
        },
        Post {
            id: "seed-4".into(),
            username: "@echo_field".into(),
            avatar: "E".into(),
            text: "The best ideas I've ever had came in the three minutes between waking and getting up. Not dreams — something else. A liminal channel. I've started keeping a voice recorder on my nightstand. Three months of recordings. The patterns are starting to show.".into(),
            timestamp: now - 64_800_000.0,
            score: 56, views: 320,
            tips: vec![],
        },
    ];
    save_posts(&posts);
    let _ = s.set_item(SEEDED_KEY, "1");
}
