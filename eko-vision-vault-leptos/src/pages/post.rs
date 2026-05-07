use leptos::prelude::*;
use crate::store;
use crate::types::Tip;

fn nav_to(path: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(path);
    }
}

fn copy_to_clipboard(text: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.navigator().clipboard().write_text(text);
    }
}

fn show_toast(msg: &str) {
    use wasm_bindgen::JsCast;
    let Some(w) = web_sys::window() else { return };
    let Some(doc) = w.document() else { return };
    if let Some(el) = doc.get_element_by_id("toast") {
        el.set_text_content(Some(msg));
        let _ = el.class_list().add_1("show");
        let closure = wasm_bindgen::closure::Closure::once(move || {
            if let Some(w) = web_sys::window() {
                if let Some(doc) = w.document() {
                    if let Some(el) = doc.get_element_by_id("toast") {
                        let _ = el.class_list().remove_1("show");
                    }
                }
            }
        });
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            3000,
        );
        closure.forget();
    }
}

#[derive(Clone, PartialEq)]
struct TipPreset {
    label: &'static str,
    amount: u32,
    currency: &'static str,
}

const PRESETS: &[TipPreset] = &[
    TipPreset { label: "☕ $1",     amount: 1,  currency: "USD" },
    TipPreset { label: "🎯 $5",     amount: 5,  currency: "USD" },
    TipPreset { label: "💎 $20",    amount: 20, currency: "USD" },
    TipPreset { label: "🎱 10 Doge", amount: 10, currency: "Doge" },
];

