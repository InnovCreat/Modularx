// Full HTML pages served by the Rust binary. All CSS and JS is inline — no external deps.
// NOTE: raw strings use r##"..."## so "# sequences inside (href="#", content="#...") are safe.

const CSS: &str = r##"
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
html{scroll-behavior:smooth}
body{
  background:#0a0a0a;color:#f0f0f0;
  font-family:ui-sans-serif,system-ui,-apple-system,sans-serif;
  font-size:15px;line-height:1.6;min-height:100dvh;
}
a{color:#86e77c;text-decoration:none}
a:hover{text-decoration:underline}
button{cursor:pointer;font-family:inherit;font-size:inherit;border:none;background:none}

::-webkit-scrollbar{width:4px;height:4px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:#2a2a2a;border-radius:4px}

.app{max-width:640px;margin:0 auto;padding-bottom:72px}

.hdr{
  position:sticky;top:0;z-index:50;
  background:rgba(10,10,10,.92);backdrop-filter:blur(12px);
  border-bottom:1px solid #1e1e1e;
  padding:14px 16px;display:flex;align-items:center;gap:12px;
}
.logo{font-size:18px;font-weight:800;letter-spacing:-.5px;color:#86e77c}
.tagline{font-size:11px;color:#666;letter-spacing:.06em;text-transform:uppercase;flex:1;text-align:center}

.btn{
  display:inline-flex;align-items:center;justify-content:center;gap:6px;
  padding:8px 16px;border-radius:8px;font-weight:600;font-size:14px;
  transition:all .15s;white-space:nowrap;cursor:pointer;
}
.btn-primary{background:#86e77c;color:#0a0a0a;border:none}
.btn-primary:hover{background:#6dd463;transform:translateY(-1px)}
.btn-primary:active{transform:translateY(0)}
.btn-primary:disabled{opacity:.4;cursor:not-allowed;transform:none;pointer-events:none}
.btn-outline{border:1px solid #2a2a2a;color:#f0f0f0;background:none}
.btn-outline:hover{border-color:#86e77c;color:#86e77c}
.btn-ghost{color:#666;padding:6px 10px;background:none;border:none}
.btn-ghost:hover{color:#f0f0f0}

.card{
  border-bottom:1px solid #1e1e1e;padding:18px 16px;cursor:pointer;
  transition:background .12s,transform .12s;
}
.card:hover{background:#111;transform:translateY(-1px)}
.card-hdr{display:flex;align-items:center;gap:10px;margin-bottom:10px}
.avatar{
  width:36px;height:36px;border-radius:50%;background:#1a1a1a;
  border:2px solid #86e77c;display:flex;align-items:center;justify-content:center;
  font-weight:700;font-size:14px;color:#86e77c;flex-shrink:0;
}
.avatar-lg{width:46px;height:46px;font-size:18px}
.meta{display:flex;flex-direction:column;gap:1px;min-width:0}
.uname{font-weight:700;font-size:14px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.ts{font-size:12px;color:#666}
.card-body{font-size:15px;line-height:1.65;margin-bottom:14px;color:#e8e8e8}
.card-body-clamp{
  display:-webkit-box;-webkit-line-clamp:4;-webkit-box-orient:vertical;overflow:hidden;
}
.card-ftr{display:flex;align-items:center;justify-content:space-between;gap:8px;flex-wrap:wrap}

.score-wrap{display:flex;align-items:center;gap:8px;flex:1;min-width:0}
.score-bar{flex:1;height:4px;border-radius:2px;background:#1e1e1e;overflow:hidden;max-width:100px}
.score-fill{height:100%;border-radius:2px;background:#86e77c;transition:width .4s}
.score-num{font-size:13px;font-weight:700;color:#86e77c;min-width:24px}
.views{font-size:12px;color:#666}
.tip-total{font-size:12px;color:#86e77c;font-weight:600}

.btn-tip{
  border:1px solid #86e77c;color:#86e77c;background:transparent;
  border-radius:8px;padding:5px 12px;font-size:13px;font-weight:600;
  cursor:pointer;font-family:inherit;transition:all .15s;
}
.btn-tip:hover{background:#86e77c;color:#0a0a0a;box-shadow:0 0 12px rgba(134,231,124,.3)}

.section-lbl{
  font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;
  color:#666;padding:14px 16px 10px;border-bottom:1px solid #1e1e1e;
}

.empty{text-align:center;padding:60px 24px;color:#666}
.empty-icon{font-size:48px;margin-bottom:16px}

.bot-nav{
  position:fixed;bottom:0;left:0;right:0;z-index:40;
  background:rgba(10,10,10,.96);backdrop-filter:blur(12px);
  border-top:1px solid #1e1e1e;
  display:flex;align-items:center;justify-content:space-around;
  padding:10px 0;
}
.nav-item{
  display:flex;flex-direction:column;align-items:center;gap:3px;
  color:#666;font-size:11px;font-weight:500;padding:4px 20px;
  transition:color .15s;text-decoration:none;
}
.nav-item:hover,.nav-item.active{color:#86e77c}
.nav-item:hover{text-decoration:none}
.nav-icon{font-size:20px;line-height:1}

.post-wrap{padding:16px}
.post-hdr{display:flex;align-items:center;gap:12px;margin-bottom:16px}
.post-body{font-size:16px;line-height:1.75;color:#e8e8e8;margin-bottom:24px;white-space:pre-wrap;word-break:break-word}
.vote-row{display:flex;align-items:center;gap:14px;margin-bottom:24px}
.vote-btn{
  width:36px;height:36px;border-radius:8px;border:1px solid #1e1e1e;
  color:#666;font-size:18px;display:flex;align-items:center;justify-content:center;
  cursor:pointer;background:none;font-family:inherit;transition:all .15s;
}
.vote-btn:hover{border-color:#86e77c;color:#86e77c}
.score-big{font-size:28px;font-weight:800;color:#86e77c}

.tip-section{border:1px solid #1e1e1e;border-radius:12px;padding:20px;margin-bottom:24px}
.tip-lbl{font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;color:#666;margin-bottom:14px}
.tip-chips{display:flex;gap:8px;flex-wrap:wrap;margin-bottom:14px}
.chip{
  padding:7px 14px;border-radius:20px;border:1px solid #1e1e1e;
  font-size:13px;font-weight:600;color:#666;cursor:pointer;
  background:none;font-family:inherit;transition:all .15s;
}
.chip:hover,.chip.active{border-color:#86e77c;color:#86e77c;background:rgba(134,231,124,.06)}
.tip-custom{display:flex;gap:8px;margin-bottom:14px;align-items:center}
.inp{
  background:#0d0d0d;border:1px solid #1e1e1e;border-radius:8px;
  color:#f0f0f0;font-family:inherit;font-size:14px;padding:9px 12px;
  transition:border-color .15s;width:100%;
}
.inp:focus{outline:none;border-color:#86e77c}
.inp::placeholder{color:#444}
.inp-sm{max-width:120px;width:auto}
.tip-from-row{display:flex;gap:8px;align-items:center;margin-bottom:14px}
.tip-from-lbl{font-size:13px;color:#666;white-space:nowrap}
.btn-send{
  width:100%;padding:12px;background:#86e77c;color:#0a0a0a;
  border:none;border-radius:8px;font-weight:700;font-size:15px;
  cursor:pointer;font-family:inherit;transition:all .15s;
}
.btn-send:hover{background:#6dd463}
.btn-send:disabled{opacity:.4;cursor:not-allowed}

.tips-list{margin-top:16px;display:flex;flex-direction:column;gap:8px}
.tip-item{display:flex;justify-content:space-between;font-size:13px;color:#888;border-top:1px solid #1a1a1a;padding-top:8px}
.tip-item:first-child{border-top:none;padding-top:0}
.tip-from{color:#aaa}
.tip-amt{color:#86e77c;font-weight:600}

.share-row{display:flex;gap:10px;padding:0 0 16px}

.create-wrap{padding:16px}
.field-lbl{font-size:12px;color:#666;font-weight:600;letter-spacing:.06em;text-transform:uppercase;margin-bottom:6px}
.field{margin-bottom:18px}
.textarea{
  width:100%;min-height:220px;background:#0d0d0d;border:1px solid #1e1e1e;
  border-radius:12px;color:#f0f0f0;font-family:inherit;font-size:16px;
  line-height:1.7;padding:16px;resize:none;overflow:hidden;
  transition:border-color .15s;
}
.textarea:focus{outline:none;border-color:#86e77c}
.textarea::placeholder{color:#333}
.char-ctr{font-size:12px;color:#555;text-align:right;margin-top:6px;transition:color .15s}
.char-ctr.warn{color:#e8b96a}
.char-ctr.limit{color:#e77c7c}
.btn-post{width:100%;padding:14px;background:#86e77c;color:#0a0a0a;border:none;border-radius:10px;font-weight:700;font-size:16px;cursor:pointer;font-family:inherit;transition:all .15s;margin-top:4px}
.btn-post:hover{background:#6dd463}
.btn-post:disabled{opacity:.4;cursor:not-allowed}

.toast{
  position:fixed;bottom:84px;left:50%;transform:translateX(-50%) translateY(16px);
  background:#111;border:1px solid #86e77c;color:#f0f0f0;
  font-size:14px;font-weight:500;padding:10px 20px;border-radius:24px;
  z-index:1000;opacity:0;transition:opacity .3s,transform .3s;
  white-space:nowrap;pointer-events:none;
}
.toast.show{opacity:1;transform:translateX(-50%) translateY(0)}

.loading{padding:40px;text-align:center;color:#444;font-size:14px}
.error{padding:20px 16px;color:#e77c7c;font-size:14px}

@media(max-width:480px){
  .tagline{display:none}
  .tip-chips{gap:6px}
  .chip{padding:6px 11px}
}
@keyframes fadeUp{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
.card{animation:fadeUp .25s ease forwards}
"##;

fn page(title: &str, body: &str, extra_js: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="theme-color" content="#0a0a0a">
<title>{title}</title>
<style>{css}</style>
</head>
<body>
{body}
<div id="toast" class="toast"></div>
<script>
function showToast(msg,ms){{
  var t=document.getElementById('toast');
  t.textContent=msg;t.classList.add('show');
  clearTimeout(t._tid);
  t._tid=setTimeout(function(){{t.classList.remove('show')}},ms||3000);
}}
function esc(s){{
  return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}}
function timeAgo(ts){{
  var d=Date.now()-ts,m=Math.floor(d/60000),h=Math.floor(d/3600000),dy=Math.floor(d/86400000);
  if(m<1)return'just now';if(m<60)return m+'m ago';if(h<24)return h+'h ago';return dy+'d ago';
}}
{extra_js}
</script>
</body>
</html>"##,
        title = title,
        css = CSS,
        body = body,
        extra_js = extra_js,
    )
}

// ── Home feed ─────────────────────────────────────────────────────────────────

pub fn index_page() -> String {
    let body = r##"
<div class="app">
  <header class="hdr">
    <span class="logo">EVK</span>
    <span class="tagline">merit-based ideas</span>
    <a href="/create" class="btn btn-primary">&#xFF0B; Post</a>
  </header>
  <div class="section-lbl">Latest Posts</div>
  <main id="feed"><div class="loading">Loading posts&#8230;</div></main>
  <nav class="bot-nav">
    <a href="/" class="nav-item active"><span class="nav-icon">&#9645;</span>Feed</a>
    <a href="/create" class="nav-item"><span class="nav-icon">&#9671;</span>Post</a>
    <a href="/" class="nav-item"><span class="nav-icon">&#9711;</span>Profile</a>
  </nav>
</div>"##;

    let js = r##"
(function(){
  var feed=document.getElementById('feed');

  function renderCard(p,maxScore){
    var pct=maxScore>0?Math.max(0,(p.score/maxScore)*100):0;
    var usd=p.tips.filter(function(t){return t.currency==='USD';}).reduce(function(s,t){return s+t.amount;},0);
    var preview=p.text.length>260?p.text.slice(0,260)+'…':p.text;
    return '<div class="card" onclick="location.href=\'/post?id='+esc(p.id)+'\'">'
      +'<div class="card-hdr">'
        +'<div class="avatar">'+esc(p.avatar)+'</div>'
        +'<div class="meta">'
          +'<div class="uname">'+esc(p.username)+'</div>'
          +'<div class="ts">'+timeAgo(p.timestamp)+'</div>'
        +'</div>'
      +'</div>'
      +'<div class="card-body card-body-clamp">'+esc(preview)+'</div>'
      +'<div class="card-ftr">'
        +'<div class="score-wrap">'
          +'<div class="score-bar"><div class="score-fill" style="width:'+pct.toFixed(0)+'%"></div></div>'
          +'<span class="score-num">'+p.score+'</span>'
          +'<span class="views">&#128065; '+p.views+'</span>'
        +'</div>'
        +(usd>0?'<span class="tip-total">$'+usd+' tipped</span>':'')
        +'<button class="btn-tip" onclick="event.stopPropagation();location.href=\'/post?id='+esc(p.id)+'\'">Tip &#9889;</button>'
      +'</div>'
    +'</div>';
  }

  fetch('/api/posts')
    .then(function(r){return r.json();})
    .then(function(posts){
      if(!posts.length){
        feed.innerHTML='<div class="empty"><div class="empty-icon">&#9711;</div><p>No posts yet.<br>Be the first to share an idea.</p></div>';
        return;
      }
      var maxScore=Math.max.apply(null,posts.map(function(p){return p.score;}));
      if(maxScore<1)maxScore=1;
      feed.innerHTML=posts.map(function(p){return renderCard(p,maxScore);}).join('');
    })
    .catch(function(){
      feed.innerHTML='<div class="error">Could not load posts.</div>';
    });
})();
"##;

    page("Eko Vision Vault", body, js)
}

// ── Create post ───────────────────────────────────────────────────────────────

pub fn create_page() -> String {
    let body = r##"
<div class="app">
  <header class="hdr">
    <a href="/" class="btn-ghost" title="Back">&#8592; Back</a>
    <span class="tagline">New Post</span>
    <span style="width:68px"></span>
  </header>
  <div class="create-wrap">
    <form id="post-form" autocomplete="off">
      <div class="field">
        <div class="field-lbl">Your handle</div>
        <input id="username" class="inp" type="text" placeholder="@yourname" maxlength="40">
      </div>
      <div class="field">
        <div class="field-lbl">Your idea <span style="color:#444;font-weight:400;text-transform:none;letter-spacing:0">(max 500 chars)</span></div>
        <textarea id="text" class="textarea" placeholder="Share an idea, observation, or insight&#8230;" maxlength="500"></textarea>
        <div class="char-ctr" id="ctr">0 / 500</div>
      </div>
      <button type="submit" id="post-btn" class="btn-post" disabled>Post &#9889;</button>
    </form>
  </div>
  <nav class="bot-nav">
    <a href="/" class="nav-item"><span class="nav-icon">&#9645;</span>Feed</a>
    <a href="/create" class="nav-item active"><span class="nav-icon">&#9671;</span>Post</a>
    <a href="/" class="nav-item"><span class="nav-icon">&#9711;</span>Profile</a>
  </nav>
</div>"##;

    let js = r##"
(function(){
  var ta=document.getElementById('text');
  var ctr=document.getElementById('ctr');
  var btn=document.getElementById('post-btn');
  var MAX=500;

  ta.addEventListener('input',function(){
    var n=ta.value.length;
    ctr.textContent=n+' / '+MAX;
    ctr.className='char-ctr'+(n>480?(n>=MAX?' limit':' warn'):'');
    btn.disabled=n<10||n>MAX;
    ta.style.height='auto';
    ta.style.height=ta.scrollHeight+'px';
  });

  document.getElementById('post-form').addEventListener('submit',function(e){
    e.preventDefault();
    var user=document.getElementById('username').value.trim()||'@anon';
    var text=ta.value.trim();
    btn.disabled=true;btn.textContent='Posting…';
    fetch('/api/posts',{
      method:'POST',
      headers:{'Content-Type':'application/json'},
      body:JSON.stringify({username:user,text:text})
    }).then(function(r){
      if(r.ok){location.href='/';}
      else{btn.textContent='Error — try again';btn.disabled=false;}
    }).catch(function(){
      btn.textContent='Error — try again';btn.disabled=false;
    });
  });
})();
"##;

    page("Create Post — Eko Vision Vault", body, js)
}

// ── Post view + tipping ───────────────────────────────────────────────────────

pub fn post_page(id: &str) -> String {
    let safe_id = id.replace('"', "\\\"").replace('\\', "\\\\");

    let body = r##"
<div class="app">
  <header class="hdr">
    <a href="/" class="btn-ghost" title="Back">&#8592; Back</a>
    <span class="tagline">Post</span>
    <span style="width:60px"></span>
  </header>
  <div id="content"><div class="loading">Loading&#8230;</div></div>

  <div id="tip-box" style="display:none">
    <div class="post-wrap" style="padding-top:0">
      <div class="tip-section">
        <div class="tip-lbl">&#9889; Send a tip</div>
        <div class="tip-chips" id="chips">
          <button class="chip active" data-amt="1" data-cur="USD">&#9749; $1</button>
          <button class="chip" data-amt="5" data-cur="USD">&#127919; $5</button>
          <button class="chip" data-amt="20" data-cur="USD">&#128142; $20</button>
          <button class="chip" data-amt="10" data-cur="Doge">&#128415; 10 Doge</button>
        </div>
        <div class="tip-custom">
          <span style="color:#666;font-size:13px;white-space:nowrap">Custom $</span>
          <input id="custom-amt" class="inp inp-sm" type="number" min="1" placeholder="0">
        </div>
        <div class="tip-from-row">
          <span class="tip-from-lbl">From</span>
          <input id="tip-from" class="inp" type="text" placeholder="@yourname or anon" maxlength="40">
        </div>
        <button id="send-tip" class="btn-send">Send Tip &#9889;</button>
      </div>
      <div id="tips-received" class="tips-list"></div>
      <div class="share-row">
        <button class="btn-outline btn" onclick="share()">&#128279; Copy link</button>
      </div>
    </div>
  </div>

  <nav class="bot-nav">
    <a href="/" class="nav-item"><span class="nav-icon">&#9645;</span>Feed</a>
    <a href="/create" class="nav-item"><span class="nav-icon">&#9671;</span>Post</a>
    <a href="/" class="nav-item"><span class="nav-icon">&#9711;</span>Profile</a>
  </nav>
</div>"##;

    let js = format!(
        r##"
(function(){{
  var POST_ID="{safe_id}";
  var selAmt=1,selCur='USD';

  fetch('/api/views',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify({{id:POST_ID}})}});

  function renderTips(tips){{
    var box=document.getElementById('tips-received');
    var usd=tips.filter(function(t){{return t.currency==='USD';}}).reduce(function(s,t){{return s+t.amount;}},0);
    if(!tips.length){{
      box.innerHTML='<div style="color:#444;font-size:13px;padding:4px 0">No tips yet — be the first!</div>';
      return;
    }}
    box.innerHTML='<div style="font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;color:#555;margin-bottom:10px">Tips received'+(usd>0?' · $'+usd+' total':'')+'</div>'
      +tips.map(function(t){{
        var amt=t.currency==='USD'?'$'+t.amount:t.amount+' '+t.currency;
        return '<div class="tip-item"><span class="tip-from">'+esc(t.from)+'</span><span class="tip-amt">'+esc(amt)+'</span></div>';
      }}).join('');
  }}

  function renderPost(p){{
    document.title=p.username+' — Eko Vision Vault';
    document.getElementById('content').innerHTML=
      '<div class="post-wrap">'
        +'<div class="post-hdr">'
          +'<div class="avatar avatar-lg">'+esc(p.avatar)+'</div>'
          +'<div class="meta">'
            +'<div class="uname">'+esc(p.username)+'</div>'
            +'<div class="ts">'+timeAgo(p.timestamp)+' · 👁 '+p.views+'</div>'
          +'</div>'
        +'</div>'
        +'<div class="post-body">'+esc(p.text)+'</div>'
        +'<div class="vote-row">'
          +'<button class="vote-btn" id="upvote" onclick="vote(1)" title="Upvote">▲</button>'
          +'<span class="score-big" id="score">'+p.score+'</span>'
          +'<button class="vote-btn" id="downvote" onclick="vote(-1)" title="Downvote">▼</button>'
        +'</div>'
      +'</div>';
    document.getElementById('tip-box').style.display='block';
    renderTips(p.tips);
  }}

  fetch('/api/posts?id='+encodeURIComponent(POST_ID))
    .then(function(r){{if(!r.ok){{location.href='/';return Promise.reject();}}return r.json();}})
    .then(renderPost)
    .catch(function(){{document.getElementById('content').innerHTML='<div class="error">Post not found.</div>';}});

  window.vote=function(dir){{
    fetch('/api/vote',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify({{id:POST_ID,dir:dir}})}})
      .then(function(r){{return r.json();}})
      .then(function(d){{document.getElementById('score').textContent=d.score;}});
  }};

  document.getElementById('chips').addEventListener('click',function(e){{
    var chip=e.target.closest('.chip');if(!chip)return;
    document.querySelectorAll('.chip').forEach(function(c){{c.classList.remove('active');}});
    chip.classList.add('active');
    selAmt=parseInt(chip.dataset.amt);selCur=chip.dataset.cur;
    document.getElementById('custom-amt').value='';
  }});
  document.getElementById('custom-amt').addEventListener('input',function(e){{
    document.querySelectorAll('.chip').forEach(function(c){{c.classList.remove('active');}});
    selAmt=parseInt(e.target.value)||0;selCur='USD';
  }});

  document.getElementById('send-tip').addEventListener('click',function(){{
    if(!selAmt||selAmt<=0){{showToast('Pick a tip amount first');return;}}
    var from=document.getElementById('tip-from').value.trim()||'anon';
    var btn=document.getElementById('send-tip');
    btn.disabled=true;btn.textContent='Sending…';
    fetch('/api/tip',{{method:'POST',headers:{{'Content-Type':'application/json'}},body:JSON.stringify({{id:POST_ID,amount:selAmt,currency:selCur,from:from}})}})
      .then(function(r){{return r.ok?r.json():Promise.reject();}})
      .then(function(){{
        showToast('Tip sent! ⚡');
        btn.textContent='Send Tip ⚡';btn.disabled=false;
        return fetch('/api/posts?id='+encodeURIComponent(POST_ID)).then(function(r){{return r.json();}});
      }})
      .then(function(p){{renderTips(p.tips);}})
      .catch(function(){{btn.textContent='Error — try again';btn.disabled=false;}});
  }});

  window.share=function(){{
    if(navigator.clipboard){{
      navigator.clipboard.writeText(location.href).then(function(){{showToast('Link copied!');}});
    }} else {{
      showToast('Copy: '+location.href,5000);
    }}
  }};
}})();
"##,
        safe_id = safe_id
    );

    page("Post — Eko Vision Vault", body, &js)
}
