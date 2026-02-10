# rustifi

Open source Rust client for the UniFi Network API.

`rustifi` gives you typed endpoint definitions, async request execution, pagination helpers, and higher-level wrappers for common UniFi workflows.

## Features

- Async API client built on `reqwest` + `tokio`
- Typed endpoint modules (`sites`, `devices`, `clients`, `networks`, `wifi`, and more)
- Local controller support and remote cloud access via `api.ui.com`
- Safe-by-default TLS constructors, plus explicit insecure variants for self-signed local setups
- Pagination helpers: fetch everything at once or stream page by page
- Utility wrappers for device details/statistics and client aggregation

## Installation

```bash
cargo add rustifi
cargo add tokio --features rt-multi-thread,macros
```

If you use streaming helpers, also add:

```bash
cargo add futures
```

## Quick Start

```rust
use rustifi::api::sites::GetSites;
use rustifi::UnifiClient;

#[tokio::main]
async fn main() -> rustifi::Result<()> {
    let client = UnifiClient::with_api_key(
        "https://unifi.example.com",
        "your-api-key",
    )?;

    let sites = client.request::<GetSites>().await?;
    for site in sites.data {
        println!("{} ({})", site.name, site.id);
    }

    Ok(())
}
```

## Connection Modes

### 1) Local controller (strict TLS, recommended)

```rust
let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
```

### 2) Local controller with self-signed certificate

```rust
// Only for trusted local networks
let client = UnifiClient::with_api_key_insecure("https://192.168.1.1", "api-key")?;
```

### 3) Custom API base path (UniFi OS integration prefix, etc.)

```rust
let client = UnifiClient::with_base_path_and_key(
    "https://unifi.example.com",
    "/proxy/network/integration",
    "api-key",
)?;
```

### 4) Remote access via `api.ui.com`

```rust
let client = UnifiClient::remote("site-manager-api-key", "your-host-id")?;
```

Remote mode targets:

`https://api.ui.com/v1/connector/consoles/{host_id}/...`

## Endpoint Pattern

Use `request::<E>()` for endpoints without parameters, and `execute(&endpoint)` for dynamic endpoints.

```rust
use rustifi::api::devices::GetDevices;
use rustifi::api::sites::GetSites;

let sites = client.request::<GetSites>().await?;
let devices = client.execute(&GetDevices::new("site-id")).await?;
```

## Pagination Helpers

Fetch all pages automatically:

```rust
let all_clients = client.fetch_all_clients("site-id").await?;
let all_devices = client.fetch_all_devices("site-id").await?;
```

Or stream page by page:

```rust
use futures::StreamExt;

let mut stream = client.stream_clients("site-id").page_size(200);
while let Some(page) = stream.next().await {
    let clients = page?;
    println!("Fetched {} clients", clients.len());
}
```

## High-Level Utilities

```rust
// Device + details + latest statistics
let device = client.fetch_device_with_info("site-id", "device-id").await?;

// All devices with details/statistics fetched concurrently
let devices = client.fetch_all_devices_with_info("site-id").await?;

// Client counts by device
let stats = client.fetch_client_stats_by_device("site-id").await?;
```

## Implemented API Modules

- Sites
- Devices (including pending devices, adoption, device actions, port actions)
- Clients
- Networks
- WiFi broadcasts
- Hotspot vouchers
- Firewall zones and policies
- ACL rules
- DNS policies
- Traffic matching lists
- Resources (WANs, VPN servers, VPN tunnels, RADIUS profiles, device tags, DPI data, countries)

## Examples

- `examples/fetch-all`: end-to-end data fetch demo
- `examples/rustifi-cli`: CLI wrapper around common fetch operations

Run the fetch-all example:

```bash
cd examples/fetch-all
UNIFI_URL=https://your-controller \
UNIFI_API_KEY=your-api-key \
cargo run
```

Run remote mode:

```bash
cd examples/fetch-all
UNIFI_HOST_ID=your-host-id \
UNIFI_API_KEY=your-api-key \
cargo run
```

## Security Notes

- Constructors without `_insecure` enforce TLS certificate validation.
- `_insecure` constructors disable cert and hostname validation and are vulnerable to MITM attacks.
- Prefer strict TLS in production.

## License

Licensed under GNU General Public License v2.0 (`GPL-2.0-only`). See `LICENSE.md`.
