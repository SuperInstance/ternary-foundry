#![forbid(unsafe_code)]

//! Casting and forging ternary strategies from raw materials.
//!
//! This crate models strategy refinement as metallurgical processes:
//! melting down old strategies, casting new ones from molds, forging under
//! pressure, testing durability on the anvil, blending alloys, and hardening
//! through controlled tempering.

use std::collections::HashMap;

/// Ternary value: -1, 0, or +1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        self as i8
    }
}

/// A strategy is a named mapping from situation keys to ternary decisions.
#[derive(Clone, Debug, PartialEq)]
pub struct Strategy {
    pub name: String,
    pub decisions: HashMap<String, Ternary>,
}

impl Strategy {
    pub fn new(name: &str) -> Self {
        Strategy {
            name: name.to_string(),
            decisions: HashMap::new(),
        }
    }

    pub fn with(mut self, key: &str, value: Ternary) -> Self {
        self.decisions.insert(key.to_string(), value);
        self
    }

    pub fn decision(&self, key: &str) -> Option<Ternary> {
        self.decisions.get(key).copied()
    }
}

/// The foundry: central hub for strategy refinement operations.
#[derive(Debug)]
pub struct Foundry {
    strategies: Vec<Strategy>,
}

impl Foundry {
    pub fn new() -> Self {
        Foundry { strategies: Vec::new() }
    }

    pub fn register(&mut self, strategy: Strategy) {
        self.strategies.push(strategy);
    }

    pub fn find(&self, name: &str) -> Option<&Strategy> {
        self.strategies.iter().find(|s| s.name == name)
    }

    pub fn find_mut(&mut self, name: &str) -> Option<&mut Strategy> {
        self.strategies.iter_mut().find(|s| s.name == name)
    }

    pub fn list(&self) -> &[Strategy] {
        &self.strategies
    }

    pub fn count(&self) -> usize {
        self.strategies.len()
    }

    pub fn remove(&mut self, name: &str) -> Option<Strategy> {
        if let Some(pos) = self.strategies.iter().position(|s| s.name == name) {
            Some(self.strategies.remove(pos))
        } else {
            None
        }
    }
}

impl Default for Foundry {
    fn default() -> Self {
        Self::new()
    }
}

/// A template that defines the shape (keys) a strategy should have.
#[derive(Clone, Debug, PartialEq)]
pub struct Mold {
    pub name: String,
    pub keys: Vec<String>,
    pub default: Ternary,
}

impl Mold {
    pub fn new(name: &str, keys: &[&str]) -> Self {
        Mold {
            name: name.to_string(),
            keys: keys.iter().map(|s| s.to_string()).collect(),
            default: Ternary::Zero,
        }
    }

    pub fn with_default(mut self, default: Ternary) -> Self {
        self.default = default;
        self
    }

    /// Cast a new strategy from this mold, filling all keys with the default value.
    pub fn cast(&self, strategy_name: &str) -> Strategy {
        let mut decisions = HashMap::new();
        for key in &self.keys {
            decisions.insert(key.clone(), self.default);
        }
        Strategy {
            name: strategy_name.to_string(),
            decisions,
        }
    }
}

/// Melts down an existing strategy into raw materials (key-value pairs).
#[derive(Debug)]
pub struct Crucible;

impl Crucible {
    pub fn melt(strategy: Strategy) -> Vec<(String, Ternary)> {
        strategy.decisions.into_iter().collect()
    }

    pub fn melt_filtered(strategy: Strategy, filter: Ternary) -> Vec<(String, Ternary)> {
        strategy
            .decisions
            .into_iter()
            .filter(|(_, v)| *v == filter)
            .collect()
    }

    /// Melt multiple strategies, combining their decisions (last wins).
    pub fn melt_all(strategies: Vec<Strategy>) -> HashMap<String, Ternary> {
        let mut combined = HashMap::new();
        for s in strategies {
            for (k, v) in s.decisions {
                combined.insert(k, v);
            }
        }
        combined
    }
}

