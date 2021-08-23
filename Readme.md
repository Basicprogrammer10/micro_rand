# 🎲 micro_rand

A tiny, no STD library for generating (pseudo) random numbers.

## 💠 Install

Just add the following to your `Cargo.toml`:

```toml
[dependencies]
micro_rand = "0.0.1"
```

## 📄 Info

Uses the Linear congruential generator algorithm to generate pseudo random numbers. You must supply a seed value in the form of a `i64` integer.

## 💥 Examples
Super Simple Example
```rust
// Import Lib
use micro_rand::*;

// Create a new random generator with seed 1234
let mut rnd = Random::new(1234);

// Generate a Number!
let num1 = rnd.next_f64();

// Generate an int between two values
let num2 = rnd.next_int_i64(0, 100);
```