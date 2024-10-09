mod core;
mod layers;
use clap::{Arg, Command};

use core::{CryptoPipeline};
use layers::{TimeLock, LocationLock, BiometricLock};
use std::{thread::sleep, time::Duration};

fn main() {
    let matches = Command::new("MDTLE Encryption")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Multi-Dimensional Time-Locked Encryption CLI")
        .arg(
            Arg::new("action")
                .long("action")
                .value_parser(["encrypt", "decrypt"]) // Specify valid values
                .required(true)
                .help("Specify whether to encrypt or decrypt (encrypt, decrypt)"),
        )
        .arg(
            Arg::new("data")
                .long("data")
                .value_parser(clap::value_parser!(String)) // Specify value type
                .required(true)
                .help("The data to encrypt or decrypt"),
        )
        .arg(
            Arg::new("time_lock")
                .long("time-lock")
                .value_parser(clap::value_parser!(u64)) // Specify duration as u64
                .help("Duration in seconds for time lock"),
        )
        .arg(
            Arg::new("location_lock")
                .long("location-lock")
                .num_args(2) // Specify number of expected values
                .value_parser(clap::value_parser!(f64)) // Specify latitude and longitude as f64
                .help("Allowed latitude and longitude for location lock"),
        )
        .arg(
            Arg::new("biometric_lock")
                .long("biometric-lock")
                .value_parser(clap::value_parser!(String)) // Specify value type
                .help("Biometric hash for biometric lock"),
        )
        .get_matches();

    let action = matches.get_one::<String>("action").unwrap();
    let data = matches.get_one::<String>("data").unwrap().as_bytes();

    let mut pipeline = CryptoPipeline::new();

    // Time Lock
    if let Some(time_duration) = matches.get_one::<u64>("time_lock") {
        let duration = Duration::from_secs(*time_duration);
        let time_lock = Box::new(TimeLock::new(duration));
        pipeline.add_layer(time_lock);
    }

    // Location Lock
    if let Some(location_values) = matches.get_many::<f64>("location_lock") {
        let values: Vec<&f64> = location_values.collect();
        let allowed_lat = *values[0]; // Dereference to get the value
        let allowed_lon = *values[1]; // Dereference to get the value
        // Mock current location same as allowed (can be real in production)
        let current_lat = allowed_lat;
        let current_lon = allowed_lon;
        let location_lock = Box::new(LocationLock::new(allowed_lat, allowed_lon, current_lat, current_lon));
        pipeline.add_layer(location_lock);
    }

    // Biometric Lock
    if let Some(biometric_hash) = matches.get_one::<String>("biometric_lock") {
        let biometric_lock = Box::new(BiometricLock::new(
            biometric_hash.to_string(),
            biometric_hash.to_string(),
        ));
        pipeline.add_layer(biometric_lock);
    }

    match action.as_str() {
        "encrypt" => {
            let encrypted_data = pipeline.encrypt(data);
            println!("Encrypted data: {:?}", encrypted_data);
        }
        "decrypt" => {
            // Simulate waiting for time lock to expire
            if matches.get_one::<u64>("time_lock").is_some() {
                println!("Waiting for time lock to expire...");
                sleep(Duration::from_secs(6)); // Mock time-lock expiration
            }
            let decrypted_data = pipeline.decrypt(data);
            println!("Decrypted data: {:?}", String::from_utf8(decrypted_data).unwrap());
        }
        _ => {
            println!("Invalid action: {}", action);
        }
    }
}
