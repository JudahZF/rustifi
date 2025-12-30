# Rustifi v1.0.0 - Endpoint Roadmap

## Implemented (v1.0.0)
- [x] API Key Authentication (X-API-Key header)
- [x] Application Info (GET /api/self)
- [x] List Sites (GET /api/sites)
- [x] List Devices (GET /api/devices)
- [x] Device Details (GET /api/devices/{id})
- [x] List Clients (GET /api/clients)
- [x] List Networks (GET /api/networks)

## Usage

```rust
use rustifi::UnifiClient;

let client = UnifiClient::with_api_key(
    "https://unifi.controller.local",
    "your-api-key-here"
)?;

let sites = client.request::<ListSites>().await?;
```

## Next Release (v1.1.0)
### Networks CRUD
- [ ] Create Network (POST /api/networks)
- [ ] Update Network (PUT /api/networks/{id})
- [ ] Delete Network (DELETE /api/networks/{id})

### WiFi
- [ ] List WiFi Broadcasts (GET /api/wifi-broadcasts)
- [ ] Create WiFi Broadcast (POST /api/wifi-broadcasts)
- [ ] Update WiFi Broadcast (PUT /api/wifi-broadcasts/{id})
- [ ] Delete WiFi Broadcast (DELETE /api/wifi-broadcasts/{id})

## Future Releases

### v1.2.0 - Device Actions
- [ ] Adopt Device (POST /api/devices/{id}/adopt)
- [ ] Execute Device Action (POST /api/devices/{id}/action)
- [ ] Pending Devices (GET /api/devices/pending)

### v1.3.0 - Clients & Security
- [ ] Client Details (GET /api/clients/{id})
- [ ] Client Action (POST /api/clients/{id}/action)

### v1.4.0 - Firewall & ACL
- [ ] List Firewall Zones (GET /api/firewall/zones)
- [ ] Create Firewall Zone (POST /api/firewall/zones)
- [ ] Update Firewall Zone (PUT /api/firewall/zones/{id})
- [ ] Delete Firewall Zone (DELETE /api/firewall/zones/{id})
- [ ] List Firewall Policies (GET /api/firewall/policies)
- [ ] List ACL Rules (GET /api/acl-rules)

### v1.5.0 - Hotspot & DNS
- [ ] List Vouchers (GET /api/hotspot/vouchers)
- [ ] Create Voucher (POST /api/hotspot/vouchers)
- [ ] Delete Voucher (DELETE /api/hotspot/vouchers/{id})
- [ ] List DNS Policies (GET /api/dns-policies)
- [ ] Create DNS Policy (POST /api/dns-policies)
- [ ] Update DNS Policy (PUT /api/dns-policies/{id})
- [ ] Delete DNS Policy (DELETE /api/dns-policies/{id})

### v1.6.0 - Traffic & Supporting Resources
- [ ] List Traffic Matching Lists (GET /api/traffic-matching-lists)
- [ ] Create Traffic Matching List (POST /api/traffic-matching-lists)
- [ ] Update Traffic Matching List (PUT /api/traffic-matching-lists/{id})
- [ ] Delete Traffic Matching List (DELETE /api/traffic-matching-lists/{id})
- [ ] List WAN Settings (GET /api/wan)
- [ ] List VPN Settings (GET /api/vpn)
- [ ] List RADIUS Profiles (GET /api/radius)
- [ ] List DPI Applications (GET /api/dpi)
- [ ] List Countries (GET /api/countries)

## How to Add New Endpoints

1. Define endpoint struct in `src/api/endpoints.rs`:
```rust
#[derive(Debug)]
pub struct GetDevices;

impl Endpoint for GetDevices {
    const PATH: &'static str = "api/devices";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = PaginatedResponse<RawDevice>;
}
```

2. Add method to `UnifiClient` in `src/client.rs`:
```rust
pub async fn list_devices(&self) -> Result<PaginatedResponse<RawDevice>> {
    self.request::<GetDevices>().await
}
```

3. Add model conversion in `src/models/device.rs`:
```rust
impl From<RawDevice> for Device {
    fn from(raw: RawDevice) -> Self {
        // conversion logic
    }
}
```
