use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const DATA_FILE: &str = "evv_data.json";
static COUNTER: AtomicU64 = AtomicU64::new(0);

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Tip {
    pub amount: u32,
    pub currency: String,
    pub from: String,
}

#[derive(Clone)]
pub struct Post {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub text: String,
    pub timestamp: u64,
    pub score: i64,
    pub views: u64,
    pub tips: Vec<Tip>,
}

// ── Store ─────────────────────────────────────────────────────────────────────

pub struct Store {
    pub posts: Vec<Post>,
}

impl Store {
    pub fn load() -> Self {
        let posts = fs::read_to_string(DATA_FILE)
            .ok()
            .and_then(|s| parse_posts(&s))
            .unwrap_or_else(seed_posts);

        let s = Self { posts };
        s.save();
        s
    }

    pub fn save(&self) {
        let _ = fs::write(DATA_FILE, serialize_posts(&self.posts));
    }

    pub fn get_all(&self) -> &[Post] {
        &self.posts
    }

    pub fn get(&self, id: &str) -> Option<&Post> {
        self.posts.iter().find(|p| p.id == id)
    }

    pub fn create(&mut self, username: String, text: String) -> Post {
        let avatar = username
            .chars()
            .find(|c| c.is_alphabetic())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string());

        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let post = Post {
            id: format!("p{}{}", now_ms(), n),
            username,
            avatar,
            text,
            timestamp: now_ms(),
            score: 0,
            views: 0,
            tips: Vec::new(),
        };
        self.posts.insert(0, post.clone());
        self.save();
        post
    }

    pub fn vote(&mut self, id: &str, dir: i64) -> Option<i64> {
        let idx = self.posts.iter().position(|p| p.id == id)?;
        self.posts[idx].score += dir;
        let score = self.posts[idx].score;
        self.save();
        Some(score)
    }

    pub fn add_tip(&mut self, id: &str, tip: Tip) -> bool {
        match self.posts.iter().position(|p| p.id == id) {
            Some(idx) => {
                self.posts[idx].tips.push(tip);
                self.save();
                true
            }
            None => false,
        }
    }

    pub fn increment_views(&mut self, id: &str) {
        if let Some(idx) = self.posts.iter().position(|p| p.id == id) {
            self.posts[idx].views += 1;
            self.save();
        }
    }
}

// ── Serialization ─────────────────────────────────────────────────────────────

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn serialize_tip(t: &Tip) -> String {
    format!(
        r#"{{"amount":{},"currency":"{}","from":"{}"}}"#,
        t.amount,
        esc(&t.currency),
        esc(&t.from)
    )
}

pub fn serialize_post(p: &Post) -> String {
    let tips: Vec<String> = p.tips.iter().map(serialize_tip).collect();
    format!(
        r#"{{"id":"{}","username":"{}","avatar":"{}","text":"{}","timestamp":{},"score":{},"views":{},"tips":[{}]}}"#,
        esc(&p.id),
        esc(&p.username),
        esc(&p.avatar),
        esc(&p.text),
        p.timestamp,
        p.score,
        p.views,
        tips.join(",")
    )
}

pub fn serialize_posts(posts: &[Post]) -> String {
    let items: Vec<String> = posts.iter().map(serialize_post).collect();
    format!("[{}]", items.join(","))
}

// ── Parsing ───────────────────────────────────────────────────────────────────

pub fn json_extract_str(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let rest = &rest[1..];
    let mut out = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next()? {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                c => out.push(c),
            },
            c => out.push(c),
        }
    }
}

pub fn json_extract_i64(json: &str, key: &str) -> Option<i64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '-')
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

pub fn json_extract_u32(json: &str, key: &str) -> Option<u32> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn json_extract_u64(json: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn split_json_objects(s: &str) -> Vec<String> {
    let s = s.trim();
    let inner_start = s.find('[').map(|i| i + 1).unwrap_or(0);
    let inner_end = s.rfind(']').unwrap_or(s.len());
    if inner_start >= inner_end {
        return vec![];
    }
    let content = &s[inner_start..inner_end];

    let mut result = Vec::new();
    let mut depth: i32 = 0;
    let mut in_str = false;
    let mut escaped = false;
    let mut start: Option<usize> = None;

    for (i, c) in content.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_str {
            match c {
                '\\' => escaped = true,
                '"' => in_str = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = start {
                        result.push(content[s..=i].to_string());
                        start = None;
                    }
                }
            }
            _ => {}
        }
    }
    result
}

