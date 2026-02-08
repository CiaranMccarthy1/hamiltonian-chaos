use crate::system::{HamiltonianSystem, State};
use crate::integrator::RungeKutta4;
use nalgebra::Normed; // For .norm()

pub struct LyapunovAnalyzer {
    pub main_trajectory: State,
    pub shadow_trajectory: State,
    initial_perturbation: f64,
    dt: f64,
}

impl LyapunovAnalyzer {
    pub fn new(start_state: State, epsilon: f64, dt: f64) -> Self {
        // Create a shadow trajectory slightly offset in theta1
        let mut shadow = start_state;
        shadow[0] += epsilon; // Perturb theta1

        Self {
            main_trajectory: start_state,
            shadow_trajectory: shadow,
            initial_perturbation: epsilon,
            dt,
        }
    }

    pub fn step<S: HamiltonianSystem>(&mut self, system: &S) {
        // 1. Advance both trajectories
        self.main_trajectory = RungeKutta4::step(system, &self.main_trajectory, self.dt);
        self.shadow_trajectory = RungeKutta4::step(system, &self.shadow_trajectory, self.dt);
    }

    /// Calculates local divergence and renormalizes the shadow trajectory
    /// Returns the local exponent contribution (log of growth)
    pub fn current_divergence_rate(&mut self) -> f64 {
        // Calculate distance between trajectories in phase space
        let diff = self.shadow_trajectory - self.main_trajectory;
        let dist = diff.norm();

        // Avoid division by zero
        if dist == 0.0 { return 0.0; }

        // The growth factor
        let ratio = dist / self.initial_perturbation;

        // Calculate local Lyapunov exponent
        let local_lambda = ratio.ln() / self.dt;

        // Renormalization: Pull shadow trajectory back along the vector of difference
        // This keeps it in the "linear" regime relative to the main trajectory
        // New Shadow = Main + (Unit Vector of Diff) * Epsilon
        self.shadow_trajectory = self.main_trajectory + (diff / dist) * self.initial_perturbation;

        local_lambda
    }
}