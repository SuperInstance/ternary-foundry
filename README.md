# Ternary Foundry — Forging and Refining Ternary Strategies

**Ternary Foundry** models strategy creation and refinement as metallurgical processes: melting down old strategies, casting new ones from molds, forging under pressure, testing on the anvil, blending alloys, and hardening through tempering. Each operation transforms ternary strategy vectors {-1, 0, +1} to produce improved strategies.

## Why It Matters

Strategy optimization in ternary systems is more than gradient descent — it requires creative operations that explore the strategy space in structured ways. The foundry metaphor captures real optimization techniques: **melting** decomposes a strategy into components for analysis; **casting** instantiates a template with specific values; **forging** applies pressure (constraints) to reshape strategies; **alloying** blends strategies from different lineages; **hardening** increases commitment to successful patterns. These operations are especially natural for ternary strategies, where the small alphabet {-1, 0, +1} means every operation is a controlled perturbation.

## How It Works

### Strategy Representation

A `Strategy` is a named mapping from string keys to ternary decisions (`HashMap<String, Ternary>`). Each key represents a decision point, and the ternary value encodes the chosen action: +1 (do), 0 (defer), -1 (don't).

### Foundry Operations

- **Melt**: Decompose a strategy into individual decisions for inspection. O(k) for k decisions.
- **Cast**: Create a new strategy from a template (mold) by filling in specific values. O(k).
- **Forge**: Apply pressure — given a strategy and a set of constraints, modify decisions that violate constraints. O(k · c) for c constraints.
- **Anvil (Test)**: Evaluate a strategy against a test suite and measure durability (consistency across scenarios). O(k · t) for t tests.
- **Alloy (Blend)**: Combine two parent strategies. For each decision key, if both parents agree → that value; if they disagree → 0 (neutral). O(k).
- **Harden (Temper)**: Given a strategy and its performance history, increase confidence: decisions that were consistently successful are reinforced toward ±1, uncertain decisions drift toward 0. O(k).

### Foundry Hub

The `Foundry` struct manages a collection of registered strategies and provides lookup, modification, and batch processing. It acts as the central workspace for strategy development.

## Quick Start

```rust
use ternary_foundry::{Strategy, Ternary, Foundry};

// Create strategies
let offensive = Strategy::new("offensive")
    .with("attack", Ternary::Pos)
    .with("defend", Ternary::Neg)
    .with("scout", Ternary::Pos);

let defensive = Strategy::new("defensive")
    .with("attack", Ternary::Neg)
    .with("defend", Ternary::Pos)
    .with("scout", Ternary::Zero);

// Register in foundry
let mut foundry = Foundry::new();
foundry.register(offensive);
foundry.register(defensive);

// Look up a strategy
if let Some(s) = foundry.find("offensive") {
    println!("Attack decision: {:?}", s.decision("attack"));
}
```

```bash
cargo add ternary-foundry
```

## API

| Type / Function | Description |
|---|---|
| `Ternary` | `Neg(-1)`, `Zero(0)`, `Pos(+1)` |
| `Strategy` | Named HashMap of decisions: `with(key, value)`, `decision(key)` |
| `Foundry` | Strategy registry: `register()`, `find()`, `find_mut()` |

## Architecture Notes

The foundry is the strategy development environment in **SuperInstance**. Agents forge strategies through the foundry's operations, and successful strategies are deployed to the fleet. The γ + η = C conservation law is enforced by the hardening operation: strategies that push too hard on γ (aggressive growth) accumulate η cost (fragility), and tempering balances the two. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Holland, John H. *Adaptation in Natural and Artificial Systems*, MIT Press, 1992 — genetic algorithms and recombination.
- Mitchell, Melanie. *An Introduction to Genetic Algorithms*, MIT Press, 1996 — crossover and mutation operators.
- Goldberg, David E. *Genetic Algorithms in Search, Optimization, and Machine Learning*, Addison-Wesley, 1989.

## License

MIT
