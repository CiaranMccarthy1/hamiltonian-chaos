# Hamiltonian Chaos Engine

A double pendulum simulator that tracks how chaotic the system gets across different starting positions.

## Overview

This is a Rust project that simulates a double pendulum and measures how sensitive it is to tiny changes in starting conditions. That sensitivity is called the **Lyapunov exponent**.

The project:
- Uses Hamiltonian mechanics to simulate 
- Simulates 1,296 different starting positions
- Measures chaos at each one
- Exports results to JSON

## The Physics 

### State

The pendulum is described by four numbers:
```
[θ₁, θ₂, p₁, p₂]
```

- `θ₁, θ₂` = angles of the two pendulums
- `p₁, p₂` = momentum (derived from angle velocities, not just mass × velocity)

### How It Moves

```
dθᵢ/dt = ∂H/∂pᵢ     (angle changes based on momentum)
dpᵢ/dt = -∂H/∂θᵢ    (momentum changes based on gravity/forces)
```

Where `H` is the **Hamiltonian** energy function.

### Measuring Chaos

Start two pendulums at almost the same position. Run them both forward. If they diverge fast, it's chaotic. Measure how fast:

```
λ = (1/time) × ln(distance_apart / initial_distance)
```

- `λ > 0.5` = chaotic (butterfly effect)
- `λ < 0.01` = regular (predictable)

## How to Run

```bash
# Compile and run
cargo build --release
cargo run --release

# Pick a visualization
python visual.py           # Quick 2D heatmap
python 3d-visualizer.py            # 3D surface plot
python animation.py                # Animated grid
python fractial.py                 # Fractal dimension
```

This simulates 1,296 starting positions over 40 seconds each. Takes ~5-10 minutes.

Output file: `chaos_results.json`

```json
[
  {
    "theta1_initial": 0.0,
    "theta2_initial": 0.0,
    "energy": -29.43,
    "lyapunov_exponent": 0.0389,
    "predictability_horizon": 473.46
  },
  ...
]
```

## Code Layout

```
src/
├── main.rs        # Run the simulations in parallel
├── system.rs      # Physics equations
├── integrator.rs  # Different ways to solve them
└── analysis.rs    # Compute Lyapunov exponents

visualizers/
├── visual.py           # 2D chaos heatmap
├── 3d-visualizer.py    # 3D energy landscape colored by chaos
├── animation.py        # Animate a grid of pendulums evolving
└── fractial.py         # Calculate fractal dimension of chaos boundary
```

## Why Hamiltonian

Standard numerical methods drift in energy over time. Symplectic integrators like Störmer-Verletand and Yoshida4 are designed for Hamiltonian systems. They keep energy stable for decades of simulated time.

## Parameters You Can Tweak

In `src/main.rs`:
```rust
const L1: f64 = 1.0;      // Length of pendulum 1 (m)
const L2: f64 = 1.0;      // Length of pendulum 2 (m)
const M1: f64 = 1.0;      // Mass 1 (kg)
const M2: f64 = 1.0;      // Mass 2 (kg)
const G: f64 = 9.81;      // Gravity (m/s²)

let dt = 0.001;           // Timestep (seconds)
let max_time = 40.0;      // How long to run
```

Change these and re-run to see how the chaos map changes.

## Dependencies

**Rust:**
```toml
nalgebra = "0.32"    # Linear algebra
rayon = "1.7"        # Parallel loops
serde_json = "1.0"   # JSON output
```

**Python:**
```bash
pip install pandas numpy matplotlib scipy
```


## Results

The output is a 36×36 grid of Lyapunov exponents over the angle space. Visualize it as a heatmap:
- Dark blue = regular (λ ≈ 0)
- Red = chaotic (λ > 1.0)

