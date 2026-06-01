# lau-geometric-growth

**Shell growth as computation** — logarithmic spirals, golden ratio growth, and temporal recording in spatial geometry.

---

## What This Does

This crate implements the thesis that **biological growth is computation**: a conch shell records its entire growth history in its spatial geometry, and each new increment is determined by the curvature of what already exists. It provides:

- **Logarithmic spirals** — the fundamental growth law, with golden spiral specialization
- **Curvature inheritance** — each growth ring's curvature is inherited from its parent
- **Temporal recording** — time maps to space through growth rings (read the shell, read the history)
- **Fibonacci phyllotaxis** — sunflower spirals via the golden angle (optimal packing)
- **Nautilus chamber growth** — gas pressure determines chamber geometry
- **Fractal self-similarity** — Mandelbrot zoom equivalence in biological growth
- **Golden growth rate** — φ as the most irrational number, continued fraction analysis
- **Quasicrystal growth** — aperiodic but deterministic tiling via cut-and-project
- **Agent capability shells** — skills positioned on a growing spiral like a nautilus
- **Ricci flow growth** — curvature equalization as the growth driver

The crate contains **105 tests** across 10 modules.

---

## Key Idea

> *The growth law IS the constructor.*

A nautilus doesn't calculate where to build its next chamber. The gas pressure in existing chambers determines the next one. A sunflower doesn't compute optimal packing — the golden angle (~137.508°) produces it automatically because φ is the most irrational number. The shell's spatial geometry IS the temporal record of its growth.

---

## Install

```toml
[dependencies]
lau-geometric-growth = { git = "https://github.com/SuperInstance/lau-geometric-growth" }
```

### Dependencies

- `nalgebra` 0.33 — linear algebra (points, vectors, rotations)
- `serde` 1.x (with `derive`) — serialization
- `serde_json` 1.x — JSON support for temporal data payloads

### Dev Dependencies

- `approx` 0.5 — floating-point comparison macros

---

## Quick Start

```rust
use lau_geometric_growth::{
    LogarithmicSpiral, CurvatureInheritance, TemporalRecorder,
    Phyllotaxis, NautilusChamber, GoldenGrowthRate, RicciFlowGrowth,
};

// Golden spiral
let mut spiral = LogarithmicSpiral::golden(1.0);
let points = spiral.generate(50, 0.1);
println!("Generated {} points", points.len());

// Curvature inheritance (shell growth)
let mut shell = CurvatureInheritance::new(0.95, 0.1);
shell.seed(1.0, 0.1);
for _ in 0..20 { shell.grow(); }

// Phyllotaxis (sunflower)
let phylo = Phyllotaxis::new(100, 1.0);
let points = phylo.generate();
println!("Packing quality: {}", phylo.packing_quality());

// Ricci flow (curvature smoothing)
let mut ricci = RicciFlowGrowth::new(0.01, 0.5);
ricci.init_perturbed_circle(50, 0.3);
ricci.flow(100);
println!("Final curvature variance: {}", ricci.curvature_variance());
```

Run all 105 tests:

```bash
cargo test
```

---

## API Reference

### `spiral` — Logarithmic Spiral Growth

| Type/Method | Description |
|------|-------------|
| `LogarithmicSpiral::golden(a)` | Golden spiral: b = ln(φ)/(π/2), growth per quarter turn = φ |
| `::new(a, b)` | Custom growth rate |
| `.radius_at(θ)` | r = a·e^(bθ) |
| `.curvature_at(θ)` | κ = 1 / (r·√(1+b²)) |
| `.to_cartesian(θ)` | Polar → (x, y) |
| `.generate(n, dθ)` | Generate n points with angular step dθ |
| `.arc_length(θ₁, θ₂)` | Arc length between angles |
| `.zoom(θ₁, θ₂, n)` | Sub-spiral between two angles (self-similarity) |
| `SpiralPoint` | θ, r, x, y, curvature, time_index |

### `curvature` — Curvature Inheritance

