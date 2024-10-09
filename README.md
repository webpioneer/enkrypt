## Usage
To encrypt:
cargo run -- --action encrypt --data "Some secret data" --time-lock 5 --location-lock 40.7128 74.0060 --biometric-lock "hash_value"
```

To decrypt:
```cargo run -- --action decrypt --data "Some secret data" --time-lock 5 --location-lock 40.7128 74.0060 --biometric-lock "hash_value"

```