/// Shapes a strategy under pressure by applying transformations.
#[derive(Debug)]
pub struct Forge;

impl Forge {
    /// Apply a transformation: negate all decisions.
    pub fn negate(strategy: &Strategy) -> Strategy {
        let mut forged = Strategy::new(&format!("forged_{}", strategy.name));
        for (key, val) in &strategy.decisions {
            let negated = match val {
                Ternary::Neg => Ternary::Pos,
                Ternary::Zero => Ternary::Zero,
                Ternary::Pos => Ternary::Neg,
            };
            forged.decisions.insert(key.clone(), negated);
        }
        forged
    }

    /// Apply pressure: override specific decisions.
    pub fn override_decisions(strategy: &Strategy, overrides: &HashMap<String, Ternary>) -> Strategy {
        let mut forged = strategy.clone();
        forged.name = format!("forged_{}", strategy.name);
        for (key, val) in overrides {
            forged.decisions.insert(key.clone(), *val);
        }
        forged
    }

    /// Zero out all decisions.
    pub fn flatten(strategy: &Strategy) -> Strategy {
        let mut forged = Strategy::new(&format!("forged_{}", strategy.name));
        for key in strategy.decisions.keys() {
            forged.decisions.insert(key.clone(), Ternary::Zero);
        }
        forged
    }
}

/// Tests strategy durability by measuring stability under perturbation.
#[derive(Debug)]
pub struct Anvil;

impl Anvil {
    /// Count how many decisions would change if a specific key is flipped.
    pub fn impact_count(strategy: &Strategy, key: &str) -> usize {
        // If flipping one decision causes others to be re-evaluated,
        // this measures the cascade. For now: simple presence check.
        if strategy.decisions.contains_key(key) { 1 } else { 0 }
    }

    /// Test if a strategy has complete coverage of a mold's keys.
    pub fn completeness(strategy: &Strategy, mold: &Mold) -> f64 {
        if mold.keys.is_empty() {
            return 1.0;
        }
        let covered = mold.keys.iter().filter(|k| strategy.decisions.contains_key(*k)).count();
        covered as f64 / mold.keys.len() as f64
    }

    /// Measure strategy density: ratio of non-zero decisions to total.
    pub fn density(strategy: &Strategy) -> f64 {
        if strategy.decisions.is_empty() {
            return 0.0;
        }
        let non_zero = strategy.decisions.values().filter(|v| **v != Ternary::Zero).count();
        non_zero as f64 / strategy.decisions.len() as f64
    }

    /// Test structural integrity: no empty decisions map.
    pub fn is_sound(strategy: &Strategy) -> bool {
        !strategy.decisions.is_empty()
    }
}

/// Blend two strategies into an alloy. When keys overlap, uses the blender function.
#[derive(Debug)]
pub struct Alloy;

impl Alloy {
    /// Blend by majority vote when keys overlap.
    pub fn blend_majority(a: &Strategy, b: &Strategy, name: &str) -> Strategy {
        Self::blend(a, b, name, |va, vb| {
            let sum = va.to_i8() + vb.to_i8();
            if sum > 0 { Ternary::Pos }
            else if sum < 0 { Ternary::Neg }
            else { Ternary::Zero }
        })
    }

    /// Blend by preferring the first strategy on conflict.
    pub fn blend_prefer_first(a: &Strategy, b: &Strategy, name: &str) -> Strategy {
        Self::blend(a, b, name, |va, _| va)
    }

    /// Generic blend with a custom resolver function.
    pub fn blend<F>(a: &Strategy, b: &Strategy, name: &str, resolver: F) -> Strategy
    where
        F: Fn(Ternary, Ternary) -> Ternary,
    {
        let mut result = Strategy::new(name);
        // Add all from a
        for (key, val) in &a.decisions {
            result.decisions.insert(key.clone(), *val);
        }
        // Merge b
        for (key, val_b) in &b.decisions {
            if let Some(val_a) = result.decisions.get(key) {
                result.decisions.insert(key.clone(), resolver(*val_a, *val_b));
            } else {
                result.decisions.insert(key.clone(), *val_b);
            }
        }
        result
    }
}

