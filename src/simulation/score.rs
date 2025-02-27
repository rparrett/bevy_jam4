//! Module responsible for score calculation.
use super::*;

/// - x: Input
/// - l: Max value
/// - k: Growth rate
/// - x_0: Midpoint
fn logistic_function(x: f64, l: f64, k: f64, x_0: f64) -> f64 {
    l / (1.0 + (-k * (x - x_0)).exp())
}

/// Calculate the score (between 1 and 5) using a logistic function
/// based on the given upper and lower values.
/// When x = upper, the output will be approximately 4.523.
/// When x = lower, the output will be approximately 1.477.
fn calculate_score(x: f64, lower: f64, upper: f64) -> f64 {
    let k = 4.0 / (upper - lower);
    let x_0 = (upper + lower) / 2.0;
    1.0 + logistic_function(x, 4.0, k, x_0)
}

impl Simulation {
    /// Returns the calculated score (between 1 and 5) for each category.
    /// 1. Fun
    /// 2. Presentation
    /// 3. Theme Interpretation
    /// 4. Overall
    pub fn calculate_scores(&self) -> [f64; 4] {
        let mut scores: [f64; 4] = [
            // Fun
            calculate_score(self.fun_score, 0.0, 17.5),
            // Presentation
            calculate_score(self.presentation_score, 0.0, 21.0),
            // Theme Interpretation
            calculate_score((self.entities.abs() + 1.0).log10(), -20.0, 40.0),
            // Overall
            0.0,
        ];
        scores[3] = scores.iter().sum::<f64>() / 3.0;
        scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_score_test() {
        // Low
        assert_eq!(format!("{:.3}", calculate_score(-1.0, -1.0, 1.0)), "1.477");
        // Midpoint
        assert_eq!(format!("{:.3}", calculate_score(0.0, -1.0, 1.0)), "3.000");
        // High
        assert_eq!(format!("{:.3}", calculate_score(1.0, -1.0, 1.0)), "4.523");
    }

    #[test]
    fn calculate_score_edge_cases_test() {
        // Same test with different scales
        assert!(f64::abs(calculate_score(f64::INFINITY, -1.0, 1.0) - 5.0) < 0.001);
        assert!(f64::abs(calculate_score(-f64::INFINITY, -1.0, 1.0) - 1.0) < 0.001);

        assert!(f64::abs(calculate_score(f64::INFINITY, 0.0, 1e9) - 5.0) < 0.001);
        assert!(f64::abs(calculate_score(-f64::INFINITY, 0.0, 1e9) - 1.0) < 0.001);

        assert!(f64::abs(calculate_score(f64::INFINITY, -1e9, 1e9) - 5.0) < 0.001);
        assert!(f64::abs(calculate_score(-f64::INFINITY, -1e9, 1e9) - 1.0) < 0.001);

        assert!(f64::abs(calculate_score(f64::INFINITY, -1e32, 1e32) - 5.0) < 0.001);
        assert!(f64::abs(calculate_score(-f64::INFINITY, -1e32, 1e32) - 1.0) < 0.001);
    }
}
