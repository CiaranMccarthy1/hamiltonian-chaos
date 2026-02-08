use nalgebra::SVector;

// State is a vector of dimension 4: [theta1, theta2, omega1, omega2]
pub type State = SVector<f64, 4>;

pub trait HamiltonianSystem {
    fn derivatives(&self, state: &State) -> State;
    fn energy(&self, state: &State) -> f64;
}

pub struct DoublePendulum {
    m1: f64, m2: f64,
    l1: f64, l2: f64,
    g: f64,
}

impl DoublePendulum {
    pub fn new(m1: f64, m2: f64, l1: f64, l2: f64, g: f64) -> Self {
        Self { m1, m2, l1, l2, g }
    }

    pub fn make_state(&self, t1: f64, t2: f64, w1: f64, w2: f64) -> State {
        State::from_vec(vec![t1, t2, w1, w2])
    }
}

impl HamiltonianSystem for DoublePendulum {
    fn energy(&self, state: &State) -> f64 {
        let t1 = state[0]; let t2 = state[1];
        let w1 = state[2]; let w2 = state[3];

        // Potential Energy V
        let v = -self.m1 * self.g * self.l1 * t1.cos()
            - self.m2 * self.g * (self.l1 * t1.cos() + self.l2 * t2.cos());

        // Kinetic Energy T
        let t = 0.5 * self.m1 * (self.l1 * w1).powi(2)
            + 0.5 * self.m2 * ( (self.l1 * w1).powi(2) + (self.l2 * w2).powi(2)
            + 2.0 * self.l1 * self.l2 * w1 * w2 * (t1 - t2).cos() );

        t + v
    }

    fn derivatives(&self, state: &State) -> State {
        let t1 = state[0]; let t2 = state[1];
        let w1 = state[2]; let w2 = state[3];

        let m1 = self.m1; let m2 = self.m2;
        let l1 = self.l1; let l2 = self.l2;
        let g = self.g;

        // Equations of Motion (Lagrange formulation resolved to accelerations)
        // These are algebraically messy, standard for Double Pendulum simulation.

        let delta = t1 - t2;
        let den1 = (m1 + m2) * l1 - m2 * l1 * delta.cos().powi(2);
        let den2 = (l2 / l1) * den1;

        let dw1_num = m2 * l1 * w1.powi(2) * delta.sin() * delta.cos()
            + m2 * g * t2.sin() * delta.cos()
            + m2 * l2 * w2.powi(2) * delta.sin()
            - (m1 + m2) * g * t1.sin();

        let dw2_num = - m2 * l2 * w2.powi(2) * delta.sin() * delta.cos()
            + (m1 + m2) * g * t1.sin() * delta.cos()
            - (m1 + m2) * l1 * w1.powi(2) * delta.sin()
            - (m1 + m2) * g * t2.sin();

        let dw1 = dw1_num / den1;
        let dw2 = dw2_num / den2;

        State::from_vec(vec![w1, w2, dw1, dw2])
    }
}