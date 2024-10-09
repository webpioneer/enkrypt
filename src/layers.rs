use std::time::{Duration, SystemTime};

use crate::core::CryptoLayer;

pub struct TimeLock {
    unlock_time: SystemTime,
}

impl TimeLock {
    pub fn new(duration: Duration) -> Self {
        Self {
            unlock_time: SystemTime::now() + duration,
        }
    }
}

impl CryptoLayer for TimeLock {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Just return the data as is (could add actual encryption here)
        data.to_vec()
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        if self.is_satisfied() {
            data.to_vec() // Proceed with decryption
        } else {
            panic!("Time lock not satisfied yet");
        }
    }

    fn is_satisfied(&self) -> bool {
        SystemTime::now() >= self.unlock_time
    }
}


pub struct LocationLock {
    allowed_lat: f64,
    allowed_lon: f64,
    current_lat: f64,
    current_lon: f64,
}

impl LocationLock {
    pub fn new(allowed_lat: f64, allowed_lon: f64, current_lat: f64, current_lon: f64) -> Self {
        Self {
            allowed_lat,
            allowed_lon,
            current_lat,
            current_lon,
        }
    }

    fn is_in_allowed_area(&self) -> bool {
        // Simple distance check (use real geofencing in production)
        (self.allowed_lat - self.current_lat).abs() < 0.01 && (self.allowed_lon - self.current_lon).abs() < 0.01
    }
}

impl CryptoLayer for LocationLock {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Return the data unchanged
        data.to_vec()
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        if self.is_satisfied() {
            data.to_vec() // Proceed with decryption
        } else {
            panic!("Location lock not satisfied");
        }
    }

    fn is_satisfied(&self) -> bool {
        self.is_in_allowed_area()
    }
}

pub struct BiometricLock {
    expected_biometric_hash: String,
    provided_biometric_hash: String,
}

impl BiometricLock {
    pub fn new(expected_biometric_hash: String, provided_biometric_hash: String) -> Self {
        Self {
            expected_biometric_hash,
            provided_biometric_hash,
        }
    }

    fn is_biometric_match(&self) -> bool {
        self.expected_biometric_hash == self.provided_biometric_hash
    }
}

impl CryptoLayer for BiometricLock {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Return the data unchanged
        data.to_vec()
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        if self.is_satisfied() {
            data.to_vec() // Proceed with decryption
        } else {
            panic!("Biometric match failed");
        }
    }

    fn is_satisfied(&self) -> bool {
        self.is_biometric_match()
    }
}

