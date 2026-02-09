mod system;
mod integrator;
mod analysis;

use system::{DoublePendulum, HamiltonianSystem};
use integrator::Yoshida4;
use analysis::LyapunovAnalyzer;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let res_atlas = 180;
    let dt = 0.001;
    let max_time_atlas = 100.0;
    let steps_atlas = (max_time_atlas / dt) as usize;
    let perturbation = 1e-8;

    println!("Calculating Scientific Atlas ({}x{} points)...", res_atlas, res_atlas);

    let atlas_results: Vec<_> = (0..res_atlas).into_par_iter().flat_map(|i| {
        (0..res_atlas).into_par_iter().map(move |j| {
            let t1 = i as f64 * 2.0 * std::f64::consts::PI / res_atlas as f64;
            let t2 = j as f64 * 2.0 * std::f64::consts::PI / res_atlas as f64;

            let system = DoublePendulum::new(1.0, 1.0, 1.0, 1.0, 9.81);
            let mut analyzer = LyapunovAnalyzer::new(system.make_state(t1, t2, 0.0, 0.0), perturbation, dt);

            for k in 0..steps_atlas {
                analyzer.step(&system);
                if k % 5 == 0 { analyzer.renormalize_and_get_stretch(); }
            }
            (t1, t2, analyzer.current_mle())
        })
    }).collect();

    let mut atlas_file = BufWriter::new(File::create("chaos_atlas.csv")?);
    writeln!(atlas_file, "theta1,theta2,lambda")?;
    for (t1, t2, l) in atlas_results {
        writeln!(atlas_file, "{:.6},{:.6},{:.6}", t1, t2, l)?;
    }

    let res_anim = 180;
    let total_frames = 600;
    let dt_anim = 0.001;
    let steps_per_frame = 50;

    println!("Generating Animation Data ({} pendulums)...", res_anim * res_anim);

    let mut anim_file = BufWriter::new(File::create("animation_data.csv")?);
    writeln!(anim_file, "frame,id,x,y,theta1,theta2")?;

    // --- STEP 1: Initialize states ONCE outside the loop ---
    let mut anim_states: Vec<_> = (0..res_anim).flat_map(|i| {
        (0..res_anim).map(move |j| {
            let t1 = i as f64 * 2.0 * std::f64::consts::PI / res_anim as f64;
            let t2 = j as f64 * 2.0 * std::f64::consts::PI / res_anim as f64;
            let system = DoublePendulum::new(1.0, 1.0, 1.0, 1.0, 9.81);
            let state = system.make_state(t1, t2, 0.0, 0.0);
            (system, state)
        })
    }).collect();

    // --- STEP 2: The loop only UPDATES and SAVES the existing states ---
    for frame in 0..total_frames {
        // This updates the positions based on the PREVIOUS frame's positions
        anim_states.par_iter_mut().for_each(|(sys, state)| {
            for _ in 0..steps_per_frame {
                *state = Yoshida4::step(sys, state, dt_anim);
            }
        });

        for (id, (sys, state)) in anim_states.iter().enumerate() {
            let t1 = state[0];
            let t2 = state[1];
            let x = 1.0 * t1.sin() + 1.0 * (t1 + t2).sin();
            let y = -1.0 * t1.cos() - 1.0 * (t1 + t2).cos();
            writeln!(anim_file, "{},{},{:.4},{:.4},{:.4},{:.4}", frame, id, x, y, t1, t2)?;
        }
    }

    let total_points = res_atlas * res_atlas;
    println!("Successfully saved {total_points} points to chaos_atlas.csv");
    println!("Animation frames saved to animation_data.csv");

    Ok(())
}