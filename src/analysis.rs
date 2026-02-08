use crate::system::{HamiltonianSystem, State};
use crate::integrator::RungeKutta4;
use nalgebra::Normed;

pub struct LyapunovAnalyzer {
    pub main_trajectory: State,
    pub shadow_trajectory: State,
    initial_perturbation: f64,
    dt: f64,
    cumulative_log_stretch: f64,
    total_time: f64,
}

impl LyapunovAnalyzer {
    pub fn new(start_state: State, epsilon: f64, dt: f64) -> Self {
        let mut shadow = start_state;
        shadow[0] += epsilon;

        Self {
            main_trajectory: start_state,
            shadow_trajectory: shadow,
            initial_perturbation: epsilon,
            dt,
            cumulative_log_stretch: 0.0,
            total_time: 0.0,
        }
    }

    pub fn step<S: HamiltonianSystem>(&mut self, system: &S) {
        self.main_trajectory = RungeKutta4::step(system, &self.main_trajectory, self.dt);
        self.shadow_trajectory = RungeKutta4::step(system, &self.shadow_trajectory, self.dt);
        self.total_time += self.dt;
    }

    pub fn renormalize_and_get_stretch(&mut self) -> f64 {
        let diff = self.shadow_trajectory - self.main_trajectory;
        let dist = diff.norm();

        if dist == 0.0 {
            return 0.0;
        }

        let stretch_factor = dist / self.initial_perturbation;
        let log_stretch = stretch_factor.ln();

        self.shadow_trajectory = self.main_trajectory
            + (diff / dist) * self.initial_perturbation;

        self.cumulative_log_stretch += log_stretch;

        log_stretch
    }

    pub fn current_mle(&self) -> f64 {
        if self.total_time == 0.0 {
            0.0
        } else {
            self.cumulative_log_stretch / self.total_time
        }
    }

    pub fn get_total_time(&self) -> f64 {
        self.total_time
    }
}