# xeto-bacnet

Open-source Xeto type library for ASHRAE 135-2020 BACnet.

## Overview

`xeto-bacnet` provides a complete Xeto type model for the BACnet protocol,
covering 63 object types, 5 transport types, network topology, device
properties, services, and enumerations as defined in ASHRAE Standard 135-2020.

## Architecture

The library is organized in four layers:

```
┌─────────────────────────────────┐
│  Transport Layer                │
│  (IP, MSTP, Ethernet, ZigBee,  │
│   LonTalk)                     │
├─────────────────────────────────┤
│  Network Layer                  │
│  (Network, Router, BBMD,       │
│   Foreign-Device)              │
├─────────────────────────────────┤
│  Device Layer                   │
│  (Device, Properties, Services,│
│   Segmentation, COV)           │
├─────────────────────────────────┤
│  Object Layer                   │
│  (63 object types: AnalogInput,│
│   BinaryOutput, Schedule, etc.)│
└─────────────────────────────────┘
```

## File Structure

```
xeto-bacnet/
├── src/
│   └── bacnet/
│       ├── lib.xeto              # Library pragma
│       ├── enums.xeto            # BACnet enumerations
│       ├── transport.xeto        # Transport-layer types
│       ├── network.xeto          # Network-layer types
│       ├── device.xeto           # Device-layer types
│       ├── services.xeto         # BACnet services
│       └── objects/              # Object-type definitions
│           ├── analog.xeto
│           ├── binary.xeto
│           ├── schedule.xeto
│           └── ...
├── tests/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # Rust test harness
├── LICENSE
└── README.md
```

## Usage

Load the BACnet library with [rusty-haystack](https://github.com/j2inn/rusty-haystack):

```rust
use haystack_core::ontology::DefNamespace;
use std::fs;

let mut ns = DefNamespace::load_standard().unwrap();
let source = fs::read_to_string("src/bacnet/lib.xeto").unwrap();
ns.load_xeto_str(&source, "bacnet").unwrap();
```

## Status

**v0.1.0** — Initial scaffolding with library pragma and test harness.

## License

[MIT](LICENSE)
