use sha2::{Sha256, Digest};
use tiny_keccak::{Hasher, Keccak};
use neptune::poseidon::PoseidonConstants;
use blstrs::Scalar as Fr;
use ff::PrimeField;
use std::time::Instant;

const NUM_ITERATIONS: usize = 1000;
const INPUT_DATA: &[u8] = b"This is a test message.";

// Hash function wrappers
fn hash_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn hash_keccak256(data: &[u8]) -> Vec<u8> {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut output);
    output.to_vec()
}

fn hash_poseidon(data: &[u8]) -> Vec<u8> {
    use neptune::Poseidon;
    
    let constants = PoseidonConstants::<Fr, typenum::U2>::new();
    let mut p = Poseidon::<Fr, typenum::U2>::new(&constants);
    
    // Convert input data to field elements (simplified approach)
    // We'll just take the first 31 bytes and convert to a field element
    let mut bytes = [0u8; 32];
    let len = data.len().min(31);
    bytes[1..len+1].copy_from_slice(&data[..len]);
    let input = Fr::from_repr(bytes.into()).unwrap_or_else(|| Fr::from(0u64));
    
    // Input and hash using Poseidon
    p.input(input).unwrap();
    let hash = p.hash();
    
    let mut result = [0u8; 32];
    hash.to_repr().as_ref()[..32].iter().enumerate().for_each(|(i, &b)| result[i] = b);
    result.to_vec()
}

// Benchmark function
fn benchmark_hash<F>(name: &str, hash_fn: F) -> u128 
where
    F: Fn(&[u8]) -> Vec<u8>
{
    let start = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let _ = hash_fn(INPUT_DATA);
    }
    let duration = start.elapsed();
    
    println!("  {} => {} ms ({} μs per hash)", 
             name, 
             duration.as_millis(),
             duration.as_micros() / NUM_ITERATIONS as u128);
    
    duration.as_millis()
}

// SNARK constraint estimates (from literature)
fn get_snark_constraints(hash_type: &str) -> usize {
    match hash_type {
        "SHA-256" => 25_000,
        "Keccak-256" => 150_000,
        "Poseidon" => 100,
        _ => 0,
    }
}

fn print_header() {
    println!("\n{}", "=".repeat(70));
    println!("    Ethereum Hash Function Comparison Framework");
    println!("{}", "=".repeat(70));
    println!("\nComparing traditional vs SNARK-friendly hash functions");
    println!("Iterations: {}", NUM_ITERATIONS);
    println!("{}\n", "=".repeat(70));
}

fn print_section(title: &str) {
    println!("\n>>> {}", title);
    println!("{}", "-".repeat(70));
}

fn print_use_cases() {
    print_section("Use Case Recommendations");
    
    println!("\n  SHA-256:");
    println!("    ✓ General-purpose cryptographic hashing");
    println!("    ✓ Bitcoin and legacy systems");
    println!("    ✗ Not optimized for zkSNARKs (high constraint count)");
    
    println!("\n  Keccak-256:");
    println!("    ✓ Ethereum smart contracts (native opcode)");
    println!("    ✓ Address generation and transaction hashing");
    println!("    ✗ Very expensive in zkSNARKs");
    
    println!("\n  Poseidon:");
    println!("    ✓ Zero-knowledge proof systems");
    println!("    ✓ Rollups and Layer 2 solutions");
    println!("    ✓ Privacy-preserving applications");
    println!("    ✗ Not hardware-accelerated like SHA-256");
}

fn print_summary_table(sha_time: u128, keccak_time: u128, poseidon_time: u128) {
    print_section("Summary Comparison Table");
    
    println!("\n  {:<15} {:<20} {:<20} {:<20}", 
             "Property", "SHA-256", "Keccak-256", "Poseidon");
    println!("  {}", "-".repeat(75));
    println!("  {:<15} {:<20} {:<20} {:<20}", 
             "Speed", 
             format!("{} ms", sha_time),
             format!("{} ms", keccak_time),
             format!("{} ms", poseidon_time));
    println!("  {:<15} {:<20} {:<20} {:<20}", 
             "SNARK Cost",
             "~25,000 constr.",
             "~150,000 constr.",
             "~100 constr.");
    println!("  {:<15} {:<20} {:<20} {:<20}", 
             "Ethereum Use",
             "Legacy systems",
             "Native (EVM)",
             "zkApps/Rollups");
    println!("  {:<15} {:<20} {:<20} {:<20}", 
             "Best For",
             "General purpose",
             "Smart contracts",
             "Zero-knowledge");
    println!();
}

fn main() {
    print_header();
    
    // 1. Performance Benchmarks
    print_section("1. Performance Benchmarks");
    println!();
    let sha_time = benchmark_hash("SHA-256   ", hash_sha256);
    let keccak_time = benchmark_hash("Keccak-256", hash_keccak256);
    let poseidon_time = benchmark_hash("Poseidon  ", hash_poseidon);
    
    // 2. SNARK Constraint Analysis
    print_section("2. SNARK Constraint Estimates");
    println!("\n  (Lower is better for zero-knowledge proofs)\n");
    println!("  SHA-256    => ~{:>6} constraints", get_snark_constraints("SHA-256"));
    println!("  Keccak-256 => ~{:>6} constraints", get_snark_constraints("Keccak-256"));
    println!("  Poseidon   => ~{:>6} constraints (250x better!)", get_snark_constraints("Poseidon"));
    
    // 3. Use Cases
    print_use_cases();
    
    // 4. Summary Table
    print_summary_table(sha_time, keccak_time, poseidon_time);
    
}