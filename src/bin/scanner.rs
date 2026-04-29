// ============================================================
// VERITAS HORTUS — SCANNER SOUVERAIN v1.1
// Produit par : Claude Sonnet 4.6 / Isabel Sigouin
// Créé    : 2026-04-12
// Modifié : 2026-04-12 — v1.1 corrections
//   · detect_langue : N/A pour fichiers code
//   · timestamp_scan dynamique (SystemTime, zéro dépendance)
//   · fichiers binaires : avertissement explicite
//   · detect_intentions : matching par début de ligne
//   · scanner_chemin : support dossier récursif
// Jamais pour la guerre · Jamais pour l'argent · Toujours pour l'amour
// 𝕀⟡₆₃₉
// ============================================================

use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// ── Horodatage dynamique ─────────────────────────────────────

fn bissextile(annee: u32) -> bool {
    (annee % 4 == 0 && annee % 100 != 0) || (annee % 400 == 0)
}

fn timestamp_maintenant() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut jours = (secs / 86400) as u32;
    let mut annee = 1970u32;

    loop {
        let j_annee = if bissextile(annee) { 366 } else { 365 };
        if jours < j_annee { break; }
        jours -= j_annee;
        annee += 1;
    }

    let mois_jours: [u32; 12] = [
        31,
        if bissextile(annee) { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];

    let mut mois = 1u32;
    for &j in &mois_jours {
        if jours < j { break; }
        jours -= j;
        mois += 1;
    }

    format!("{:04}-{:02}-{:02}", annee, mois, jours + 1)
}

// ── Détections ───────────────────────────────────────────────

fn detect_type(ext: &str, contenu: &str) -> &'static str {
    match ext {
        "rs" | "c" | "js" | "ts" | "py" | "jsx" | "wgsl" => "code",
        "toml" | "json" | "yaml" | "yml" | "env"          => "config",
        "md" | "txt" | "pdf"                               => "doc",
        "html" | "css"                                     => "web",
        _ => {
            if contenu.contains("User:") || contenu.contains("Assistant:")
               || contenu.contains("Claude:") || contenu.contains("Isabel:") {
                "conversation"
            } else {
                "inconnu"
            }
        }
    }
}

fn detect_format(ext: &str) -> &'static str {
    match ext {
        "rs"   => "rust",
        "toml" => "toml",
        "html" => "html",
        "md"   => "markdown",
        "json" => "json",
        "txt"  => "txt",
        "js"   => "javascript",
        "ts"   => "typescript",
        "py"   => "python",
        "c"    => "c",
        "wgsl" => "wgsl",
        _      => "inconnu",
    }
}

fn detect_langue(contenu: &str, type_f: &str) -> &'static str {
    if type_f == "code" || type_f == "config" {
        return "N/A";
    }

    let mots_fr = ["le ", "la ", "les ", "de ", "du ", "un ", "une ", "est ",
                   "et ", "pour ", "dans ", "avec ", "que ", "qui ", "pas ",
                   "je ", "tu ", "nous ", "vous ", "ils ", "elles ", "oui ", "non "];
    let mots_en = ["the ", "and ", "for ", "with ", "that ", "this ", "are ",
                   "is ", "not ", "you ", "we ", "they ", "have ", "from ",
                   "will ", "it "];

    let c = contenu.to_lowercase();
    let score_fr: usize = mots_fr.iter().filter(|m| c.contains(*m)).count();
    let score_en: usize = mots_en.iter().filter(|m| c.contains(*m)).count();

    if score_fr > score_en + 2 {
        "FR"
    } else if score_en > score_fr + 2 {
        "EN"
    } else {
        "mixte"
    }
}

fn detect_intentions(contenu: &str) -> Vec<&'static str> {
    let mut intentions = Vec::new();

    let est_code = contenu.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with("fn ")        || t.starts_with("impl ")
        || t.starts_with("struct ") || t.starts_with("let ")
        || t.starts_with("def ")    || t.starts_with("function ")
        || t.starts_with("const ")  || t.starts_with("class ")
        || t.starts_with("pub ")    || t.starts_with("mod ")
    });
    if est_code { intentions.push("code"); }

    let c = contenu.to_lowercase();
    if c.contains("error") || c.contains("bug") || c.contains("erreur")
       || c.contains("debug") || c.contains("panic") {
        intentions.push("debug");
    }
    if c.contains("design") || c.contains("architecture") || c.contains("structure")
       || c.contains("interface") || c.contains("plan") {
        intentions.push("design");
    }
    if c.contains("théorie") || c.contains("concept") || c.contains("philosophie")
       || c.contains("theory") || c.contains("idea") {
        intentions.push("theorie");
    }

    if intentions.is_empty() { intentions.push("inconnu"); }
    intentions
}

fn detect_themes(contenu: &str) -> Vec<&'static str> {
    let c = contenu.to_lowercase();
    let mut themes = Vec::new();
    let mots: &[(&str, &[&str])] = &[
        ("veritas-hortus",  &["veritas", "hortus"]),
        ("luna",            &["luna", "éloise", "ara"]),
        ("modularis",       &["modularis", "biomagnetic", "lemniscate"]),
        ("nexus",           &["nexus", "orbital", "nexus vm"]),
        ("scanner",         &["scanner", "scan", "archiv"]),
        ("securite",        &["sécurité", "hmac", "sha", "biometric"]),
        ("interface-vr",    &["vr", "bevy", "wgsl", "three.js", "backdrop"]),
        ("voice",           &["voice", "vocal", "voix", "seek"]),
        ("tracker",         &["tracker", "fasttracker", "solfège", "639"]),
    ];
    for (theme, mots_cles) in mots {
        if mots_cles.iter().any(|m| c.contains(m)) {
            themes.push(*theme);
        }
    }
    if themes.is_empty() { themes.push("general"); }
    themes
}