fn extract_tips_array(json: &str) -> Vec<Tip> {
    let needle = "\"tips\":[";
    let start = match json.find(needle) {
        Some(i) => i + needle.len() - 1,
        None => return vec![],
    };

    // Find the matching ] for this array
    let rest = &json[start..];
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let mut end = rest.len();

    for (i, c) in rest.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_str {
            match c {
                '\\' => escaped = true,
                '"' => in_str = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    let array_str = &rest[..end];
    split_json_objects(array_str)
        .iter()
        .filter_map(|obj| {
            Some(Tip {
                amount: json_extract_u32(obj, "amount").unwrap_or(0),
                currency: json_extract_str(obj, "currency").unwrap_or_else(|| "USD".into()),
                from: json_extract_str(obj, "from").unwrap_or_else(|| "anon".into()),
            })
        })
        .collect()
}

fn parse_post(obj: &str) -> Option<Post> {
    Some(Post {
        id: json_extract_str(obj, "id")?,
        username: json_extract_str(obj, "username")?,
        avatar: json_extract_str(obj, "avatar").unwrap_or_else(|| "?".into()),
        text: json_extract_str(obj, "text")?,
        timestamp: json_extract_u64(obj, "timestamp").unwrap_or(0),
        score: json_extract_i64(obj, "score").unwrap_or(0),
        views: json_extract_u64(obj, "views").unwrap_or(0),
        tips: extract_tips_array(obj),
    })
}

fn parse_posts(json: &str) -> Option<Vec<Post>> {
    let objects = split_json_objects(json);
    if objects.is_empty() {
        return None;
    }
    let posts: Vec<Post> = objects.iter().filter_map(|o| parse_post(o)).collect();
    if posts.is_empty() { None } else { Some(posts) }
}

// ── Seed data ─────────────────────────────────────────────────────────────────

fn seed_posts() -> Vec<Post> {
    let base = now_ms();
    vec![
        Post {
            id: "seed-1".into(),
            username: "@merkaba_mind".into(),
            avatar: "M".into(),
            text: "The Fibonacci spiral isn't just a pattern in nature — it's a compression algorithm the universe uses to fold infinite complexity into finite form. Every nautilus shell, every galaxy arm, every tree branch is a lossy render of the same source code. What if we built cities the same way?".into(),
            timestamp: base - 7_200_000,
            score: 142,
            views: 891,
            tips: vec![
                Tip { amount: 5, currency: "USD".into(), from: "@nova_lens".into() },
                Tip { amount: 1, currency: "USD".into(), from: "anon".into() },
                Tip { amount: 20, currency: "Doge".into(), from: "@prism_arc".into() },
            ],
        },
        Post {
            id: "seed-2".into(),
            username: "@nova_lens".into(),
            avatar: "N".into(),
            text: "Unpopular take: most AI art tools are training people to be worse at prompting reality. When you can hallucinate any image instantly, you stop learning how to see. The friction in photography — light, timing, positioning — was never a bug. It was the curriculum.".into(),
            timestamp: base - 18_000_000,
            score: 89,
            views: 534,
            tips: vec![
                Tip { amount: 1, currency: "USD".into(), from: "@echo_field".into() },
            ],
        },
        Post {
            id: "seed-3".into(),
            username: "@prism_arc".into(),
            avatar: "P".into(),
            text: "Sound at 639 Hz — the frequency of connection and relationships — causes water molecules to form hexagonal lattice structures. We're 60% water. Every conversation you have is a geometry lesson your body is silently taking. Choose your frequencies.".into(),
            timestamp: base - 39_600_000,
            score: 217,
            views: 1204,
            tips: vec![
                Tip { amount: 5, currency: "USD".into(), from: "@merkaba_mind".into() },
                Tip { amount: 5, currency: "USD".into(), from: "@nova_lens".into() },
                Tip { amount: 20, currency: "USD".into(), from: "anon".into() },
            ],
        },
        Post {
            id: "seed-4".into(),
            username: "@echo_field".into(),
            avatar: "E".into(),
            text: "The best ideas I've ever had came in the three minutes between waking and getting up. Not dreams — something else. A liminal channel. I've started keeping a voice recorder on my nightstand. Three months of recordings. The patterns are starting to show.".into(),
            timestamp: base - 64_800_000,
            score: 56,
            views: 320,
            tips: vec![],
        },
    ]
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
