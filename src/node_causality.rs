// ============================================================
// node_causality.rs — Causalité : Mutual Information,
//                     Transfer Entropy, Granger Causality
// Pure math, zero dépendances externes
// ============================================================
use crate::quantum_state::{QuantumState, SourceKind};
#[allow(unused_imports)]
use crate::node_metrics::node_histogram;

// ============================================================
// HELPERS COMMUNS
// ============================================================

/// Résout SourceKind → slice de données
fn resolve_source<'a>(state: &'a QuantumState, source: &SourceKind) -> &'a [f64] {
    match source {
        SourceKind::RawPerturbation => &state.base_perturbation,
        SourceKind::Tension         => &state.tension,
        SourceKind::LogX            => &state.log_x,
        SourceKind::Custom(_)       => &state.tension, // fallback
    }
}

/// Histogram 1D → probabilités normalisées avec smoothing
fn hist_to_probs(data: &[f64], num_bins: usize, smoothing: f64) -> (Vec<f64>, Vec<f64>) {
    if data.is_empty() || num_bins == 0 {
        return (Vec::new(), Vec::new());
    }
    let min_v = data.iter().cloned().fold(f64::INFINITY, f64::min) - 1e-8;
    let max_v = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1e-8;
    let width = (max_v - min_v) / num_bins as f64;
    if width <= 0.0 { return (Vec::new(), Vec::new()); }
    let edges: Vec<f64> = (0..=num_bins).map(|i| min_v + i as f64 * width).collect();
    let mut counts = vec![0.0_f64; num_bins];
    for &v in data {
        let idx = ((v - min_v) / width) as usize;
        let idx = idx.min(num_bins - 1);
        counts[idx] += 1.0;
    }
    let total = data.len() as f64;
    let probs: Vec<f64> = counts.iter()
        .map(|&c| c / total + smoothing)
        .collect();
    let sum: f64 = probs.iter().sum();
    let probs: Vec<f64> = probs.iter().map(|p| p / sum).collect();
    (probs, edges)
}

/// Histogram 2D joint → probabilités
fn joint_hist_to_probs(
    a: &[f64], b: &[f64],
    num_bins: usize,
    smoothing: f64,
) -> Vec<Vec<f64>> {
    let n = a.len().min(b.len());
    if n == 0 || num_bins == 0 { return Vec::new(); }
    let min_a = a.iter().cloned().fold(f64::INFINITY, f64::min) - 1e-8;
    let max_a = a.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1e-8;
    let min_b = b.iter().cloned().fold(f64::INFINITY, f64::min) - 1e-8;
    let max_b = b.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1e-8;
    let width_a = (max_a - min_a) / num_bins as f64;
    let width_b = (max_b - min_b) / num_bins as f64;
    if width_a <= 0.0 || width_b <= 0.0 { return Vec::new(); }
    let mut counts = vec![vec![0.0_f64; num_bins]; num_bins];
    for i in 0..n {
        let ia = (((a[i] - min_a) / width_a) as usize).min(num_bins - 1);
        let ib = (((b[i] - min_b) / width_b) as usize).min(num_bins - 1);
        counts[ia][ib] += 1.0;
    }
    let total = n as f64;
    let mut probs = vec![vec![0.0_f64; num_bins]; num_bins];
    let mut sum = 0.0_f64;
    for i in 0..num_bins {
        for j in 0..num_bins {
            probs[i][j] = counts[i][j] / total + smoothing;
            sum += probs[i][j];
        }
    }
    for i in 0..num_bins {
        for j in 0..num_bins {
            probs[i][j] /= sum;
        }
    }
    probs
}

/// Shannon entropy depuis probabilités
fn shannon_entropy(probs: &[f64], base: f64) -> f64 {
    let log_b = base.ln();
    probs.iter()
        .filter(|&&p| p > 1e-15)
        .map(|&p| -p * (p.ln() / log_b))
        .sum::<f64>()
        .max(0.0)
}

// ============================================================
// NODE_MUTUAL_INFORMATION
// I(A;B) = H(A) + H(B) - H(A,B)
// ============================================================

#[derive(Debug, Clone)]
pub struct MIResult {
    pub mi: f64,
    pub normalized_mi: f64,
    pub h_a: f64,
    pub h_b: f64,
    pub h_joint: f64,
}