/// Hardens a strategy by applying controlled stress (iteration of transformations).
#[derive(Debug)]
pub struct Temper {
    pub iterations: usize,
}

impl Temper {
    pub fn new(iterations: usize) -> Self {
        Temper { iterations }
    }

    /// Harden by repeatedly applying a closure that modifies the strategy.
    pub fn harden<F>(&self, strategy: &Strategy, stress: F) -> Strategy
    where
        F: Fn(&Strategy) -> Strategy,
    {
        let mut current = strategy.clone();
        for _ in 0..self.iterations {
            current = stress(&current);
        }
        current
    }

    /// Stress test: flip random decisions based on a predicate.
    pub fn stress_flip(strategy: &Strategy, predicate: impl Fn(&str) -> bool) -> Strategy {
        let mut hardened = Strategy::new(&format!("tempered_{}", strategy.name));
        for (key, val) in &strategy.decisions {
            let new_val = if predicate(key) {
                match val {
                    Ternary::Neg => Ternary::Pos,
                    Ternary::Zero => Ternary::Pos,
                    Ternary::Pos => Ternary::Zero,
                }
            } else {
                *val
            };
            hardened.decisions.insert(key.clone(), new_val);
        }
        hardened
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_strategy() -> Strategy {
        Strategy::new("test")
            .with("attack", Ternary::Pos)
            .with("defend", Ternary::Neg)
            .with("wait", Ternary::Zero)
    }

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(Ternary::from_i8(-1), Some(Ternary::Neg));
        assert_eq!(Ternary::from_i8(0), Some(Ternary::Zero));
        assert_eq!(Ternary::from_i8(1), Some(Ternary::Pos));
        assert_eq!(Ternary::from_i8(2), None);
    }

    #[test]
    fn test_strategy_create_and_access() {
        let s = sample_strategy();
        assert_eq!(s.decision("attack"), Some(Ternary::Pos));
        assert_eq!(s.decision("defend"), Some(Ternary::Neg));
        assert_eq!(s.decision("wait"), Some(Ternary::Zero));
        assert_eq!(s.decision("unknown"), None);
    }

    #[test]
    fn test_foundry_register_and_find() {
        let mut f = Foundry::new();
        f.register(sample_strategy());
        assert_eq!(f.count(), 1);
        assert!(f.find("test").is_some());
        assert!(f.find("missing").is_none());
    }

    #[test]
    fn test_foundry_remove() {
        let mut f = Foundry::new();
        f.register(sample_strategy());
        let removed = f.remove("test");
        assert!(removed.is_some());
        assert_eq!(f.count(), 0);
    }

    #[test]
    fn test_foundry_list() {
        let mut f = Foundry::new();
        f.register(Strategy::new("a"));
        f.register(Strategy::new("b"));
        assert_eq!(f.list().len(), 2);
    }

    #[test]
    fn test_mold_cast() {
        let mold = Mold::new("warrior", &["attack", "defend", "retreat"]);
        let strategy = mold.cast("my_warrior");
        assert_eq!(strategy.decisions.len(), 3);
        assert_eq!(strategy.decision("attack"), Some(Ternary::Zero));
        assert_eq!(strategy.name, "my_warrior");
    }

    #[test]
    fn test_mold_with_default() {
        let mold = Mold::new("aggressive", &["charge", "slam"])
            .with_default(Ternary::Pos);
        let s = mold.cast("berzerker");
        assert_eq!(s.decision("charge"), Some(Ternary::Pos));
    }

    #[test]
    fn test_crucible_melt() {
        let s = sample_strategy();
        let raw = Crucible::melt(s);
        assert_eq!(raw.len(), 3);
    }

