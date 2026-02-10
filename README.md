# rustifi

A Rust API library for UniFi Network controllers.

## Quick Start

```rust
use rustifi::UnifiClient;

#[tokio::main]
async fn main() -> rustifi::Result<()> {
    // Create a client with strict TLS validation (recommended)
    let client = UnifiClient::with_api_key(
        "https://unifi.example.com",
        "your-api-key"
    )?;

    // Fetch all sites
    let sites = client.request::<rustifi::api::sites::GetSites>().await?;
    for site in sites.data {
        println!("Site: {} ({})", site.name, site.id);
    }

    Ok(())
}
```

## TLS Certificate Handling

By default, rustifi uses **strict TLS validation** and requires valid certificates.

### For Controllers with Valid Certificates

Use the standard constructors:

```rust
// These require valid TLS certificates
let client = UnifiClient::new("https://unifi.example.com")?;
let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
```

### For Local Controllers with Self-Signed Certificates

Many local UniFi controllers use self-signed certificates. For these, use the `_insecure` variants:

```rust
// WARNING: Only use for local controllers on trusted networks
let client = UnifiClient::new_insecure("https://192.168.1.1")?;
let client = UnifiClient::with_api_key_insecure("https://192.168.1.1", "api-key")?;
```

> **Security Warning**: The `_insecure` methods disable TLS certificate validation,
> which makes connections vulnerable to man-in-the-middle attacks. Only use these
> methods for local controllers on trusted networks.

## WORK IN PROGRESS


## To Do

- [x] Implement some access points
- [ ] Implement some switches
- [ ] Ensure compliance with Rust API Guidelines Checklist, Including traits
    - Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default
    - From, TryFrom, AsRef, AsMut
    - Errors
- [ ] Link capacity on uplinks
- [ ] Tests
- [ ] Documentation
- [ ] Examples

## Layout & Features

- [ ] Devices
    - [x] Basic Information
        - [x] Get
        - [x] Update
    - [ ] AP
        - [ ] Group
        - [x] Model
        - [x] Name
        - [x] Radio
            - [x] Channel
            - [x] Width
            - [x] Power
            - [ ] Meshing
        - [x] Number of Clients
        - [ ] CPU
        - [ ] Memory
    - [ ] SW
        - [x] Name
        - [x] Model
        - [ ] Port
            - [ ] Number
            - [ ] Type
            - [ ] Status
            - [ ] Uptime
            - [ ] Bytes
            - [ ] Packets
            - [ ] Dropped
            - [ ] Errors
            - [ ] Native VLAN
            - [ ] Allowed VLANs
            - [ ] Port Isolation
        - [x] CPU
        - [x] Memory
- [ ] Clients
    - [ ] Type
    - [ ] Impose Punishment
    - [ ] Ban
- [ ] WiFi
- [ ] Network

## Models

### APs

- [ ] UX
- [ ] NanoHD
- [ ] U7-Pro
- [ ] U7-Pro-Max
- [x] U6-Mesh
- [x] UWB-XG
- [x] UAP-XG
- [x] AC-Mesh
- [x] AC-Mesh-Pro
- [x] AC-Pro

### Switch

- [ ] USW-Pro-Aggregation
- [ ] USW-Aggregation

## License

This project is licensed under GNU General Public License v2.0 (`GPL-2.0-only`).
