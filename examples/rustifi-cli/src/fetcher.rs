use rustifi::UnifiClient;
use rustifi::api::sites::GetSites;
use rustifi::api::devices::{GetDevices, GetDeviceDetails, GetDeviceStatistics};
use rustifi::api::clients::GetClients;
use rustifi::api::networks::GetNetworks;
use rustifi::models::Site;

pub struct UnifiFetcher {
    client: UnifiClient,
}

impl UnifiFetcher {
    pub fn new(
        controller_url: &str,
        api_key: &str,
        base_path: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = if let Some(path) = base_path {
            UnifiClient::with_base_path_and_key(controller_url, path, api_key)?
        } else {
            UnifiClient::with_api_key(controller_url, api_key)?
        };
        Ok(Self { client })
    }

    pub async fn fetch_sites(&self) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
        let response = self.client.request::<GetSites>().await?;
        println!("\n=== Sites ({}) ===", response.data.len());

        for site in &response.data {
            print_site(site);
            println!();
        }

        Ok(response.data)
    }

    pub async fn fetch_devices(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetDevices::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!("\n=== Devices for site {} ({} of {}) ===", site_id, response.count, response.total_count);

        let mut online = 0;
        let mut offline = 0;

        for device in &response.data {
            print_device(device);
            println!();

            if device.is_online() {
                online += 1;
            } else {
                offline += 1;
            }
        }

        println!("\n--- Device Summary ---");
        println!("Online: {}", online);
        println!("Offline: {}", offline);
        println!("Total: {}", response.total_count);

        if response.has_more() {
            println!("(More devices available, showing {} of {})", response.count, response.total_count);
        }

        Ok(())
    }

    pub async fn fetch_clients(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetClients::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!("\n=== Clients for site {} ({} of {}) ===", site_id, response.count, response.total_count);

        let mut wireless = 0;
        let mut wired = 0;
        let mut connected = 0;

        for client in &response.data {
            print_client(client);
            println!();

            if client.is_wireless() {
                wireless += 1;
            } else if client.is_wired() {
                wired += 1;
            }

            if client.is_connected() {
                connected += 1;
            }
        }

        println!("\n--- Client Summary ---");
        println!("Wireless: {}", wireless);
        println!("Wired: {}", wired);
        println!("Connected: {}", connected);
        println!("Total: {}", response.total_count);

        if response.has_more() {
            println!("(More clients available, showing {} of {})", response.count, response.total_count);
        }

        Ok(())
    }

    pub async fn fetch_networks(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetNetworks::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!("\n=== Networks for site {} ({} of {}) ===", site_id, response.count, response.total_count);

        for network in &response.data {
            print_network(network);
            println!();
        }

        if response.has_more() {
            println!("(More networks available, showing {} of {})", response.count, response.total_count);
        }

        Ok(())
    }

    pub async fn fetch_device_details(&self, site_id: &str, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetDeviceDetails::new(site_id, device_id);
        let device = self.client.execute(&endpoint).await?;
        println!("\n=== Device Details ===");
        print_device_details(&device);

        // Also fetch and display statistics
        let stats_endpoint = GetDeviceStatistics::new(site_id, device_id);
        match self.client.execute(&stats_endpoint).await {
            Ok(stats) => {
                println!("\n=== Device Statistics ===");
                print_device_statistics(&stats);
            }
            Err(e) => {
                println!("\n(Could not fetch device statistics: {})", e);
            }
        }

        Ok(())
    }

    pub async fn fetch_device_statistics(&self, site_id: &str, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetDeviceStatistics::new(site_id, device_id);
        let stats = self.client.execute(&endpoint).await?;
        println!("\n=== Device Statistics ===");
        print_device_statistics(&stats);
        Ok(())
    }
}

fn print_site(site: &Site) {
    println!("Site:");
    println!("  ID: {}", site.id);
    println!("  Name: {}", site.name);
    println!("  Description: {:?}", site.desc);
    if let Some(ref role) = site.role {
        println!("  Role: {}", role);
    }
}

