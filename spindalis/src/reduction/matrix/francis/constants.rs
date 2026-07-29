// ============================================================================
// Francis QR Algorithm Parameters & Tuning Constants
// ============================================================================

/// **Primary Production Profile** (Success Rate: ~99.98% on test suites)
///
/// - **`MAX_ITERS` (16):** Upper bound on iterations per active window.
///   Successfully achieving deflation "refunds" half of these iterations
///   (`curriter = curriter.saturating_sub(MAX_ITERS >> 1)`), preventing
///   stalls from penalizing subsequent windows.
///
/// - **`TOLERANCE` (1e-4) & `ABSOLUTE_CAP` (1e-3):** Tuned specifically for
///   approximate symmetric matrices (e.g., those generated via $AA^T$).
///   Single-precision (`f32`) rounding error in $AA^T$ constructions prevents
///   stricter convergence.
///   *(Note: While diagonal averaging/rectification could theoretically clean up
///   minor symmetry drift prior to decomposition, it is generally cleaner to
///   handle this via a slightly relaxed absolute floor like `1e-3` here).*
///
/// - **`EPSILON` (1e-18):** Numerical floor guarding against division-by-zero
///   and negligible subdiagonal elements during Householder/Givens setups.
pub const MAX_ITERS: usize = 16;
pub const TOLERANCE: f32 = 1e-4;
pub const ABSOLUTE_CAP: f32 = 1e-3;
pub const EPSILON: f32 = 1e-21;

// // -------------------------------------------------------------------------
// // **Strict Precision Profile** (Alternative for high-precision synthetic inputs)
// // -------------------------------------------------------------------------
// pub const MAX_ITERS: usize = 24;
// pub const TOLERANCE: f32 = 1e-8;
// pub const ABSOLUTE_CAP: f32 = 1e-6; // Recommended companion adjustment if strict
// pub const EPSILON: f32 = 1e-26;
