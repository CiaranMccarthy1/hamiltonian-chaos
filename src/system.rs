use nalgebra::SVector;

pub type State = SVector<f64, 4>;

pub trait HamiltonianSystem {
    fn derivatives(&self, state: &State) -> State;
    fn energy(&self, state: &State) -> f64;
    fn get_accelerations(&self, theta1: f64, theta2: f64, p1: f64, p2: f64) -> (f64, f64);
    fn get_velocities(&self, theta1: f64, theta2: f64, p1: f64, p2: f64) -> (f64, f64);
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
        let delta = t1 - t2;
        let mu = 1.0 + self.m1 / self.m2;

        let p1 = (self.m1 + self.m2) * self.l1.powi(2) * w1
            + self.m2 * self.l1 * self.l2 * w2 * delta.cos();
        let p2 = self.m2 * self.l2.powi(2) * w2
            + self.m2 * self.l1 * self.l2 * w1 * delta.cos();

        State::from_vec(vec![t1, t2, p1, p2])
    }
}

impl HamiltonianSystem for DoublePendulum {
    fn get_velocities(&self, theta1: f64, theta2: f64, p1: f64, p2: f64) -> (f64, f64) {
        let delta = theta1 - theta2;
        let cos_delta = delta.cos();
        let sin_delta = delta.sin();

        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;

        let denom = l1 * l2 * (m1 + m2 * sin_delta.powi(2));

        let omega1 = (l2 * p1 - l1 * p2 * cos_delta) / (l1 * denom);
        let omega2 = (l1 * (m1 + m2) * p2 - l2 * m2 * p1 * cos_delta) / (l2 * denom);

        (omega1, omega2)
    }

    fn get_accelerations(&self, theta1: f64, theta2: f64, p1: f64, p2: f64) -> (f64, f64) {
        let delta = theta1 - theta2;
        let cos_delta = delta.cos();
        let sin_delta = delta.sin();

        let (omega1, omega2) = self.get_velocities(theta1, theta2, p1, p2);

        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;
        let g = self.g;

        let c1 = p1 * p2 * sin_delta / (l1 * l2 * (m1 + m2 * sin_delta.powi(2)));
        let c2 = (p1.powi(2) * m2 - 2.0 * p1 * p2 * m2 * cos_delta
            + p2.powi(2) * (m1 + m2)) * sin_delta * cos_delta
            / (2.0 * l1.powi(2) * l2.powi(2) * (m1 + m2 * sin_delta.powi(2)).powi(2));

        let dp1_dt = -(m1 + m2) * g * l1 * theta1.sin() - c1 + c2;
        let dp2_dt = -m2 * g * l2 * theta2.sin() + c1 - c2;

        (dp1_dt, dp2_dt)
    }

    fn energy(&self, state: &State) -> f64 {
        let t1 = state[0];
        let t2 = state[1];
        let p1 = state[2];
        let p2 = state[3];

        let delta = t1 - t2;
        let cos_delta = delta.cos();

        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;
        let g = self.g;

        let denom = m1 + m2 * delta.sin().powi(2);

        let kinetic = (m2 * l2.powi(2) * p1.powi(2)
            + (m1 + m2) * l1.powi(2) * p2.powi(2)
            - 2.0 * m2 * l1 * l2 * p1 * p2 * cos_delta)
            / (2.0 * m2 * l1.powi(2) * l2.powi(2) * denom);

        let potential = -(m1 + m2) * g * l1 * t1.cos()
            - m2 * g * l2 * t2.cos();

        kinetic + potential
    }

    fn derivatives(&self, state: &State) -> State {
        let theta1 = state[0];
        let theta2 = state[1];
        let p1 = state[2];
        let p2 = state[3];

        let (omega1, omega2) = self.get_velocities(theta1, theta2, p1, p2);
        let (dp1_dt, dp2_dt) = self.get_accelerations(theta1, theta2, p1, p2);

        State::from_vec(vec![omega1, omega2, dp1_dt, dp2_dt])
    }
}