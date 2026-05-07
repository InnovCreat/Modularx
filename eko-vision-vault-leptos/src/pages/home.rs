use leptos::prelude::*;

fn nav_to(path: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(path);
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = RwSignal::new(crate::store::load_posts());

    let max_score = move || {
        let ps = posts.get();
        ps.iter().map(|p| p.score).max().unwrap_or(1).max(1)
    };

    view! {
        <div class="app">
            <header class="hdr">
                <span class="logo">"EVK"</span>
                <span class="tagline">"merit-based ideas"</span>
                <button class="btn btn-primary" on:click=move |_| nav_to("/create")>
                    "＋ Post"
                </button>
            </header>
            <div class="section-lbl">"Latest Posts"</div>
            <main>
                {move || {
                    let ps = posts.get();
                    let mx = max_score();
                    if ps.is_empty() {
                        view! {
                            <div class="empty">
                                <div class="empty-icon">"◎"</div>
                                <p>"No posts yet. Be the first to share an idea."</p>
                            </div>
                        }.into_any()
                    } else {
                        let cards: Vec<_> = ps.iter().cloned().map(|p| {
                            let id = p.id.clone();
                            let go = {
                                let id = id.clone();
                                move |_| nav_to(&format!("/post?id={}", id))
                            };
                            let tip_go = {
                                let id = id.clone();
                                move |e: web_sys::MouseEvent| {
                                    e.stop_propagation();
                                    nav_to(&format!("/post?id={}", id));
                                }
                            };
                            let pct = if mx > 0 {
                                (p.score.max(0) as i64 * 100 / mx as i64).max(0)
                            } else {
                                0
                            };
                            let usd = p.usd_tips();
                            let preview: String = if p.text.len() > 260 {
                                format!("{}…", &p.text[..260])
                            } else {
                                p.text.clone()
                            };
                            let time = p.time_ago();
                            let avatar = p.avatar.clone();
                            let username = p.username.clone();
                            let score = p.score;
                            let views = p.views;

                            view! {
                                <div class="card" on:click=go>
                                    <div class="card-hdr">
                                        <div class="avatar">{avatar}</div>
                                        <div class="meta">
                                            <div class="uname">{username}</div>
                                            <div class="ts">{time}</div>
                                        </div>
                                    </div>
                                    <div class="card-body">{preview}</div>
                                    <div class="card-ftr">
                                        <div class="score-wrap">
                                            <div class="score-bar">
                                                <div class="score-fill" style=format!("width:{}%", pct)></div>
                                            </div>
                                            <span class="score-num">{score}</span>
                                            <span class="views">"👁 " {views}</span>
                                        </div>
                                        {if usd > 0 {
                                            view! { <span class="tip-total">"$"{usd}" tipped"</span> }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        <button class="btn-tip" on:click=tip_go>"Tip ⚡"</button>
                                    </div>
                                </div>
                            }
                        }).collect();
                        view! { <div>{cards}</div> }.into_any()
                    }
                }}
            </main>
            <nav class="bot-nav">
                <button class="nav-item active" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"⊡"</span>
                    "Feed"
                </button>
                <button class="nav-item" on:click=move |_| nav_to("/create")>
                    <span class="nav-icon">"◈"</span>
                    "Post"
                </button>
                <button class="nav-item" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"◎"</span>
                    "Profile"
                </button>
            </nav>
        </div>
    }
}