fn print_device(device: &rustifi::models::SiteDevice) {
    println!("Device:");
    println!("  ID: {}", device.id);
    println!("  Name: {}", device.name);
    println!("  MAC: {}", device.mac_address.as_str());
    println!("  Model: {}", device.model);
    println!("  State: {:?}", device.state);

    if let Some(ref ip) = device.ip_address {
        println!("  IP: {}", ip.as_str());
    }

    if let Some(ref version) = device.firmware_version {
        println!("  Firmware: {}", version);
    }

    if let Some(updatable) = device.firmware_updatable {
        println!("  Firmware Updatable: {}", updatable);
    }

    if let Some(supported) = device.supported {
        println!("  Supported: {}", supported);
    }

    if !device.features.is_empty() {
        let features: Vec<&str> = device.features.iter().map(|f| match f {
            rustifi::models::DeviceFeature::Switching => "switching",
            rustifi::models::DeviceFeature::AccessPoint => "accessPoint",
            rustifi::models::DeviceFeature::Gateway => "gateway",
            rustifi::models::DeviceFeature::Unknown => "unknown",
        }).collect();
        println!("  Features: {}", features.join(", "));
    }

    if !device.interfaces.is_empty() {
        let interfaces: Vec<&str> = device.interfaces.iter().map(|i| match i {
            rustifi::models::DeviceInterface::Ports => "ports",
            rustifi::models::DeviceInterface::Radios => "radios",
            rustifi::models::DeviceInterface::Unknown => "unknown",
        }).collect();
        println!("  Interfaces: {}", interfaces.join(", "));
    }
}

fn print_client(client: &rustifi::models::Client) {
    println!("Client:");
    println!("  ID: {}", client.id);

    if let Some(ref name) = client.name {
        println!("  Name: {}", name);
    }

    if let Some(ref ip) = client.ip_address {
        println!("  IP: {}", ip.as_str());
    }

    println!("  Type: {:?}", client.client_type);
    println!("  Connected: {}", client.is_connected());

    if let Some(ref connected_at) = client.connected_at {
        println!("  Connected At: {}", connected_at);
    }

    if let Some(ref access) = client.access {
        println!("  Access: {:?}", access.access_type);
    }

    if client.is_blocked() {
        println!("  BLOCKED");
    }
}

fn print_network(network: &rustifi::api::networks::Network) {
    println!("Network:");
    println!("  ID: {}", network.id);
    println!("  Name: {}", network.name);
    println!("  Enabled: {}", network.enabled);

    if let Some(ref vlan_id) = network.vlan_id {
        println!("  VLAN ID: {}", vlan_id);
    }

    if let Some(ref management) = network.management {
        println!("  Management: {:?}", management);
    }

    if let Some(ref metadata) = network.metadata {
        if let Some(ref origin) = metadata.origin {
            println!("  Origin: {:?}", origin);
        }
    }
}

