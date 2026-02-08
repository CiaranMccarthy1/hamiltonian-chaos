use crate::system::{HamiltonianSystem, State};

pub struct RungeKutta4;

impl RungeKutta4 {
    pub fn step<S: HamiltonianSystem>(system: &S, state: &State, dt: f64) -> State {
        let k1 = system.derivatives(state);
        let k2 = system.derivatives(&(state + k1 * (0.5 * dt)));
        let k3 = system.derivatives(&(state + k2 * (0.5 * dt)));
        let k4 = system.derivatives(&(state + k3 * dt));

        state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0)
    }
}