#[component]
pub fn PostPage(id: String) -> impl IntoView {
    store::increment_views(&id);

    let post = RwSignal::new(store::get_post(&id));
    let sel_amount = RwSignal::new(1u32);
    let sel_currency = RwSignal::new("USD".to_string());
    let active_preset = RwSignal::new(0usize);
    let custom_amount = RwSignal::new(String::new());
    let tip_from = RwSignal::new(String::new());
    let vote = {
        let id = id.clone();
        move |dir: i64| {
            let new_score = store::vote(&id, dir);
            post.update(|p| {
                if let Some(p) = p {
                    p.score = new_score;
                }
            });
        }
    };

    let send_tip = {
        let id = id.clone();
        move |_| {
            let amt = if custom_amount.get().is_empty() {
                sel_amount.get()
            } else {
                custom_amount.get().parse::<u32>().unwrap_or(0)
            };
            if amt == 0 {
                show_toast("Pick a tip amount first");
                return;
            }
            let from = {
                let s = tip_from.get();
                if s.trim().is_empty() { "anon".to_string() } else { s }
            };
            let tip = Tip {
                amount: amt,
                currency: sel_currency.get(),
                from,
            };
            store::add_tip(&id, tip.clone());
            post.update(|p| {
                if let Some(p) = p { p.tips.push(tip); }
            });
            show_toast("Tip sent! ⚡");
        }
    };

    view! {
        <div class="app">
            <header class="hdr">
                <button class="btn-ghost" on:click=move |_| nav_to("/")>"← Back"</button>
                <span class="tagline">"Post"</span>
                <span style="width:60px"></span>
            </header>

            {move || match post.get() {
                None => view! {
                    <div class="error-msg">"Post not found."</div>
                }.into_any(),
                Some(p) => {
                    let time = p.time_ago();
                    view! {
                        <div class="post-wrap">
                            <div class="post-hdr">
                                <div class="avatar avatar-lg">{p.avatar.clone()}</div>
                                <div class="meta">
                                    <div class="uname">{p.username.clone()}</div>
                                    <div class="ts">{time}" · 👁 "{p.views}</div>
                                </div>
                            </div>
                            <div class="card-body-full">{p.text.clone()}</div>
                            <div class="vote-row">
                                <button class="vote-btn" on:click={
                                    let vote = vote.clone();
                                    move |_| vote(1)
                                } title="Upvote">"▲"</button>
                                <span class="score-big">{p.score}</span>
                                <button class="vote-btn" on:click={
                                    let vote = vote.clone();
                                    move |_| vote(-1)
                                } title="Downvote">"▼"</button>
                            </div>
                        </div>

                        // Tip section
                        <div class="post-wrap" style="padding-top:0">
                            <div class="tip-section">
                                <div class="tip-lbl">"⚡ Send a tip"</div>
                                <div class="tip-chips">
                                    {PRESETS.iter().enumerate().map(|(i, preset)| {
                                        let amt = preset.amount;
                                        let cur = preset.currency.to_string();
                                        let lbl = preset.label;
                                        view! {
                                            <button
                                                class=move || if active_preset.get() == i && custom_amount.get().is_empty() {
                                                    "chip active"
                                                } else {
                                                    "chip"
                                                }
                                                on:click=move |_| {
                                                    sel_amount.set(amt);
                                                    sel_currency.set(cur.clone());
                                                    active_preset.set(i);
                                                    custom_amount.set(String::new());
                                                }
                                            >{lbl}</button>
                                        }
                                    }).collect_view()}
                                </div>
                                <div class="tip-custom">
                                    <span style="color:#666;font-size:13px;white-space:nowrap">"Custom $"</span>
                                    <input
                                        class="inp inp-sm"
                                        type="number"
                                        min="1"
                                        placeholder="0"
                                        on:input=move |e| {
                                            custom_amount.set(event_target_value(&e));
                                            sel_currency.set("USD".into());
                                        }
                                        prop:value=custom_amount
                                    />
                                </div>
                                <div class="tip-from-row">
                                    <span class="tip-from-lbl">"From"</span>
                                    <input
                                        class="inp"
                                        type="text"
                                        placeholder="@yourname or anon"
                                        maxlength="40"
                                        on:input=move |e| tip_from.set(event_target_value(&e))
                                        prop:value=tip_from
                                    />
                                </div>
                                <button class="btn-send" on:click=send_tip.clone()>
                                    "Send Tip ⚡"
                                </button>
                            </div>

                            // Tips received
                            <div class="tips-list">
                                {move || {
                                    let tips = post.get().map(|p| p.tips).unwrap_or_default();
                                    let usd: u32 = tips.iter().filter(|t| t.currency == "USD").map(|t| t.amount).sum();
                                    if tips.is_empty() {
                                        view! {
                                            <div style="color:#444;font-size:13px">"No tips yet — be the first!"</div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div style="font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;color:#555;margin-bottom:10px">
                                                "Tips received"
                                                {if usd > 0 { format!(" · ${} total", usd) } else { String::new() }}
                                            </div>
                                            {tips.iter().map(|t| {
                                                let amt_str = if t.currency == "USD" {
                                                    format!("${}", t.amount)
                                                } else {
                                                    format!("{} {}", t.amount, t.currency)
                                                };
                                                let from = t.from.clone();
                                                view! {
                                                    <div class="tip-item">
                                                        <span class="tip-from-name">{from}</span>
                                                        <span class="tip-amt">{amt_str}</span>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        }.into_any()
                                    }
                                }}
                            </div>

                            <div class="share-row" style="margin-top:16px">
                                <button class="btn-outline btn" on:click={
                                    let url = web_sys::window()
                                        .and_then(|w| w.location().href().ok())
                                        .unwrap_or_default();
                                    move |_| {
                                        copy_to_clipboard(&url);
                                        show_toast("Link copied!");
                                    }
                                }>"🔗 Copy link"</button>
                            </div>
                        </div>
                    }.into_any()
                }
            }}

            <div id="toast" class="toast"></div>
            <nav class="bot-nav">
                <button class="nav-item" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"⊡"</span>"Feed"
                </button>
                <button class="nav-item" on:click=move |_| nav_to("/create")>
                    <span class="nav-icon">"◈"</span>"Post"
                </button>
                <button class="nav-item" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"◎"</span>"Profile"
                </button>
            </nav>
        </div>
    }
}
