mod system;
mod integrator;
mod analysis;


use system::{DoublePendulum, HamiltonianSystem};
use integrator::RungeKutta4;
use analysis::LyapunovAnalyzer;
use std::fs::File;
use std::io::Write;

// Constants
const G: f64 = 9.81;
const L1: f64 = 1.0;
const L2: f64 = 1.0;
const M1: f64 = 1.0;
const M2: f64 = 1.0;

fn main() -> std::io::Result<()> {
    println!("--- Hamiltonian Chaos Engine Initialized ---");
    println!("Target: Double Pendulum | Integrator: RK4");

    // Configuration
    let dt = 0.00001;
    let max_time = 100.0;
    let steps = (max_time / dt) as usize;
    let perturbation_magnitude = 1e-8;

    // We scan different initial energy levels (approximated by changing initial angle theta1)
    // In a full implementation, you would solve for exact angles given a target Energy V.
    let mut initial_angles = Vec::new(); // Radians

    for _i in 0..=100 {
        initial_angles.push(_i as f64 * 0.5);
    }

    let mut results = Vec::new();

    for &theta_start in &initial_angles {
        // 1. Initialize System
        let system = DoublePendulum::new(M1, M2, L1, L2, G);

        // State vector: [theta1, theta2, p_theta1, p_theta2]
        // Note: For RK4 simplicity here, we simulate in Lagrangian coordinates [t1, t2, w1, w2]
        // and convert to Hamiltonian for energy checks.
        let initial_state = system.make_state(theta_start, 0.0, 0.0, 0.0);

        // 2. Initialize Analyzer
        let mut analyzer = LyapunovAnalyzer::new(
            initial_state,
            perturbation_magnitude,
            dt
        );

        // 3. Simulation Loop
        let mut _time = 0.0;
        let mut running_lambda_sum = 0.0;
        let mut energy_drift = 0.0;
        let initial_energy = system.energy(&initial_state);

        for i in 0..steps {
            // Evolve the system
            analyzer.step(&system);
            _time += dt;

            // Accumulate Lyapunov Exponent
            running_lambda_sum += analyzer.current_divergence_rate();

            // Check Energy Conservation (Drift)
            let current_energy = system.energy(&analyzer.main_trajectory);
            let drift = (current_energy - initial_energy).abs() / initial_energy;
            if drift > energy_drift { energy_drift = drift; }
        }

        let avg_lambda = running_lambda_sum / (steps as f64);

        // Calculate Predictability Horizon T = (1/lambda) * ln(1/epsilon)
        let horizon_t = (1.0 / avg_lambda) * (1.0 / perturbation_magnitude).ln();

        println!(
            "Theta: {:.2} rad | Energy: {:.4} J | LLE (λ): {:.4} | Horizon (T): {:.2}s | Max Drift: {:.2e}%",
            theta_start, initial_energy, avg_lambda, horizon_t, energy_drift * 100.0
        );

        results.push((initial_energy, avg_lambda, horizon_t));
    }

    // Export Logic (Simplified)
    let json = serde_json::to_string(&results)?;
    let mut file = File::create("chaos_results.json")?;
    file.write_all(json.as_bytes())?;

    println!("--- Data Exported to chaos_results.json ---");
    Ok(())
}