# ternary-foundry

**Strategy refinement as metallurgy. Melt, cast, forge, temper, test.**

Where do strategies come from? You don't just *have* one — you build it. This crate models strategy creation as a metallurgical process: start with raw materials (ternary decisions), melt them down, cast them into molds (templates), forge them under pressure (competition), temper them (controlled stress), and test them on the anvil.

A strategy is a mapping from situation keys to ternary decisions: `{-1, 0, +1}`. Simple. But the *process* of creating a good strategy — that's where the foundry comes in.

## What's Inside

- **`Strategy`** — a named mapping from `String` keys to `Ternary` decisions. Build with `.with("attack", Pos).with("defend", Neg)`
- **`Mold`** — a template for casting new strategies. Defines required decision keys with default values
- **`Foundry`** — the central hub. Melts strategies, casts from molds, forges through competition
- **`alloy(strategies)`** — blend multiple strategies into a consensus. Majority vote per key
- **`forge(strategy, pressure)`** — apply competitive pressure: flip weak decisions toward the pressure direction
- **`temper(strategy, rounds)`** — controlled stress: random perturbation with stability bias
- **`anvil_test(strategy, scenarios)`** — score a strategy against a battery of test scenarios

## Quick Example

```rust
use ternary_foundry::*;

// Define a mold (template)
let mold = Mold::new("combat")
    .require("attack", Ternary::Zero)
    .require("defend", Ternary::Zero)
    .require("retreat", Ternary::Neg);

// Cast a strategy from the mold
let base = mold.cast();

// Create variants
let aggressive = Strategy::new("berserker")
    .with("attack", Ternary::Pos)
    .with("defend", Ternary::Neg)
    .with("retreat", Ternary::Neg);

let cautious = Strategy::new("turtle")
    .with("attack", Ternary::Neg)
    .with("defend", Ternary::Pos)
    .with("retreat", Ternary::Zero);

// Alloy: blend strategies
let blended = alloy(&[aggressive.clone(), cautious.clone()]);
// Majority vote per key: attack=Zero, defend=Zero, retreat=Neg

// Forge under pressure: push toward aggressive
let forged = forge(aggressive, Ternary::Pos);
```

## The Insight

**Strategy creation is a *process*, not a snapshot.** The best strategy isn't discovered — it's refined. Melting breaks down existing strategies into components. Casting builds new ones from templates. Forging adapts them to conditions. Tempering makes them robust. Testing validates them. The metallurgical metaphor isn't just cute — it's *accurate*. Real metallurgy and real strategy development follow the same pattern: raw material → shaped → stressed → refined.

**Use cases:**
- **Game AI** — create and evolve NPC strategies
- **Decision support** — model and refine decision policies
- **Evolutionary algorithms** — the foundry IS an EA, but with better metaphors
- **Organizational design** — model how organizations develop strategies
- **Multi-agent systems** — each agent forges its own strategy in competition

## See Also

- **ternary-auction** — strategies for bidding
- **ternary-game-theory** — strategic interaction analysis
- **ternary-ga** — genetic algorithm optimization of strategies
- **ternary-consensus** — how strategies converge in groups

## Install

```bash
cargo add ternary-foundry
```

## License

MIT
