use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use secp256k1::{Secp256k1, SecretKey};
use web3::types::Address;
use web3::signing::keccak256;
use hex;
use std::fs;
use serde::Deserialize;
use std::time::Instant;
use chrono;
use std::io::Write;
use std::fs::OpenOptions;

// Function to generate a random private key
fn generate_private_key() -> SecretKey {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    SecretKey::from_slice(&key).expect("Invalid private key")
}

// Function to compute Ethereum address from a private key
fn private_key_to_address(secret_key: &SecretKey) -> Address {
    let secp = Secp256k1::new();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, secret_key);
    let public_key_bytes = &public_key.serialize_uncompressed()[1..];
    let hash = keccak256(public_key_bytes);
    Address::from_slice(&hash[12..])
}

// Main function to find the vanity address
fn find_vanity_address(target_prefix: &str, target_suffix: &str, threads: usize) -> (SecretKey, Address) {
    let found = Arc::new(AtomicBool::new(false));
    let result = Arc::new(Mutex::new(None));
    let start_time = Instant::now();
    let attempts = Arc::new(AtomicUsize::new(0));

    // Calculate the probability of finding the address
    let prefix_len = target_prefix.len();
    let suffix_len = target_suffix.len();
    // Each hex character represents 4 bits, so we calculate probability as 1/16^(total_chars)
    let required_chars = prefix_len + suffix_len;
    let probability = 1.0 / (16.0_f64.powi(required_chars as i32));

    // Thread to print speed and estimated time every second
    let attempts_clone = Arc::clone(&attempts);
    let found_clone = Arc::clone(&found);
    thread::spawn(move || {
        let expected_attempts = 1.0 / probability;
        
        println!("Expected number of attempts: {:.0}", expected_attempts);
        
        while !found_clone.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_secs(1));
            let elapsed = start_time.elapsed();
            let total_attempts = attempts_clone.load(Ordering::Relaxed);
            let speed = total_attempts as f64 / elapsed.as_secs_f64();
            let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let estimated_remaining_time = if speed > 0.0 {
                (expected_attempts - total_attempts as f64) / speed / 60.0
            } else {
                f64::INFINITY
            };
            
            print!("\r{}", " ".repeat(100)); // Clear previous line
            print!("\r[{}] Speed: {:.2} addr/s, Remaining: {:.2} min", 
                current_time, speed, estimated_remaining_time
            );
            std::io::stdout().flush().unwrap();
        }
    });

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let found = Arc::clone(&found);
            let result = Arc::clone(&result);
            let target_prefix = target_prefix.to_lowercase();
            let target_suffix = target_suffix.to_lowercase();
            let attempts = Arc::clone(&attempts);

            thread::spawn(move || {
                while !found.load(Ordering::Relaxed) {
                    let private_key = generate_private_key();
                    let address = private_key_to_address(&private_key);
                    attempts.fetch_add(1, Ordering::Relaxed);

                    let address_str = format!("{:x}", address).to_lowercase();
                    if address_str.starts_with(&target_prefix) && address_str.ends_with(&target_suffix) {
                        if !found.swap(true, Ordering::Relaxed) {
                            let mut result_lock = result.lock().unwrap();
                            *result_lock = Some((private_key, address));
                        }
                        break;
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread finished with an error");
    }

    let result = result.lock().unwrap();
    result.clone().expect("Failed to find address")
}

#[derive(Deserialize)]
struct Settings {
    target: TargetSettings,
}

#[derive(Deserialize)]
struct TargetSettings {
    prefix: String,
    suffix: String,
    threads: usize,
    addresses_count: usize,
}

// Add this new function
fn to_checksum_address(address: Address) -> String {
    let address_hex = hex::encode(address.as_bytes());
    let address_hash = hex::encode(keccak256(address_hex.as_bytes()));
    
    let mut checksum = String::with_capacity(42);
    checksum.push_str("0x");
    
    for (i, ch) in address_hex.chars().enumerate() {
        if ch >= 'a' && ch <= 'f' {
            if address_hash.chars().nth(i).unwrap() >= '8' {
                checksum.push(ch.to_ascii_uppercase());
            } else {
                checksum.push(ch);
            }
        } else {
            checksum.push(ch);
        }
    }
    
    checksum
}

fn main() {
    let settings_content = fs::read_to_string("settings.toml").expect("Failed to read settings.toml");
    let settings: Settings = toml::from_str(&settings_content).expect("Failed to parse settings.toml");

    let target_prefix = settings.target.prefix;
    let target_suffix = settings.target.suffix;
    let threads = settings.target.threads;
    let addresses_count = settings.target.addresses_count;

    println!("Looking for {} addresses like: 0x{}[random]{}", 
        addresses_count, target_prefix, target_suffix);
    println!("Starting address search...\n");
    
    for i in 1..=addresses_count {
        println!("Searching for address {}/{}", i, addresses_count);
        let (private_key, address) = find_vanity_address(&target_prefix, &target_suffix, threads);

        print!("\r{}", " ".repeat(100)); // Clear the progress line
        let checksum_address = to_checksum_address(address);
        println!("\rFound address: {}", checksum_address);
        
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("address_key_pair.txt")
            .expect("Failed to open file");

        writeln!(file, "\n[{}]\nAddress: {}\nPrivate Key: 0x{}\n", 
            timestamp, checksum_address, hex::encode(private_key.as_ref()))
            .expect("Failed to write to file");
            
        println!("Address-key pair saved to file!");
        
        if i < addresses_count {
            println!(); // Add newline only between addresses, not after the last one
        }
    }
}
