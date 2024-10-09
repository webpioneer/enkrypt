pub trait CryptoLayer {
    /// Encrypt the data using the layer's logic
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;

    /// Decrypt the data using the layer's logic
    fn decrypt(&self, data: &[u8]) -> Vec<u8>;

    /// Check if the layer is satisfied (e.g., time-lock has expired, correct location, etc.)
    fn is_satisfied(&self) -> bool;
}

pub struct CryptoPipeline {
    layers: Vec<Box<dyn CryptoLayer>>,
}

impl CryptoPipeline {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Box<dyn CryptoLayer>) {
        self.layers.push(layer);
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();
        for layer in &self.layers {
            result = layer.encrypt(&result);
        }
        result
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();
        for layer in &self.layers {
            if layer.is_satisfied() {
                result = layer.decrypt(&result);
            } else {
                panic!("Decryption failed: One or more layers are not satisfied.");
            }
        }
        result
    }
}

