# Ferrite

**A fast, memory-safe cheminformatics toolkit written in Rust**

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

Ferrite is a modern, high-performance alternative to existing cheminformatics toolkits like RDKit. Built from the ground up in Rust, it provides a memory-safe, concurrent, and blazingly fast toolkit for working with chemical structures and properties.

## Current Status

🚧 **Early Development** 🚧

Ferrite is currently in the early development phase. We're building the core molecule representation and basic functionality. While not yet ready for production use, early contributors can help shape the architecture and feature set.

## Features (Planned)

- Comprehensive molecular representation
- SMILES and SDF file I/O
- Multiple fingerprint algorithms (ECFP, MACCS, etc.)
- Similarity searching and molecule comparison
- Substructure matching
- Property calculation
- 2D coordinate generation

## Getting Started

```bash
# Clone the repository
git clone https://github.com/mariusrueve/ferrite.git
cd ferrite

# Build the library
cargo build

# Run tests
cargo test
```

## How to Contribute

We welcome contributors of all experience levels! If you're interested in cheminformatics, Rust, or both, there are many ways to help:

1. **Code**: Implement new features, fix bugs, or improve performance
2. **Documentation**: Help make the API clearer and more accessible
3. **Testing**: Add test cases, especially with real-world molecules
4. **Design**: Provide input on the API design and feature priorities

## License

Ferrite is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.