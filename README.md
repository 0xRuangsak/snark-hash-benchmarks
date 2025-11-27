# Ethereum Hash Function Comparison

A Rust-based benchmarking tool that compares traditional cryptographic hash functions (SHA-256, Keccak-256) with SNARK-friendly hash functions (Poseidon) in the context of Ethereum and zero-knowledge proof systems.

## Overview

This project demonstrates the critical trade-offs between computational performance and zero-knowledge proof efficiency when choosing hash functions for blockchain applications.

## Features

- **Performance Benchmarking**: Measures execution time for SHA-256, Keccak-256, and Poseidon
- **SNARK Constraint Analysis**: Compares circuit complexity for zero-knowledge proofs
- **Use Case Recommendations**: Explains when to use each hash function
- **Summary Comparison**: Visual table of key metrics

## Installation

### Prerequisites
- Rust (1.70+)
- Cargo

### Setup
```bash
git clone https://github.com/yourusername/ethereum-hash-comparison.git
cd ethereum-hash-comparison
cargo build --release
```

## Usage

Simply run:
```bash
cargo run --release
```

Output:
```
======================================================================
    Ethereum Hash Function Comparison Framework
======================================================================

Comparing traditional vs SNARK-friendly hash functions
Iterations: 1000

>>> 1. Performance Benchmarks
----------------------------------------------------------------------
  SHA-256    => 7 ms (7 μs per hash)
  Keccak-256 => 31 ms (31 μs per hash)
  Poseidon   => 31834 ms (31834 μs per hash)

>>> 2. SNARK Constraint Estimates
----------------------------------------------------------------------
  SHA-256    => ~ 25000 constraints
  Keccak-256 => ~150000 constraints
  Poseidon   => ~   100 constraints (250x better!)
```

## Key Findings

- **SHA-256**: Fast (7 μs/hash) but expensive in zkSNARKs (~25,000 constraints)
- **Keccak-256**: Ethereum-native, moderate speed (31 μs/hash), very expensive in zkSNARKs (~150,000 constraints)
- **Poseidon**: Slow to compute (31,834 μs/hash) but extremely efficient in zkSNARKs (~100 constraints)

## Why?

Zero-knowledge proof systems (zkSNARKs) are crucial for Ethereum's scaling roadmap, including:
- Rollups (zkSync, Scroll, Polygon zkEVM)
- Privacy applications (Tornado Cash alternatives)
- Stateless clients

Traditional hash functions like SHA-256 and Keccak-256 require tens of thousands of constraints in zkSNARK circuits, making proofs slow and expensive. SNARK-friendly hashes like Poseidon reduce constraints by 250x, enabling practical zero-knowledge applications.

## Technical Details

### Hash Functions Implemented
- **SHA-256**: Standard cryptographic hash (Bitcoin, TLS)
- **Keccak-256**: Ethereum's primary hash function (EVM opcode)
- **Poseidon**: Algebraic hash designed for arithmetic circuits

### Dependencies
- `sha2` - SHA-256 implementation
- `tiny-keccak` - Keccak-256 implementation
- `neptune` - Poseidon hash implementation
- `blstrs` - BLS12-381 curve operations
- `ff` - Finite field arithmetic

## Related Research

- [Poseidon: A New Hash Function for Zero-Knowledge Proof Systems](https://eprint.iacr.org/2019/458)
- [Ethereum Foundation: Cryptography Research](https://protocol.ethereum.foundation/)

## License

MIT