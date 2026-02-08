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

pub struct SymplecticEuler;

impl SymplecticEuler {
    pub fn step<S: HamiltonianSystem>(system: &S, state: &State, dt: f64) -> State {
        let theta1 = state[0];
        let theta2 = state[1];
        let p1 = state[2];
        let p2 = state[3];

        let (dp1, dp2) = system.get_accelerations(theta1, theta2, p1, p2);

        let p1_new = p1 + dp1 * dt;
        let p2_new = p2 + dp2 * dt;

        let (omega1, omega2) = system.get_velocities(theta1, theta2, p1_new, p2_new);

        let theta1_new = theta1 + omega1 * dt;
        let theta2_new = theta2 + omega2 * dt;

        State::from_vec(vec![theta1_new, theta2_new, p1_new, p2_new])
    }
}

pub struct LeapfrogIntegrator;

impl LeapfrogIntegrator {
    pub fn step<S: HamiltonianSystem>(system: &S, state: &State, dt: f64) -> State {
        let theta1 = state[0];
        let theta2 = state[1];
        let p1 = state[2];
        let p2 = state[3];

        let (dp1, dp2) = system.get_accelerations(theta1, theta2, p1, p2);
        let p1_half = p1 + dp1 * (dt / 2.0);
        let p2_half = p2 + dp2 * (dt / 2.0);

        let (omega1, omega2) = system.get_velocities(theta1, theta2, p1_half, p2_half);
        let theta1_new = theta1 + omega1 * dt;
        let theta2_new = theta2 + omega2 * dt;

        let (dp1_new, dp2_new) = system.get_accelerations(theta1_new, theta2_new, p1_half, p2_half);
        let p1_new = p1_half + dp1_new * (dt / 2.0);
        let p2_new = p2_half + dp2_new * (dt / 2.0);

        State::from_vec(vec![theta1_new, theta2_new, p1_new, p2_new])
    }
}

pub struct Verlet;

impl Verlet {
    pub fn step<S: HamiltonianSystem>(system: &S, state: &State, dt: f64) -> State {
        let theta1 = state[0];
        let theta2 = state[1];
        let p1 = state[2];
        let p2 = state[3];

        let (omega1_0, omega2_0) = system.get_velocities(theta1, theta2, p1, p2);

        let theta1_half = theta1 + omega1_0 * (dt / 2.0);
        let theta2_half = theta2 + omega2_0 * (dt / 2.0);

        let (dp1, dp2) = system.get_accelerations(theta1_half, theta2_half, p1, p2);
        let p1_new = p1 + dp1 * dt;
        let p2_new = p2 + dp2 * dt;

        let (omega1_new, omega2_new) = system.get_velocities(theta1_half, theta2_half, p1_new, p2_new);

        let theta1_new = theta1_half + omega1_new * (dt / 2.0);
        let theta2_new = theta2_half + omega2_new * (dt / 2.0);

        State::from_vec(vec![theta1_new, theta2_new, p1_new, p2_new])
    }
}

pub struct Yoshida4;

impl Yoshida4 {
    pub fn step<S: HamiltonianSystem>(system: &S, state: &State, dt: f64) -> State {
        let w0 = -1.702414383919315;
        let w1 = 1.351207191959658;
        let w2 = w0;

        let c = vec![w1 / 2.0, (w0 + w1) / 2.0, (w0 + w2) / 2.0, w2 / 2.0];
        let d = vec![w1, w0, w2];

        let mut theta1 = state[0];
        let mut theta2 = state[1];
        let mut p1 = state[2];
        let mut p2 = state[3];

        for i in 0..4 {
            let (omega1, omega2) = system.get_velocities(theta1, theta2, p1, p2);
            theta1 += c[i] * dt * omega1;
            theta2 += c[i] * dt * omega2;

            if i < 3 {
                let (dp1, dp2) = system.get_accelerations(theta1, theta2, p1, p2);
                p1 += d[i] * dt * dp1;
                p2 += d[i] * dt * dp2;
            }
        }

        State::from_vec(vec![theta1, theta2, p1, p2])
    }
}