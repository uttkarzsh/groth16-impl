# Groth16 Implementation in Rust

An implementation of the **Groth16 zk-SNARK protocol** in Rust.  
This project follows the algorithm end-to-end: from **R1CS constraints**, to **QAP construction**, to **trusted setup**, **proof generation**, and **verification**.

The implementation is intentionally explicit and low-level. Nothing is hidden behind DSLs or circuit compilers. If you want to understand how Groth16 actually works under the hood, this code is meant to be read.

---

## Cryptographic Stack

This implementation uses the following `arkworks` libraries:

- `ark-bn254` — BN254 pairing-friendly elliptic curve
- `ark-ff` — finite field arithmetic
- `ark-ec` — elliptic curve and pairing abstractions

All group operations, pairings, and field arithmetic are handled explicitly through these libraries.

---

## Features

- **Proper Groth16 implementation**
  - Produces a valid proof for a correct witness
  - Rejects proofs generated from an invalid witness

- **Full pipeline**
  - R1CS (matrix representation of constraints)
  - QAP construction
  - Polynomial evaluations
  - Encoding into elliptic curve group elements
  - Proof generation and verification

- **Proper trusted setup**
  - Includes toxic waste parameters
  - Uses structured reference strings derived from a secret τ

- **Soundness and zero-knowledge**
  - **α, β**: prevent forged proofs
  - **γ, δ**: enforce public input consistency
  - **r, s**: proof randomization for true zero-knowledge

- **Flexible R1CS**
  - Manually define **any valid R1CS matrix of arbitrary size**
  - Control:
    - `N` → number of rows
    - `M` → number of columns
    - `L` → number of public inputs

- **Minimal interface**
  - Define constraints in `r1cs.rs`
  - Define witness values in `witness.rs`
  - Run with `cargo run`

---