fn print_device_statistics(stats: &rustifi::models::DeviceStatistics) {
    println!("Device Statistics:");
    println!("  Uptime: {}", stats.uptime_formatted());

    if let Some(ref last_hb) = stats.last_heartbeat_at {
        println!("  Last Heartbeat: {}", last_hb);
    }

    if let Some(ref next_hb) = stats.next_heartbeat_at {
        println!("  Next Heartbeat: {}", next_hb);
    }

    if let Some(cpu) = stats.cpu_utilization_pct {
        println!("  CPU Utilization: {:.1}%", cpu);
    }

    if let Some(mem) = stats.memory_utilization_pct {
        println!("  Memory Utilization: {:.1}%", mem);
    }

    if let Some(load_1) = stats.load_average_1_min {
        println!("  Load Average (1m): {:.2}", load_1);
    }

    if let Some(load_5) = stats.load_average_5_min {
        println!("  Load Average (5m): {:.2}", load_5);
    }

    if let Some(load_15) = stats.load_average_15_min {
        println!("  Load Average (15m): {:.2}", load_15);
    }

    if let Some(ref uplink) = stats.uplink {
        println!("\n  Uplink:");
        println!("    TX Rate: {:.2} Mbps ({} bps)", uplink.tx_rate_mbps(), uplink.tx_rate_bps);
        println!("    RX Rate: {:.2} Mbps ({} bps)", uplink.rx_rate_mbps(), uplink.rx_rate_bps);
    }

    if stats.has_radios() {
        if let Some(ref interfaces) = stats.interfaces {
            println!("\n  Radios ({}):", interfaces.radios.len());
            for (idx, radio) in interfaces.radios.iter().enumerate() {
                println!("    Radio {}:", idx + 1);
                if let Some(freq) = radio.frequency_ghz {
                    println!("      Frequency: {} GHz", freq);
                }
                if let Some(retries) = radio.tx_retries_pct {
                    println!("      TX Retries: {:.1}%", retries);
                }
            }
        }
    }
}

fn print_device_details(device: &rustifi::models::DeviceDetails) {
    println!("Device Details:");
    println!("  ID: {}", device.id);
    println!("  Name: {}", device.name);
    println!("  MAC: {}", device.mac_address.as_str());
    println!("  IP: {}", device.ip_address.as_str());
    println!("  Model: {}", device.model);
    println!("  State: {}", device.state);
    println!("  Supported: {}", device.supported);
    println!("  Firmware Updatable: {}", device.firmware_updatable);

    if let Some(ref version) = device.firmware_version {
        println!("  Firmware Version: {}", version);
    }

    if let Some(ref adopted) = device.adopted_at {
        println!("  Adopted At: {}", adopted);
    }

    if let Some(ref provisioned) = device.provisioned_at {
        println!("  Provisioned At: {}", provisioned);
    }

    println!("  Configuration ID: {}", device.configuration_id);
    if let Some(ref uplink) = device.uplink {
        println!("  Uplink Device ID: {}", uplink.device_id);
    }

    // Print features
    let mut features = Vec::new();
    if device.has_switching() {
        features.push("Switching");
    }
    if device.has_access_point() {
        features.push("Access Point");
    }
    if !features.is_empty() {
        println!("  Features: {}", features.join(", "));
    }

    // Print ports
    if device.port_count() > 0 {
        println!("\n  Ports ({}):", device.port_count());
        for port in &device.interfaces.ports {
            println!("    Port {}:", port.idx);
            println!("      State: {:?}", port.state);
            println!("      Connector: {:?}", port.connector);
            println!("      Max Speed: {}Mbps", port.max_speed_mbps);
            if let Some(speed) = port.speed_mbps {
                println!("      Speed: {}Mbps", speed);
            }
            if let Some(ref poe) = port.poe {
                println!("      PoE:");
                if let Some(ref standard) = poe.standard {
                    println!("        Standard: {}", standard);
                }
                if let Some(poe_type) = poe.r#type {
                    println!("        Type: {}", poe_type);
                }
                if let Some(enabled) = poe.enabled {
                    println!("        Enabled: {}", enabled);
                }
                if let Some(ref state) = poe.state {
                    println!("        State: {}", state);
                }
            }
        }
    }

    // Print radios
    if device.radio_count() > 0 {
        println!("\n  Radios ({}):", device.radio_count());
        for (idx, radio) in device.interfaces.radios.iter().enumerate() {
            println!("    Radio {}:", idx + 1);
            println!("      Standard: {:?}", radio.wlan_standard);
            println!("      Frequency: {}GHz", radio.frequency_ghz);
            println!("      Channel Width: {}MHz", radio.channel_width_mhz);
            if let Some(channel) = radio.channel {
                println!("      Channel: {}", channel);
            }
        }
    }
}