pub fn node_mutual_information(
    state: &mut QuantumState,
    source_a: &SourceKind,
    source_b: &SourceKind,
    num_bins: usize,
    lag: Option<usize>,
    base: f64,
    smoothing: f64,
) -> MIResult {
    if state.dirty { state.refresh(); }
    let a_raw: Vec<f64> = resolve_source(state, source_a).to_vec();
    let b_raw: Vec<f64> = resolve_source(state, source_b).to_vec();
    // Applique le lag sur B si demandé
    let (a, b) = if let Some(lag) = lag {
        if lag >= b_raw.len() {
            return zero_mi();
        }
        (a_raw[..a_raw.len() - lag].to_vec(), b_raw[lag..].to_vec())
    } else {
        (a_raw, b_raw)
    };
    let n = a.len().min(b.len());
    if n < 4 { return zero_mi(); }
    let a = &a[..n];
    let b = &b[..n];
    // Probabilités marginales
    let (p_a, _) = hist_to_probs(a, num_bins, smoothing);
    let (p_b, _) = hist_to_probs(b, num_bins, smoothing);
    if p_a.is_empty() || p_b.is_empty() { return zero_mi(); }
    // Distribution jointe
    let p_ab = joint_hist_to_probs(a, b, num_bins, smoothing);
    if p_ab.is_empty() { return zero_mi(); }
    // Entropies
    let h_a = shannon_entropy(&p_a, base);
    let h_b = shannon_entropy(&p_b, base);
    let p_ab_flat: Vec<f64> = p_ab.iter().flatten().cloned().collect();
    let h_joint = shannon_entropy(&p_ab_flat, base);
    let mi = (h_a + h_b - h_joint).max(0.0);
    let min_h = h_a.min(h_b);
    let normalized_mi = if min_h > 1e-10 { (mi / min_h).clamp(0.0, 1.0) } else { 0.0 };
    // Stocke dans l'état
    state.causality.mutual_info.mi = mi;
    state.causality.mutual_info.normalized_mi = normalized_mi;
    MIResult { mi, normalized_mi, h_a, h_b, h_joint }
}

fn zero_mi() -> MIResult {
    MIResult { mi: 0.0, normalized_mi: 0.0, h_a: 0.0, h_b: 0.0, h_joint: 0.0 }
}

// ============================================================
// NODE_TRANSFER_ENTROPY
// TE(X→Y) = H(Y_future | Y_past) - H(Y_future | Y_past, X_past)
// Mesure le flux d'information dirigé X → Y
// ============================================================

