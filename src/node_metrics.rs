// ============================================================
// node_metrics.rs — Minimal stub
// ============================================================

/// Returns a histogram (counts per bin) over the given slice.
#[allow(dead_code)]
pub fn node_histogram(data: &[f64], num_bins: usize) -> Vec<usize> {
    if data.is_empty() || num_bins == 0 {
        return Vec::new();
    }
    let min_v = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_v = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_v - min_v).max(1e-12);
    let mut counts = vec![0usize; num_bins];
    for &v in data {
        let idx = (((v - min_v) / range) * num_bins as f64) as usize;
        counts[idx.min(num_bins - 1)] += 1;
    }
    counts
}