| Type/Method | Description |
|------|-------------|
| `CurvatureInheritance::new(damping, base_rate)` | Create engine |
| `.seed(curvature, radius)` | Initial growth ring |
| `.grow()` | Add one ring: κ_{n+1} = damping·κ_n + base_rate/(1+κ_n) |
| `.grow_n(n)` | Add n rings |
| `.total_thickness()` | Cumulative shell thickness |
| `GrowthRing` | index, radius, curvature, thickness, growth_angle |

### `temporal` — Temporal Recording in Space

| Type/Method | Description |
|------|-------------|
| `TemporalRecorder::new(growth_rate)` | Create recorder |
| `.record(data)` | Record a moment (with optional JSON payload) |
| `.record_n(n, data_fn)` | Record n moments |
| `.read_at(ring_index)` | Read the state at a spatial position (= time) |
| `.time_range()` | (first, last) physical times recorded |
| `.time_to_space(time)` | Convert time → ring index |
| `TimeSlice` | ring_index, physical_time, radius, curvature, growth_rate, data |

### `fibonacci` — Phyllotaxis and Golden Angle

| Type/Method | Description |
|------|-------------|
| `GoldenAngle::radians()` / `::degrees()` | ~2.399 rad / ~137.508° |
| `GoldenAngle::uniformity_score(n)` | How uniformly golden angle distributes n points |
| `GoldenAngle::uniformity_vs_rational(n, angle)` | Compare golden to rational angles |
| `Phyllotaxis::new(n, scale)` | n points, scale factor |
| `.generate()` | Points using golden angle on Fermat's spiral |
| `.packing_quality()` | Minimum pairwise distance |
| `.spiral_counts()` | Count clockwise/counterclockwise parastichy spirals |

### `nautilus` — Nautilus Chamber Growth

| Type/Method | Description |
|------|-------------|
| `NautilusChamber::new(growth_factor)` | Create shell |
| `.add_chamber()` | Grow one chamber (pressure-determined) |
| `.add_chambers(n)` | Grow n chambers |
| `.current_pressure()` | Gas pressure for next chamber |
| `.total_volume()` | Sum of all chamber volumes |
| `Chamber` | index, inner/outer radius, volume, gas_pressure, wall_thickness, angle_span |

### `fractal` — Fractal Self-Similarity

| Type/Method | Description |
|------|-------------|
| `FractalZoom::new(spiral)` | Create from a base spiral |
| `.fractal_dimension()` | D ≈ 1 + b²/(1+b²) for logarithmic spiral |
| `.zoom(center_θ, scale, n)` | Zoom into a region |
| `.verify_self_similarity()` | Compare curvature stats across zoom levels |

### `growth_rate` — Golden Growth Rate Analysis

| Type/Method | Description |
|------|-------------|
| `GoldenGrowthRate::new()` | Create analyzer |
| `.continued_fraction(x, depth)` | CF decomposition |
| `.phi_continued_fraction()` | CF of φ: all 1s (slowest convergence) |
| `.verify_phi_ones()` | Verify φ's CF is all 1s |
| `.convergents(depth)` | Successive CF convergents → φ |
| `.irrationality_ranking(numbers)` | Rank numbers by irrationality measure |
| `ContinuedFraction` | terms, value |

### `quasicrystal` — Aperiodic Deterministic Growth

| Type/Method | Description |
|------|-------------|
| `QuasicrystalGrowth::new()` | Create engine |
| `.generate_1d(n, slope)` | Cut-and-project from 2D square lattice |
| `.fibonacci_word(n)` | Substitution: 0→01, 1→0 |
| `.penrose_decompose(depth)` | Penrose-like rhomb subdivision |
| `.is_aperiodic()` | Verify no translational period |
| `Tile` | index, vertices, tile_type, orientation, scale |

### `agent_shell` — Agent Capability Shells