    #[test]
    fn test_crucible_melt_filtered() {
        let s = sample_strategy();
        let raw = Crucible::melt_filtered(s, Ternary::Pos);
        assert_eq!(raw.len(), 1);
        assert_eq!(raw[0].0, "attack");
    }

    #[test]
    fn test_crucible_melt_all() {
        let a = Strategy::new("a").with("x", Ternary::Pos);
        let b = Strategy::new("b").with("y", Ternary::Neg);
        let combined = Crucible::melt_all(vec![a, b]);
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn test_forge_negate() {
        let s = sample_strategy();
        let neg = Forge::negate(&s);
        assert_eq!(neg.decision("attack"), Some(Ternary::Neg));
        assert_eq!(neg.decision("defend"), Some(Ternary::Pos));
        assert_eq!(neg.decision("wait"), Some(Ternary::Zero));
    }

    #[test]
    fn test_forge_override() {
        let s = sample_strategy();
        let mut overrides = HashMap::new();
        overrides.insert("attack".to_string(), Ternary::Neg);
        let forged = Forge::override_decisions(&s, &overrides);
        assert_eq!(forged.decision("attack"), Some(Ternary::Neg));
        assert_eq!(forged.decision("defend"), Some(Ternary::Neg)); // unchanged
    }

    #[test]
    fn test_forge_flatten() {
        let s = sample_strategy();
        let flat = Forge::flatten(&s);
        assert!(flat.decisions.values().all(|v| *v == Ternary::Zero));
    }

    #[test]
    fn test_anvil_completeness() {
        let mold = Mold::new("m", &["a", "b", "c"]);
        let s = Strategy::new("s").with("a", Ternary::Pos).with("b", Ternary::Neg);
        let comp = Anvil::completeness(&s, &mold);
        assert!((comp - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_anvil_completeness_full() {
        let mold = Mold::new("m", &["a", "b"]);
        let s = Strategy::new("s").with("a", Ternary::Pos).with("b", Ternary::Neg);
        assert_eq!(Anvil::completeness(&s, &mold), 1.0);
    }

    #[test]
    fn test_anvil_density() {
        let s = sample_strategy();
        let d = Anvil::density(&s);
        assert!((d - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_anvil_sound() {
        assert!(Anvil::is_sound(&sample_strategy()));
        assert!(!Anvil::is_sound(&Strategy::new("empty")));
    }

    #[test]
    fn test_alloy_blend_majority() {
        let a = Strategy::new("a").with("x", Ternary::Pos);
        let b = Strategy::new("b").with("x", Ternary::Pos);
        let alloy = Alloy::blend_majority(&a, &b, "ab");
        assert_eq!(alloy.decision("x"), Some(Ternary::Pos));
    }

    #[test]
    fn test_alloy_blend_prefer_first() {
        let a = Strategy::new("a").with("x", Ternary::Pos);
        let b = Strategy::new("b").with("x", Ternary::Neg);
        let alloy = Alloy::blend_prefer_first(&a, &b, "ab");
        assert_eq!(alloy.decision("x"), Some(Ternary::Pos));
    }

    #[test]
    fn test_temper_harden() {
        let s = Strategy::new("s").with("x", Ternary::Pos);
        let temper = Temper::new(3);
        let hardened = temper.harden(&s, |strat| Forge::negate(strat));
        // After 3 negations of Pos: Neg -> Pos -> Neg
        assert_eq!(hardened.decision("x"), Some(Ternary::Neg));
    }

    #[test]
    fn test_temper_stress_flip() {
        let s = Strategy::new("s").with("flip_me", Ternary::Pos).with("keep", Ternary::Neg);
        let stressed = Temper::stress_flip(&s, |key| key == "flip_me");
        assert_eq!(stressed.decision("flip_me"), Some(Ternary::Zero));
        assert_eq!(stressed.decision("keep"), Some(Ternary::Neg));
    }
}