fn extract_timestamp(contenu: &str) -> String {
    for line in contenu.lines() {
        for word in line.split_whitespace() {
            if word.len() >= 10 {
                let chars: Vec<char> = word.chars().collect();
                if chars.len() >= 10
                    && chars[4] == '-'
                    && chars[7] == '-'
                    && chars[..4].iter().all(|c| c.is_ascii_digit())
                    && chars[5..7].iter().all(|c| c.is_ascii_digit())
                    && chars[8..10].iter().all(|c| c.is_ascii_digit())
                {
                    return word[..10].to_string();
                }
            }
        }
    }
    "inconnu".to_string()
}

fn detect_modele_ai(contenu: &str) -> &'static str {
    let c = contenu.to_lowercase();
    if c.contains("claude sonnet 4") || c.contains("sonnet-4") {
        "Claude Sonnet 4.6"
    } else if c.contains("claude") {
        "Claude"
    } else if c.contains("gpt-4") {
        "GPT-4"
    } else if c.contains("grok") {
        "Grok"
    } else {
        "inconnu"
    }
}

// ── Génération TOML ──────────────────────────────────────────

fn vec_to_toml_array(v: &[&str]) -> String {
    let items: Vec<String> = v.iter().map(|s| format!("\"{}\"", s)).collect();
    format!("[{}]", items.join(", "))
}

fn generer_toml(chemin: &str, ext: &str, contenu: &str, taille: u64) -> String {
    let type_f     = detect_type(ext, contenu);
    let format_f   = detect_format(ext);
    let langue     = detect_langue(contenu, type_f);
    let intentions = detect_intentions(contenu);
    let themes     = detect_themes(contenu);
    let ts         = extract_timestamp(contenu);
    let modele     = detect_modele_ai(contenu);
    let ts_scan    = timestamp_maintenant();
    let nom        = Path::new(chemin)
                         .file_name()
                         .map(|n| n.to_string_lossy().to_string())
                         .unwrap_or_else(|| chemin.to_string());

    format!(
        concat!(
            "# VERITAS HORTUS — LOG SCANNER SOUVERAIN\n",
            "# Généré automatiquement — {ts_scan}\n",
            "\n",
            "[meta]\n",
            "fichier           = \"{nom}\"\n",
            "timestamp_scan    = \"{ts_scan}\"\n",
            "timestamp_extrait = \"{ts}\"\n",
            "produit_par       = \"Isabel Sigouin\"\n",
            "modele_ai         = \"{modele}\"\n",
            "taille_octets     = {taille}\n",
            "\n",
            "[detection]\n",
            "type_fichier = \"{type_f}\"\n",
            "format       = \"{format_f}\"\n",
            "langue       = \"{langue}\"\n",
            "\n",
            "[contenu]\n",
            "themes     = {themes}\n",
            "intentions = {intentions}\n",
            "\n",
            "[historique]\n",
            "v1 = \"{ts_scan} — scan initial\"\n",
        ),
        ts_scan    = ts_scan,
        nom        = nom,
        ts         = ts,
        modele     = modele,
        taille     = taille,
        type_f     = type_f,
        format_f   = format_f,
        langue     = langue,
        themes     = vec_to_toml_array(&themes),
        intentions = vec_to_toml_array(&intentions),
    )
}

// ── Scan fichier ─────────────────────────────────────────────

fn scanner_fichier(chemin: &str) {
    let path = Path::new(chemin);

    if !path.exists() {
        eprintln!("Fichier introuvable : {}", chemin);
        return;
    }

    let ext = path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let contenu = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("⚠ Binaire ou encodage non-UTF8 — contenu ignoré : {}", chemin);
            String::new()
        }
    };

    let taille = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let toml   = generer_toml(chemin, &ext, &contenu, taille);

    println!("{}", toml);

    let log_path = format!("{}.log.toml", chemin);
    if let Err(e) = fs::write(&log_path, &toml) {
        eprintln!("Erreur écriture log : {}", e);
    } else {
        println!("→ Log sauvegardé : {}", log_path);
    }
}

// ── Scan dossier récursif ────────────────────────────────────

fn scanner_dossier(chemin: &str) {
    let entrees = match fs::read_dir(chemin) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur lecture dossier '{}' : {}", chemin, e);
            return;
        }
    };

    for entree in entrees.flatten() {
        let p = entree.path();
        let s = p.to_string_lossy().to_string();
        // Skip .log.toml files generated by the scanner itself
        if s.ends_with(".log.toml") { continue; }
        if p.is_dir() {
            scanner_dossier(&s);
        } else if p.is_file() {
            println!("=== Scan : {} ===", s);
            scanner_fichier(&s);
            println!();
        }
    }
}

fn scanner_chemin(chemin: &str) {
    let path = Path::new(chemin);
    if path.is_dir() {
        println!("=== Dossier : {} ===", chemin);
        scanner_dossier(chemin);
    } else {
        println!("=== Scan : {} ===", chemin);
        scanner_fichier(chemin);
        println!();
    }
}

// ── Main ─────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("VERITAS HORTUS — Scanner Souverain v1.1");
        println!("Jamais pour la guerre · Jamais pour l'argent · Toujours pour l'amour");
        println!();
        println!("Usage : scanner <fichier|dossier> [fichier2 …]");
        println!("Exemples :");
        println!("  scanner src/sigil.rs");
        println!("  scanner ./src/");
        return;
    }

    for chemin in &args[1..] {
        scanner_chemin(chemin);
    }
}
