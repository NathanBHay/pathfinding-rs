//! Probability Filters used within the Sampling Grid Algorithm

use std::ops::{Add, Mul};

/// A 1-dimensional Kalman filter node
/// Adapted from kalmanfilter.net/kalman1d_pn.html
#[derive(Clone, Debug)]
pub struct KalmanNode {
    pub state: f32,
    pub covariance: f32,
}

// Might make KalmanNode have Eq which is self.state == other.state
impl KalmanNode {
    /// Update the state of the Kalman filter given a measurement and measurement covariance
    /// ## Arguments
    /// * `measurement` - The measurement to update the state with
    /// * `measurement_covariance` - The covariance of the measurement
    pub fn update(&mut self, measurement: f32, measurement_covariance: f32) -> f32 {
        // As the model is 1D state and covariance predictions are the same
        let kalman_gain = self.covariance / (self.covariance + measurement_covariance);
        self.state = self.state + kalman_gain * (measurement - self.state);
        self.covariance = (1.0 - kalman_gain) * self.covariance;
        self.state
    }
}

impl Default for KalmanNode {
    fn default() -> Self {
        Self {
            state: 0.0,
            covariance: 1.0,
        }
    }
}

impl Add<f32> for KalmanNode {
    type Output = Self;

    fn add(self, _rhs: f32) -> Self {
        Self {
            state: self.state + _rhs,
            covariance: self.covariance,
        }
    }
}

impl Add<Self> for KalmanNode {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            state: self.state + _rhs.state,
            covariance: self.covariance,
        }
    }
}

impl Mul<f32> for KalmanNode {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            state: self.state * _rhs,
            covariance: self.covariance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kalman_filter() {
        let mut node = KalmanNode {
            state: 60.0,
            covariance: 225.0,
        };
        let state = node.update(49.03, 25.0);
        assert_eq!(state, 50.127);
        assert_eq!(node.covariance, 22.500006);
        let state = node.update(48.44, 25.0);
        assert_eq!(state, 49.327892);
        assert_eq!(node.covariance, 11.842108);
    }
}