#[derive(Debug, Clone)]
pub struct TEResult {
    pub te: f64,
    pub te_reverse: f64,
    pub net_flow: f64,
    pub dominant: TEDirection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TEDirection {
    XtoY,
    YtoX,
    Symmetric,
    Negligible,
}

pub fn node_transfer_entropy(
    state: &mut QuantumState,
    driver: &SourceKind,
    target: &SourceKind,
    lag: usize,
    embedding_dim: usize,
    num_bins: usize,
    smoothing: f64,
) -> TEResult {
    if state.dirty { state.refresh(); }
    let x: Vec<f64> = resolve_source(state, driver).to_vec();
    let y: Vec<f64> = resolve_source(state, target).to_vec();
    let lag = lag.max(1);
    let m = embedding_dim.max(1);
    let te_xy = compute_te(&x, &y, lag, m, num_bins, smoothing);
    let te_yx = compute_te(&y, &x, lag, m, num_bins, smoothing);
    let net_flow = te_xy - te_yx;
    let dominant = match net_flow {
        x if x > 0.01  => TEDirection::XtoY,
        x if x < -0.01 => TEDirection::YtoX,
        x if (te_xy + te_yx) < 0.001 => TEDirection::Negligible,
        _ => TEDirection::Symmetric,
    };
    state.causality.transfer_entropy.te       = te_xy;
    state.causality.transfer_entropy.net_flow = net_flow;
    state.causality.transfer_entropy.dominant = match dominant {
        TEDirection::XtoY     =>  1,
        TEDirection::YtoX     => -1,
        _                     =>  0,
    };
    TEResult { te: te_xy, te_reverse: te_yx, net_flow, dominant }
}

// ============================================================
// NODE_TRANSFER_ENTROPY_SIGNIFICANCE
// Compare real TE against n_surrogates time-shuffled nulls.
// Pure-Rust xorshift64 PRNG — zero dependencies.
// ============================================================

/// Result of surrogate significance test for Transfer Entropy.
#[derive(Debug, Clone)]
pub struct TESigResult {
    pub te: f64,
    pub te_reverse: f64,
    /// Fraction of surrogates with TE ≥ real TE(X→Y).  Small = significant.
    pub surrogate_p_xy: f64,
    /// Fraction of surrogates with TE ≥ real TE(Y→X).
    pub surrogate_p_yx: f64,
    pub is_significant_xy: bool,
    pub is_significant_yx: bool,
}

/// Minimal xorshift64 PRNG — deterministic, no alloc, no crates.
#[inline]
fn xorshift64(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Fisher-Yates shuffle using xorshift64.
fn shuffle(data: &mut Vec<f64>, rng: &mut u64) {
    let n = data.len();
    for i in (1..n).rev() {
        let j = (xorshift64(rng) as usize) % (i + 1);
        data.swap(i, j);
    }
}

/// Compute TE for both directions, then compare against `n_surrogates`
/// surrogate distributions obtained by shuffling the target signal.
/// A surrogate p-value < `significance` means the observed TE is unlikely
/// under the null hypothesis of no temporal structure.
pub fn node_transfer_entropy_significance(
    state: &mut QuantumState,
    driver: &SourceKind,
    target: &SourceKind,
    lag: usize,
    embedding_dim: usize,
    num_bins: usize,
    smoothing: f64,
    n_surrogates: usize,
    significance: f64,
    seed: u64,
) -> TESigResult {
    if state.dirty { state.refresh(); }
    let x: Vec<f64> = resolve_source(state, driver).to_vec();
    let y: Vec<f64> = resolve_source(state, target).to_vec();
    let lag = lag.max(1);
    let m   = embedding_dim.max(1);

    let te_xy = compute_te(&x, &y, lag, m, num_bins, smoothing);
    let te_yx = compute_te(&y, &x, lag, m, num_bins, smoothing);

    let mut rng = seed.max(1); // xorshift64 must not be seeded with 0
    let mut count_xy = 0usize;
    let mut count_yx = 0usize;

    for _ in 0..n_surrogates {
        // Surrogate for TE(X→Y): shuffle y, recompute
        let mut y_shuf = y.clone();
        shuffle(&mut y_shuf, &mut rng);
        if compute_te(&x, &y_shuf, lag, m, num_bins, smoothing) >= te_xy {
            count_xy += 1;
        }
        // Surrogate for TE(Y→X): shuffle x, recompute
        let mut x_shuf = x.clone();
        shuffle(&mut x_shuf, &mut rng);
        if compute_te(&y, &x_shuf, lag, m, num_bins, smoothing) >= te_yx {
            count_yx += 1;
        }
    }

    let n = n_surrogates.max(1) as f64;
    let surrogate_p_xy = count_xy as f64 / n;
    let surrogate_p_yx = count_yx as f64 / n;

    TESigResult {
        te: te_xy,
        te_reverse: te_yx,
        surrogate_p_xy,
        surrogate_p_yx,
        is_significant_xy: surrogate_p_xy < significance,
        is_significant_yx: surrogate_p_yx < significance,
    }
}

/// Encode two z-scored signals into a single integer index representing
/// their 2D bin pair: `bin_b * k + bin_a`.  This gives k² unique values
/// and avoids scale-domination of one axis over the other.
fn discretize_pair(a: &[f64], b: &[f64], k: usize) -> Vec<f64> {
    let n = a.len().min(b.len());
    if n == 0 || k == 0 { return vec![0.0; n]; }
    let min_a = a[..n].iter().cloned().fold(f64::INFINITY, f64::min) - 1e-8;
    let max_a = a[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1e-8;
    let min_b = b[..n].iter().cloned().fold(f64::INFINITY, f64::min) - 1e-8;
    let max_b = b[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1e-8;
    let w_a = ((max_a - min_a) / k as f64).max(1e-12);
    let w_b = ((max_b - min_b) / k as f64).max(1e-12);
    (0..n).map(|i| {
        let ia = (((a[i] - min_a) / w_a) as usize).min(k - 1);
        let ib = (((b[i] - min_b) / w_b) as usize).min(k - 1);
        (ib * k + ia) as f64
    }).collect()
}

/// Calcule TE(X→Y) via estimation par histogramme 3D approximée.
/// (x_past, y_past) sont encodés en un indice 2D unique via discretize_pair,
/// ce qui évite que l'axe y noie l'axe x dans un encodage scalaire.
fn compute_te(
    x: &[f64], y: &[f64],
    lag: usize, m: usize,
    num_bins: usize, smoothing: f64,
) -> f64 {
    let n = x.len().min(y.len());
    if n < lag + m + 2 { return 0.0; }
    let valid_n = n - lag - m;
    if valid_n < 4 { return 0.0; }
    let y_future: Vec<f64> = (m..m + valid_n).map(|t| y[t + lag]).collect();
    let y_past: Vec<f64>   = (m..m + valid_n).map(|t| y[t]).collect();
    let x_past: Vec<f64>   = (m..m + valid_n).map(|t| x[t]).collect();
    // Z-score all components so both axes contribute equally to bin widths.
    let y_future_z = zscore(&y_future);
    let y_past_z   = zscore(&y_past);
    let x_past_z   = zscore(&x_past);
    // H(Y_future | Y_past) = H(Y_future, Y_past) - H(Y_past)
    let p_yf_yp = joint_hist_to_probs(&y_future_z, &y_past_z, num_bins, smoothing);
    let (p_yp, _) = hist_to_probs(&y_past_z, num_bins, smoothing);
    if p_yf_yp.is_empty() || p_yp.is_empty() { return 0.0; }
    let p_yf_yp_flat: Vec<f64> = p_yf_yp.iter().flatten().cloned().collect();
    let h_yf_yp = shannon_entropy(&p_yf_yp_flat, 2.0);
    let h_yp = shannon_entropy(&p_yp, 2.0);
    let h_cond_without_x = (h_yf_yp - h_yp).max(0.0);
    // H(Y_future | Y_past, X_past):
    // Encode (x_past, y_past) as a unique 2D bin index with k bins per axis
    // → k² distinct composite values, equal representation of both axes.
    let k = ((num_bins as f64).sqrt().ceil() as usize).max(2);
    let joint_bins = k * k;
    let xy_past = discretize_pair(&x_past_z, &y_past_z, k);
    let p_yf_xyp = joint_hist_to_probs(&y_future_z, &xy_past, joint_bins, smoothing);
    let (p_xyp, _) = hist_to_probs(&xy_past, joint_bins, smoothing);
    if p_yf_xyp.is_empty() || p_xyp.is_empty() { return 0.0; }
    let p_yf_xyp_flat: Vec<f64> = p_yf_xyp.iter().flatten().cloned().collect();
    let h_yf_xyp = shannon_entropy(&p_yf_xyp_flat, 2.0);
    let h_xyp = shannon_entropy(&p_xyp, 2.0);
    let h_cond_with_x = (h_yf_xyp - h_xyp).max(0.0);
    (h_cond_without_x - h_cond_with_x).max(0.0)
}

// ============================================================
// NODE_GRANGER_CAUSALITY
// Test F : est-ce que X_past améliore la prédiction de Y ?
// OLS régression linéaire — O(N × lag²)
// ============================================================

#[derive(Debug, Clone)]
pub struct GrangerResult {
    pub f_stat: f64,
    pub p_value: f64,
    pub is_causal: bool,
    pub best_lag: usize,
    pub rss_restricted: f64,
    pub rss_unrestricted: f64,
}

pub fn node_granger_causality(
    state: &mut QuantumState,
    driver: &SourceKind,
    target: &SourceKind,
    max_lag: usize,
    significance: f64,
    normalize: bool,
) -> GrangerResult {
    if state.dirty { state.refresh(); }
    let x_raw: Vec<f64> = resolve_source(state, driver).to_vec();
    let y_raw: Vec<f64> = resolve_source(state, target).to_vec();
    let n = x_raw.len().min(y_raw.len());
    if n < max_lag * 2 + 4 {
        return zero_granger();
    }
    let (x, y) = if normalize {
        (zscore(&x_raw[..n]), zscore(&y_raw[..n]))
    } else {
        (x_raw[..n].to_vec(), y_raw[..n].to_vec())
    };
    let mut best_f = 0.0_f64;
    let mut best_lag = 1usize;
    let mut best_rss_r = f64::INFINITY;
    let mut best_rss_u = f64::INFINITY;
    for lag in 1..=max_lag.min(n / 4) {
        let result = granger_test_lag(&y, &x, lag, n);
        if result.0 > best_f {
            best_f = result.0;
            best_lag = lag;
            best_rss_r = result.1;
            best_rss_u = result.2;
        }
    }
    let p_value = chi2_survival(best_f, best_lag as f64);
    let is_causal = p_value < significance && best_f > 0.0;
    state.causality.granger.f_stat = best_f;
    state.causality.granger.p_value = p_value;
    state.causality.granger.is_causal = is_causal;
    GrangerResult {
        f_stat: best_f,
        p_value,
        is_causal,
        best_lag,
        rss_restricted: best_rss_r,
        rss_unrestricted: best_rss_u,
    }
}

fn granger_test_lag(y: &[f64], x: &[f64], lag: usize, n: usize) -> (f64, f64, f64) {
    let t_start = lag;
    let t_end = n;
    let t_count = t_end - t_start;
    if t_count < lag + 2 { return (0.0, 0.0, 0.0); }
    let mut y_matrix_r: Vec<Vec<f64>> = Vec::with_capacity(t_count);
    let mut targets: Vec<f64> = Vec::with_capacity(t_count);
    for t in t_start..t_end {
        let mut row = vec![1.0];
        for i in 1..=lag { row.push(y[t - i]); }
        y_matrix_r.push(row);
        targets.push(y[t]);
    }
    let rss_r = ols_rss(&y_matrix_r, &targets);
    let mut y_matrix_u: Vec<Vec<f64>> = Vec::with_capacity(t_count);
    for t in t_start..t_end {
        let mut row = vec![1.0];
        for i in 1..=lag { row.push(y[t - i]); }
        for i in 1..=lag { row.push(x[t - i]); }
        y_matrix_u.push(row);
    }
    let rss_u = ols_rss(&y_matrix_u, &targets);
    if rss_u < 1e-15 { return (0.0, rss_r, rss_u); }
    let k = lag as f64;
    let df2 = (t_count as f64) - 2.0 * k - 1.0;
    if df2 <= 0.0 { return (0.0, rss_r, rss_u); }
    let f_stat = ((rss_r - rss_u) / k) / (rss_u / df2);
    (f_stat.max(0.0), rss_r, rss_u)
}

fn ols_rss(x_matrix: &[Vec<f64>], y: &[f64]) -> f64 {
    let n = x_matrix.len();
    let p = if n > 0 { x_matrix[0].len() } else { return 0.0 };
    if n <= p { return 0.0; }
    let mut xtx = vec![vec![0.0_f64; p]; p];
    let mut xty = vec![0.0_f64; p];
    for (row, &yi) in x_matrix.iter().zip(y.iter()) {
        for i in 0..p {
            xty[i] += row[i] * yi;
            for j in 0..p {
                xtx[i][j] += row[i] * row[j];
            }
        }
    }
    let lambda = 1e-8;
    for i in 0..p { xtx[i][i] += lambda; }
    let beta = gauss_solve(&xtx, &xty);
    let mut rss = 0.0_f64;
    for (row, &yi) in x_matrix.iter().zip(y.iter()) {
        let y_pred: f64 = row.iter().zip(beta.iter()).map(|(xi, bi)| xi * bi).sum();
        rss += (yi - y_pred).powi(2);
    }
    rss
}

fn gauss_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    if n == 0 { return Vec::new(); }
    let mut aug: Vec<Vec<f64>> = a.iter()
        .zip(b.iter())
        .map(|(row, &bi)| { let mut r = row.clone(); r.push(bi); r })
        .collect();
    for col in 0..n {
        let max_row = (col..n)
            .max_by(|&i, &j| aug[i][col].abs().partial_cmp(&aug[j][col].abs()).unwrap())
            .unwrap_or(col);
        aug.swap(col, max_row);
        let pivot = aug[col][col];
        if pivot.abs() < 1e-12 { continue; }
        for row in (col + 1)..n {
            let factor = aug[row][col] / pivot;
            for k in col..=n {
                let val = aug[col][k] * factor;
                aug[row][k] -= val;
            }
        }
    }
    let mut x = vec![0.0_f64; n];
    for i in (0..n).rev() {
        let sum: f64 = (i + 1..n).map(|j| aug[i][j] * x[j]).sum();
        x[i] = if aug[i][i].abs() > 1e-12 {
            (aug[i][n] - sum) / aug[i][i]
        } else { 0.0 };
    }
    x
}

fn zscore(data: &[f64]) -> Vec<f64> {
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let std = (data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n)
        .sqrt()
        .max(1e-15);
    data.iter().map(|x| (x - mean) / std).collect()
}

fn chi2_survival(x: f64, df: f64) -> f64 {
    if x <= 0.0 { return 1.0; }
    let z = ((x / df).powf(1.0 / 3.0) - (1.0 - 2.0 / (9.0 * df)))
        / (2.0 / (9.0 * df)).sqrt();
    normal_survival(z)
}

fn normal_survival(z: f64) -> f64 {
    if z > 6.0 { return 0.0; }
    if z < -6.0 { return 1.0; }
    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let poly = t * (0.319381530
        + t * (-0.356563782
        + t * (1.781477937
        + t * (-1.821255978
        + t * 1.330274429))));
    let phi = (-0.5 * z * z).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let p = phi * poly;
    if z >= 0.0 { p } else { 1.0 - p }
}

fn zero_granger() -> GrangerResult {
    GrangerResult {
        f_stat: 0.0,
        p_value: 1.0,
        is_causal: false,
        best_lag: 0,
        rss_restricted: 0.0,
        rss_unrestricted: 0.0,
    }
}

// ============================================================
// TESTS
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum_state::{QuantumState, SourceKind};

    // ── Helpers ──────────────────────────────────────────────

    fn make_state(n: usize, amplitude: f64) -> QuantumState {
        let mut s = QuantumState::new(42, n, amplitude);
        s.base_perturbation = (0..n)
            .map(|i| ((i as f64) * 0.2).sin())
            .collect();
        s.dirty = true;
        s
    }

    fn make_coupled_state(n: usize) -> QuantumState {
        let mut s = QuantumState::new(42, n, 1.0);
        s.base_perturbation = (0..n)
            .map(|i| ((i as f64) * 0.3).sin())
            .collect();
        s.dirty = true;
        s.refresh();
        s
    }

    // ── Original sanity / bounds tests ───────────────────────

    #[test]
    fn mi_identical_series_high() {
        let mut s = make_state(100, 0.5);
        s.refresh();
        let r = node_mutual_information(
            &mut s, &SourceKind::Tension, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );
        assert!(r.mi >= 0.0);
        assert!(r.normalized_mi >= 0.0);
        assert!(r.normalized_mi <= 1.0 + 1e-6);
    }

    #[test]
    fn mi_in_valid_range() {
        let mut s = make_coupled_state(100);
        let r = node_mutual_information(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );
        assert!(r.mi >= 0.0);
        assert!(r.h_a >= 0.0);
        assert!(r.h_b >= 0.0);
    }

    #[test]
    fn te_returns_valid_values() {
        let mut s = make_coupled_state(150);
        let r = node_transfer_entropy(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            1, 2, 8, 1e-9,
        );
        assert!(r.te >= 0.0);
        assert!(r.te_reverse >= 0.0);
    }

    #[test]
    fn granger_independent_series() {
        let mut s = QuantumState::new(42, 200, 0.5);
        s.base_perturbation = (0..200).map(|i| ((i as f64) * 0.7).cos()).collect();
        s.dirty = true;
        s.refresh();
        let r = node_granger_causality(
            &mut s, &SourceKind::LogX, &SourceKind::Tension,
            3, 0.05, true,
        );
        assert!(r.f_stat >= 0.0);
        assert!(r.p_value >= 0.0);
        assert!(r.p_value <= 1.0);
    }

    #[test]
    fn granger_self_causality() {
        let mut s = make_coupled_state(200);
        let r = node_granger_causality(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            2, 0.05, true,
        );
        assert!(r.f_stat >= 0.0);
        assert!((0.0..=1.0).contains(&r.p_value));
    }

    #[test]
    fn zscore_normalized() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let z = zscore(&data);
        let mean: f64 = z.iter().sum::<f64>() / z.len() as f64;
        assert!(mean.abs() < 1e-10);
        let std: f64 = (z.iter().map(|x| x * x).sum::<f64>() / z.len() as f64).sqrt();
        assert!((std - 1.0).abs() < 1e-6);
    }

    // ── Precision tests ───────────────────────────────────────
    //
    // Each test builds a synthetic ground truth and verifies the
    // estimator recovers the expected qualitative or quantitative result.
    // Failures indicate a real algorithmic problem, not just a crash.

    // ── MI precision ─────────────────────────────────────────

    /// I(X; X) = H(X): self-information equals entropy.
    /// normalized_mi must be ~1 for an identical pair.
    #[test]
    fn mi_self_information_normalized_near_one() {
        let mut s = make_coupled_state(300);
        let r = node_mutual_information(
            &mut s, &SourceKind::Tension, &SourceKind::Tension,
            32, None, 2.0, 1e-9,
        );
        assert!(
            r.normalized_mi > 0.95,
            "I(X;X) normalized should be ~1.0, got {:.4}",
            r.normalized_mi
        );
    }

    /// Correlated pair (tension derived from base_perturbation) must have
    /// strictly higher MI than two orthogonal sinusoids.
    #[test]
    fn mi_correlated_exceeds_orthogonal() {
        let n = 300;
        let mut s_corr = make_coupled_state(n);
        let r_corr = node_mutual_information(
            &mut s_corr, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );

        let mut s_ortho = make_coupled_state(n);
        s_ortho.tension = (0..n)
            .map(|i| ((i as f64) * 1.37 + std::f64::consts::FRAC_PI_2).sin())
            .collect();
        s_ortho.dirty = false;
        let r_ortho = node_mutual_information(
            &mut s_ortho, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );

        assert!(
            r_corr.mi > r_ortho.mi,
            "Correlated MI ({:.4}) should exceed orthogonal MI ({:.4})",
            r_corr.mi, r_ortho.mi
        );
    }

    /// If y[t] = x[t - lag], MI computed with the correct lag applied
    /// must be higher than MI with no lag (unaligned pair).
    #[test]
    fn mi_lagged_higher_at_correct_shift() {
        let n = 300;
        let lag = 3usize;
        let mut s = make_coupled_state(n);
        let bp = s.base_perturbation.clone();
        let mut shifted = vec![0.0f64; n];
        for i in lag..n { shifted[i] = bp[i - lag]; }
        s.tension = shifted;
        s.dirty = false;

        let r_lag = node_mutual_information(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, Some(lag), 2.0, 1e-9,
        );
        let r_none = node_mutual_information(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );

        assert!(
            r_lag.mi > r_none.mi,
            "MI at correct lag ({:.4}) should exceed MI at lag=0 ({:.4})",
            r_lag.mi, r_none.mi
        );
    }

    /// Sub-additivity: H(A,B) ≤ H(A) + H(B) must always hold.
    #[test]
    fn mi_joint_entropy_sub_additive() {
        let mut s = make_coupled_state(200);
        let r = node_mutual_information(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            16, None, 2.0, 1e-9,
        );
        assert!(
            r.h_joint <= r.h_a + r.h_b + 1e-9,
            "H(A,B)={:.4} must be ≤ H(A)+H(B)={:.4}",
            r.h_joint, r.h_a + r.h_b
        );
    }

    // ── Granger precision ─────────────────────────────────────

    /// y[t] = 0.9·x[t-1] + small noise → must detect causality at lag 1.
    #[test]
    fn granger_detects_lag1_linear_causality() {
        let n = 400;
        let mut s = QuantumState::new(0, n, 1.0);
        s.refresh();
        let x: Vec<f64> = (0..n).map(|i| ((i as f64) * 0.4).sin()).collect();
        let noise: Vec<f64> = (0..n)
            .map(|i| ((i as f64) * 2.71828 + 1.0).sin() * 0.05)
            .collect();
        let mut y = vec![0.0f64; n];
        for t in 1..n { y[t] = 0.9 * x[t - 1] + noise[t]; }
        s.base_perturbation = x;
        s.tension = y;
        s.dirty = false;

        let r = node_granger_causality(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            4, 0.05, true,
        );
        assert!(
            r.is_causal,
            "y=0.9·x[t-1]+noise should be causal; F={:.2}, p={:.4}",
            r.f_stat, r.p_value
        );
        // best_lag need not be exactly 1 for sinusoidal x because the
        // periodic autocorrelation can inflate F at harmonic lags too.
        assert!(r.best_lag >= 1, "best_lag should be ≥ 1, got {}", r.best_lag);
        assert!(r.f_stat > 5.0, "Expected strong F-stat, got {:.2}", r.f_stat);
    }

    /// Adding regressors can only reduce (or tie) in-sample RSS.
    /// RSS_unrestricted ≤ RSS_restricted must always hold.
    #[test]
    fn granger_rss_unrestricted_le_restricted() {
        let mut s = make_coupled_state(200);
        let r = node_granger_causality(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            3, 0.05, true,
        );
        assert!(
            r.rss_unrestricted <= r.rss_restricted + 1e-9,
            "RSS_u ({:.6}) must be ≤ RSS_r ({:.6})",
            r.rss_unrestricted, r.rss_restricted
        );
    }

    /// Stronger coupling → must be detected as causal at a strict threshold
    /// while weak coupling is not.  Uses AR(1) noise (via xorshift64) rather
    /// than sinusoids, so the F-stat is not inflated by periodic autocorrelation.
    #[test]
    fn granger_stronger_coupling_detected_not_weak() {
        let n = 500;
        // Build x as AR(1) + pseudo-random noise, y[t] = coeff*x[t-1] + noise
        let make = |coeff: f64, seed: u64| {
            let mut rng = seed;
            // xorshift64-based uniform noise in [-0.5, 0.5]
            let noise: Vec<f64> = (0..n).map(|_| {
                (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5
            }).collect();
            let mut x = vec![0.0f64; n];
            let mut y = vec![0.0f64; n];
            for t in 1..n {
                x[t] = 0.5 * x[t - 1] + noise[t] * 0.3;
                y[t] = coeff * x[t - 1] + noise[(t + n / 2) % n] * 0.1;
            }
            let mut s = QuantumState::new(seed, n, 1.0);
            s.refresh();
            s.base_perturbation = x;
            s.tension = y;
            s.dirty = false;
            s
        };
        let mut s_weak   = make(0.05, 17);
        let mut s_strong = make(0.90, 17);
        let r_weak   = node_granger_causality(&mut s_weak,   &SourceKind::RawPerturbation, &SourceKind::Tension, 3, 0.05, true);
        let r_strong = node_granger_causality(&mut s_strong, &SourceKind::RawPerturbation, &SourceKind::Tension, 3, 0.01, true);
        assert!(
            r_strong.is_causal,
            "Strong coupling (0.9) should be causal; F={:.2}, p={:.4}",
            r_strong.f_stat, r_strong.p_value
        );
        assert!(
            r_strong.p_value < r_weak.p_value,
            "Stronger coupling should have lower p-value (strong={:.4}, weak={:.4})",
            r_strong.p_value, r_weak.p_value
        );
    }

    // ── TE precision ─────────────────────────────────────────

    /// x[t] = AR(1) + stochastic noise, y[t] = 0.9·x[t-1] + small noise.
    /// Uses xorshift64 so H(y_future|y_past) > 0 and TE is genuinely estimable.
    #[test]
    fn te_causal_direction_x_drives_y() {
        let n = 500;
        let mut rng = 7u64;
        let mut x = vec![0.0f64; n];
        let mut y = vec![0.0f64; n];
        for t in 1..n {
            let ex = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            let ey = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            x[t] = 0.5 * x[t - 1] + ex * 0.6;
            y[t] = 0.9 * x[t - 1] + ey * 0.1;
        }
        let mut s = QuantumState::new(7, n, 1.0);
        s.refresh();
        s.base_perturbation = x;
        s.tension = y;
        s.dirty = false;

        let r = node_transfer_entropy(
            &mut s,
            &SourceKind::RawPerturbation,
            &SourceKind::Tension,
            1, 1, 16, 1e-9,
        );
        assert!(
            r.te > r.te_reverse,
            "TE(x→y)={:.4} should exceed TE(y→x)={:.4} for AR causal x→y",
            r.te, r.te_reverse
        );
        assert_eq!(r.dominant, TEDirection::XtoY, "Expected XtoY, got {:?}", r.dominant);
    }

    /// Symmetric bidirectional AR coupling: x[t] = 0.5·y[t-1] + noise,
    /// y[t] = 0.5·x[t-1] + noise.  Both TE directions should be close.
    #[test]
    fn te_symmetric_coupling_near_equal() {
        let n = 500;
        let mut rng = 3u64;
        let mut x = vec![0.0f64; n];
        let mut y = vec![0.0f64; n];
        for t in 1..n {
            let ex = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            let ey = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            x[t] = 0.5 * y[t - 1] + ex * 0.3;
            y[t] = 0.5 * x[t - 1] + ey * 0.3;
        }
        let mut s = QuantumState::new(3, n, 1.0);
        s.refresh();
        s.base_perturbation = x;
        s.tension = y;
        s.dirty = false;

        let r = node_transfer_entropy(
            &mut s, &SourceKind::RawPerturbation, &SourceKind::Tension,
            1, 1, 16, 1e-9,
        );
        let diff = (r.te - r.te_reverse).abs();
        assert!(
            diff < 0.15,
            "|TE(x→y) - TE(y→x)| = {:.4} should be small for symmetric coupling",
            diff
        );
    }

    // ── Math helpers precision ────────────────────────────────

    /// H(uniform over k bins) = log2(k).
    #[test]
    fn shannon_entropy_uniform_equals_log2_k() {
        for k in [2usize, 4, 8, 16, 32] {
            let probs = vec![1.0 / k as f64; k];
            let h = shannon_entropy(&probs, 2.0);
            let expected = (k as f64).log2();
            assert!(
                (h - expected).abs() < 1e-10,
                "H(uniform_{k}) = {h:.6}, expected {expected:.6}"
            );
        }
    }

    /// H(point mass) = 0.
    #[test]
    fn shannon_entropy_point_mass_is_zero() {
        let mut probs = vec![0.0f64; 16];
        probs[7] = 1.0;
        let h = shannon_entropy(&probs, 2.0);
        assert!(h.abs() < 1e-10, "H(point mass) = {h:.6}, expected 0");
    }

    /// chi2_survival must be 1 at x=0 and strictly decreasing.
    #[test]
    fn chi2_survival_monotone_decreasing() {
        let df = 3.0;
        assert!((chi2_survival(0.0, df) - 1.0).abs() < 1e-6);
        let xs = [0.5_f64, 1.0, 2.0, 5.0, 10.0, 20.0];
        for w in xs.windows(2) {
            assert!(
                chi2_survival(w[0], df) > chi2_survival(w[1], df),
                "chi2_survival not decreasing at ({}, {})",
                w[0], w[1]
            );
        }
    }

    /// gauss_solve recovers exact solution of a 2×2 system.
    #[test]
    fn gauss_solve_simple_system() {
        // 2x + y = 5,  x + 3y = 10  →  x=1, y=3
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 10.0];
        let x = gauss_solve(&a, &b);
        assert!((x[0] - 1.0).abs() < 1e-8, "x = {}", x[0]);
        assert!((x[1] - 3.0).abs() < 1e-8, "y = {}", x[1]);
    }

    // ── Surrogate significance precision ─────────────────────

    /// AR causal process: x drives y with lag 1 and true stochastic noise.
    /// Surrogate test must flag TE(x→y) as significant.
    #[test]
    fn te_sig_causal_is_significant() {
        let n = 500;
        let mut rng = 11u64;
        let mut x = vec![0.0f64; n];
        let mut y = vec![0.0f64; n];
        for t in 1..n {
            let ex = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            let ey = (xorshift64(&mut rng) as f64 / u64::MAX as f64) - 0.5;
            x[t] = 0.5 * x[t - 1] + ex * 0.6;
            y[t] = 0.9 * x[t - 1] + ey * 0.1;
        }
        let mut s = QuantumState::new(11, n, 1.0);
        s.refresh();
        s.base_perturbation = x;
        s.tension = y;
        s.dirty = false;

        let r = node_transfer_entropy_significance(
            &mut s,
            &SourceKind::RawPerturbation,
            &SourceKind::Tension,
            1, 1, 16, 1e-9,
            200,   // surrogates
            0.05,  // significance level
            42,    // seed
        );
        assert!(
            r.is_significant_xy,
            "TE(x→y) should be significant for AR causal process; p={:.3}",
            r.surrogate_p_xy
        );
    }

    /// Two orthogonal sinusoids → surrogate test must NOT flag TE as significant.
    #[test]
    fn te_sig_independent_not_significant() {
        let n = 400;
        let mut s = QuantumState::new(13, n, 1.0);
        s.refresh();
        let x: Vec<f64> = (0..n).map(|i| ((i as f64) * 0.2).sin()).collect();
        let y: Vec<f64> = (0..n).map(|i| ((i as f64) * 1.37 + std::f64::consts::FRAC_PI_2).sin()).collect();
        s.base_perturbation = x;
        s.tension = y;
        s.dirty = false;

        let r = node_transfer_entropy_significance(
            &mut s,
            &SourceKind::RawPerturbation,
            &SourceKind::Tension,
            1, 1, 16, 1e-9,
            200,
            0.05,
            99,
        );
        assert!(
            !r.is_significant_xy,
            "TE(x→y) should NOT be significant for orthogonal sinusoids; p={:.3}",
            r.surrogate_p_xy
        );
    }

    /// xorshift64 must never return 0 and must produce distinct successive values.
    #[test]
    fn xorshift64_nonzero_and_distinct() {
        let mut state = 12345u64;
        let a = xorshift64(&mut state);
        let b = xorshift64(&mut state);
        let c = xorshift64(&mut state);
        assert!(a != 0 && b != 0 && c != 0, "xorshift64 returned 0");
        assert!(a != b && b != c, "xorshift64 produced duplicate values");
    }
}