| Type/Method | Description |
|------|-------------|
| `AgentShell::new(growth_rate)` | Create shell |
| `.add_skill(name)` | Grow a skill at curvature-determined position |
| `.manifold_curvature(angle)` | Local curvature of capability manifold |
| `.next_growth_direction()` | Predict where the next skill should emerge |
| `.skill_density()` | Skills per unit capability area |
| `Skill` | name, radius, angle, curvature, connections, depth |
| `GrowthDirection` | angle, expected_curvature, suggested_skill, confidence |

### `ricci_flow` — Ricci Flow Growth

| Type/Method | Description |
|------|-------------|
| `RicciFlowGrowth::new(dt, alpha)` | Create flow simulation |
| `.init_perturbed_circle(n, perturbation)` | Seed with perturbed circle |
| `.flow(n_steps)` | Run n steps of curvature-driven flow |
| `.curvature_variance()` | How uniform the curvature is |
| `.is_uniform(tol)` | Check if curvature is approximately uniform |
| `MeshNode` | index, position, curvature, velocity |

---

## How It Works

### Growth = Computation

Each module instantiates the same principle from a different angle:

1. **Spiral growth**: The logarithmic spiral r = a·e^(bθ) is determined entirely by two parameters. Once set, every point follows. The golden spiral (b = ln(φ)/(π/2)) is the special case where growth per quarter turn equals φ.

2. **Curvature inheritance**: Each growth ring's curvature κ_{n+1} = damping·κ_n + base_rate/(1+κ_n) is a function of the previous ring only. The shell doesn't plan ahead — it responds to local geometry.

3. **Temporal recording**: Growth maps time to space. Ring index = time index. Examining the spatial structure reconstructs the temporal history. This is Mandelbrot's insight: the geometry records the dynamics.

4. **Phyllotaxis**: The golden angle 360°/φ² ≈ 137.508° produces the most uniform point distribution on a circle. No rational angle can do this — rational angles always produce periodic patterns with gaps.

### The Ricci Flow Connection

The Ricci flow ∂g/∂t = -2Ric(g) smooths curvature across a manifold. Applied to shell growth:

- Perturbations in the initial shape are smoothed over time
- The flow converges toward uniform curvature
- Growth IS curvature equalization

This is the same flow used in Perelman's proof of the Poincaré conjecture, here applied to biological growth dynamics.

---

## The Math

### Logarithmic Spiral Properties

For r(θ) = a·e^(bθ):

- **Constant angle** α = arctan(1/b) between radius and tangent
- **Curvature** κ(θ) = sin(α) / r(θ) = b / (r(θ)·√(1+b²))
- **Arc length** L(θ₁,θ₂) = √(1+b²)·(r(θ₂)-r(θ₁))/b
- **Self-similarity**: r(θ + 2π/b) = e^{2π}·r(θ) — rotating by 2π/b scales by e^{2π}

### Golden Ratio as Most Irrational

The Hurwitz theorem states: for any irrational x, infinitely many p/q satisfy |x - p/q| < 1/(√5·q²). The constant √5 is optimal and achieved only by φ.

The continued fraction [1; 1, 1, 1, ...] has the worst convergence rate among all continued fractions. This means φ "resists" rational approximation most strongly, producing the most uniform angular distribution.

### Cut-and-Project Quasicrystals

A 1D quasicrystal is constructed by:

1. Take a 2D square lattice Z²
2. Draw a line at irrational slope α through the origin
3. Project lattice points within a strip around this line onto the line

The resulting 1D pattern is aperiodic (never repeats) but deterministic (fully determined by α). For α = φ, the intervals come in two lengths L and S with ratio L/S = φ.

### Ricci Flow on Discrete Curves

For a discrete curve with points p₁, ..., pₙ, the discrete Laplacian gives:

```
κ_i = (p_{i-1} + p_{i+1} - 2p_i) · n / h²
```

The flow moves each point proportional to its curvature excess:

```
dp_i/dt = -α · (κ_i - κ̄) · n_i
```

This converges to a circle (uniform curvature) as t → ∞.

---

## License

MIT
