mod system;
mod integrator;
mod analysis;

use system::{DoublePendulum, HamiltonianSystem};
use integrator::Yoshida4;
use analysis::LyapunovAnalyzer;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;

const G: f64 = 9.81;
const L1: f64 = 1.0;
const L2: f64 = 1.0;
const M1: f64 = 1.0;
const M2: f64 = 1.0;

fn main() -> std::io::Result<()> {
    println!("=== Hamiltonian Chaos Engine ===");
    println!("System: Double Pendulum");
    println!("Integrator: Yoshida\n");

    let dt = 0.001;
    let max_time = 60.0;
    let renormalization_interval = 5;
    let steps = (max_time / dt) as usize;
    let perturbation = 1e-8;

    println!("Configuration:");
    println!("  dt = {} s", dt);
    println!("  Duration = {} s", max_time);
    println!("  Steps = {}", steps);
    println!("  Perturbation = {:.1e}\n", perturbation);

    let resolution = 36;
    let initial_conditions: Vec<(f64, f64)> = (0..resolution)
        .flat_map(|i| {
            (0..resolution).map(move |j| {
                let theta1 = i as f64 * 2.0 * std::f64::consts::PI / resolution as f64;
                let theta2 = j as f64 * 2.0 * std::f64::consts::PI / resolution as f64;
                (theta1, theta2)
            })
        })
        .collect();

    println!("Running {} simulations in parallel...\n", initial_conditions.len());

    let results: Vec<_> = initial_conditions.par_iter()
        .map(|&(theta1, theta2)| {
            let system = DoublePendulum::new(M1, M2, L1, L2, G);
            let initial_state = system.make_state(theta1, theta2, 0.0, 0.0);

            let mut analyzer = LyapunovAnalyzer::new(
                initial_state,
                perturbation,
                dt
            );

            let initial_energy = system.energy(&initial_state);
            let mut max_drift: f64 = 0.0;

            for i in 0..steps {
                analyzer.step(&system);

                if i % renormalization_interval == 0 {
                    analyzer.renormalize_and_get_stretch();
                }

                let current_energy = system.energy(&analyzer.main_trajectory);
                let drift = ((current_energy - initial_energy) / initial_energy).abs();
                max_drift = max_drift.max(drift);
            }

            analyzer.renormalize_and_get_stretch();
            let mle = analyzer.current_mle();

            let horizon = if mle > 1e-6 {
                (1.0 / mle) * (1.0 / perturbation).ln()
            } else {
                f64::INFINITY
            };

            (theta1, theta2, initial_energy, mle, horizon, max_drift)
        })
        .collect();

    println!("{:>8} {:>8} {:>12} {:>10} {:>12} {:>12}",
             "θ₁ (rad)", "θ₂ (rad)", "Energy (J)", "λ (1/s)", "Horizon (s)", "Drift (%)");
    println!("{}", "=".repeat(78));

    for (theta1, theta2, energy, lambda, horizon, drift) in &results {
        let regime = if *lambda < 0.01 {
            "regular"
        } else if *lambda < 0.5 {
            "weakly chaotic"
        } else {
            "chaotic"
        };

        println!("{:8.3} {:8.3} {:12.4} {:10.4} {:12.2} {:11.2e}  [{}]",
                 theta1, theta2, energy, lambda, horizon, drift * 100.0, regime);
    }

    #[derive(serde::Serialize)]
    struct ResultData {
        theta1_initial: f64,
        theta2_initial: f64,
        energy: f64,
        lyapunov_exponent: f64,
        predictability_horizon: f64,
        max_energy_drift: f64,
    }

    let export: Vec<ResultData> = results.iter()
        .map(|(t1, t2, e, l, h, d)| ResultData {
            theta1_initial: *t1,
            theta2_initial: *t2,
            energy: *e,
            lyapunov_exponent: *l,
            predictability_horizon: *h,
            max_energy_drift: *d,
        })
        .collect();

    let json = serde_json::to_string_pretty(&export)?;
    let mut file = File::create("chaos_results.json")?;
    file.write_all(json.as_bytes())?;

    println!("\n✓ Results exported to chaos_results.json");
    Ok(())
}