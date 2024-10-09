## Overview of Enkrypt
Enkrypt is a cutting-edge multi-dimensional encryption framework designed to enhance data security through customizable and modular encryption layers. By leveraging various authentication methods, including time-based locks, geographical constraints, and biometric verification, Enkrypt provides a robust solution for protecting sensitive information. The framework is built in Rust, ensuring high performance and safety, making it suitable for a range of applications from personal data protection to enterprise-level security.

## Description
Enkrypt aims to revolutionize the way we think about data encryption by providing a flexible and extensible architecture. Key features of Enkrypt include:

Multi-Dimensional Encryption: Utilize multiple layers of security to protect data, each adding a unique aspect of authentication and encryption.

Modular Layer Design: Each encryption layer can be added or removed based on user requirements. This allows users to tailor the security measures according to their specific needs without compromising the integrity of the data.

Customizable Authentication: Choose from various authentication methods, including:

Time Lock: Set a specific time duration before data can be accessed or decrypted.
Location Lock: Enforce geographical restrictions by specifying allowed latitude and longitude for access.
Biometric Lock: Integrate biometric authentication using hash values, enhancing security through personal identification.
Command-Line Interface (CLI): Enkrypt provides a user-friendly CLI that allows users to easily select encryption and decryption options, as well as configure their preferred security layers with straightforward commands.

Written in Rust: Utilizing Rust’s strong safety guarantees and high performance, Enkrypt aims to deliver a reliable solution for both individual users and organizations.

Enkrypt is ideal for users who prioritize data security and wish to customize their encryption strategy. Whether it's for personal files, sensitive corporate data, or any other critical information, Enkrypt empowers users to protect their data effectively and efficiently.

## Usage
To encrypt:
```
cargo run -- --action encrypt --data "Some secret data" --time-lock 5 --location-lock 40.7128 74.0060 --biometric-lock "hash_value"
```

To decrypt:
```
cargo run -- --action decrypt --data "Some secret data" --time-lock 5 --location-lock 40.7128 74.0060 --biometric-lock "hash_value"

```