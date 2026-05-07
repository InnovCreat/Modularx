use leptos::prelude::*;
use crate::store;

fn nav_to(path: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(path);
    }
}

#[component]
pub fn CreatePage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let text = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let MAX: usize = 500;

    let char_count = move || text.get().len();
    let can_post = move || {
        let n = char_count();
        n >= 10 && n <= MAX && !submitting.get()
    };

    let counter_class = move || {
        let n = char_count();
        if n > MAX { "char-ctr limit" }
        else if n > 480 { "char-ctr warn" }
        else { "char-ctr" }
    };

    let submit = move |_| {
        if !can_post() { return; }
        submitting.set(true);
        let u = username.get();
        let u = if u.trim().is_empty() { "@anon".to_string() } else { u };
        let t = text.get();
        store::create_post(u.trim(), t.trim());
        nav_to("/");
    };

    view! {
        <div class="app">
            <header class="hdr">
                <button class="btn-ghost" on:click=move |_| nav_to("/")>"← Back"</button>
                <span class="tagline">"New Post"</span>
                <span style="width:68px"></span>
            </header>
            <div class="create-wrap">
                <div class="field">
                    <div class="field-lbl">"Your handle"</div>
                    <input
                        class="inp"
                        type="text"
                        placeholder="@yourname"
                        maxlength="40"
                        on:input=move |e| username.set(event_target_value(&e))
                        prop:value=username
                    />
                </div>
                <div class="field">
                    <div class="field-lbl">
                        "Your idea "
                        <span style="color:#444;font-weight:400;text-transform:none;letter-spacing:0">
                            "(max 500 chars)"
                        </span>
                    </div>
                    <textarea
                        class="textarea"
                        placeholder="Share an idea, observation, or insight…"
                        maxlength="500"
                        rows="8"
                        on:input=move |e| text.set(event_target_value(&e))
                        prop:value=text
                    ></textarea>
                    <div class=counter_class>
                        {move || format!("{} / {}", char_count(), MAX)}
                    </div>
                </div>
                <button
                    class="btn-post"
                    disabled=move || !can_post()
                    on:click=submit
                >
                    {move || if submitting.get() { "Posting…" } else { "Post ⚡" }}
                </button>
            </div>
            <nav class="bot-nav">
                <button class="nav-item" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"⊡"</span>"Feed"
                </button>
                <button class="nav-item active" on:click=move |_| nav_to("/create")>
                    <span class="nav-icon">"◈"</span>"Post"
                </button>
                <button class="nav-item" on:click=move |_| nav_to("/")>
                    <span class="nav-icon">"◎"</span>"Profile"
                </button>
            </nav>
        </div>
    }
}
