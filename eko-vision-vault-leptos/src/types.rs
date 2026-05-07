use js_sys::Date;

#[derive(Clone, Debug, PartialEq)]
pub struct Tip {
    pub amount: u32,
    pub currency: String,
    pub from: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub text: String,
    pub timestamp: f64, // ms since epoch (f64 matches JS Date)
    pub score: i64,
    pub views: u64,
    pub tips: Vec<Tip>,
}

impl Post {
    pub fn usd_tips(&self) -> u32 {
        self.tips.iter().filter(|t| t.currency == "USD").map(|t| t.amount).sum()
    }

    pub fn time_ago(&self) -> String {
        let now = Date::now();
        let diff = now - self.timestamp;
        let mins = (diff / 60_000.0) as u64;
        let hours = (diff / 3_600_000.0) as u64;
        let days = (diff / 86_400_000.0) as u64;
        if mins < 1 { "just now".into() }
        else if mins < 60 { format!("{}m ago", mins) }
        else if hours < 24 { format!("{}h ago", hours) }
        else { format!("{}d ago", days) }
    }
}

// ── JSON serialization (manual, no serde) ─────────────────────────────────────

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

fn ser_tip(t: &Tip) -> String {
    format!(r#"{{"amount":{},"currency":"{}","from":"{}"}}"#,
        t.amount, esc(&t.currency), esc(&t.from))
}

pub fn ser_post(p: &Post) -> String {
    let tips: Vec<String> = p.tips.iter().map(ser_tip).collect();
    format!(
        r#"{{"id":"{}","username":"{}","avatar":"{}","text":"{}","timestamp":{},"score":{},"views":{},"tips":[{}]}}"#,
        esc(&p.id), esc(&p.username), esc(&p.avatar), esc(&p.text),
        p.timestamp, p.score, p.views, tips.join(",")
    )
}

pub fn ser_posts(posts: &[Post]) -> String {
    let items: Vec<String> = posts.iter().map(ser_post).collect();
    format!("[{}]", items.join(","))
}

// ── JSON parsing (manual) ─────────────────────────────────────────────────────

pub fn parse_str(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    if !rest.starts_with('"') { return None; }
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
                c   => out.push(c),
            },
            c => out.push(c),
        }
    }
}

pub fn parse_i64(json: &str, key: &str) -> Option<i64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit() && c != '-').unwrap_or(rest.len());
    rest[..end].parse().ok()
}

pub fn parse_f64(json: &str, key: &str) -> Option<f64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit() && c != '.' && c != 'e' && c != '-').unwrap_or(rest.len());
    rest[..end].parse().ok()
}

pub fn parse_u32(json: &str, key: &str) -> Option<u32> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

pub fn parse_u64(json: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)? + needle.len();
    let rest = json[pos..].trim_start();
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

pub fn split_objects(s: &str) -> Vec<String> {
    let s = s.trim();
    let start = s.find('[').map(|i| i + 1).unwrap_or(0);
    let end = s.rfind(']').unwrap_or(s.len());
    if start >= end { return vec![]; }
    let content = &s[start..end];

    let mut result = Vec::new();
    let mut depth: i32 = 0;
    let mut in_str = false;
    let mut escaped = false;
    let mut obj_start: Option<usize> = None;

    for (i, c) in content.char_indices() {
        if escaped { escaped = false; continue; }
        if in_str {
            match c { '\\' => escaped = true, '"' => in_str = false, _ => {} }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' => { if depth == 0 { obj_start = Some(i); } depth += 1; }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = obj_start {
                        result.push(content[s..=i].to_string());
                        obj_start = None;
                    }
                }
            }
            _ => {}
        }
    }
    result
}

fn parse_tips_array(json: &str) -> Vec<Tip> {
    let needle = "\"tips\":[";
    let start = match json.find(needle) {
        Some(i) => i + needle.len() - 1,
        None => return vec![],
    };
    let rest = &json[start..];
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let mut end = rest.len();
    for (i, c) in rest.char_indices() {
        if escaped { escaped = false; continue; }
        if in_str { match c { '\\' => escaped = true, '"' => in_str = false, _ => {} } continue; }
        match c {
            '"' => in_str = true,
            '[' => depth += 1,
            ']' => { depth -= 1; if depth == 0 { end = i + 1; break; } }
            _ => {}
        }
    }
    split_objects(&rest[..end])
        .iter()
        .filter_map(|obj| {
            Some(Tip {
                amount: parse_u32(obj, "amount").unwrap_or(0),
                currency: parse_str(obj, "currency").unwrap_or_else(|| "USD".into()),
                from: parse_str(obj, "from").unwrap_or_else(|| "anon".into()),
            })
        })
        .collect()
}

pub fn parse_post(obj: &str) -> Option<Post> {
    Some(Post {
        id:        parse_str(obj, "id")?,
        username:  parse_str(obj, "username")?,
        avatar:    parse_str(obj, "avatar").unwrap_or_else(|| "?".into()),
        text:      parse_str(obj, "text")?,
        timestamp: parse_f64(obj, "timestamp").unwrap_or(0.0),
        score:     parse_i64(obj, "score").unwrap_or(0),
        views:     parse_u64(obj, "views").unwrap_or(0),
        tips:      parse_tips_array(obj),
    })
}

pub fn parse_posts(json: &str) -> Vec<Post> {
    split_objects(json).iter().filter_map(|o| parse_post(o)).collect()
}
