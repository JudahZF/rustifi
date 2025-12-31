use futures::StreamExt;
use rustifi::api::clients::GetClients;
use rustifi::api::devices::{GetDeviceDetails, GetDeviceStatistics, GetDevices};
use rustifi::api::networks::GetNetworks;
use rustifi::api::sites::GetSites;
use rustifi::models::Site;
use rustifi::stats::DeviceClientStats;
use rustifi::wrappers::DeviceWithInfo;
use rustifi::UnifiClient;

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

    /// Fetch first page of devices (no pagination)
    pub async fn fetch_devices(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetDevices::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!(
            "\n=== Devices for site {} ({} of {}) ===",
            site_id, response.count, response.total_count
        );

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
            println!(
                "(More devices available, showing {} of {}. Use --all to fetch all pages)",
                response.count, response.total_count
            );
        }

        Ok(())
    }

    /// Fetch ALL devices using automatic pagination
    pub async fn fetch_all_devices(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Fetching all devices for site {} (with pagination) ===", site_id);

        let devices = self.client.fetch_all_devices(site_id).await?;

        println!("Fetched {} devices total", devices.len());

        let mut online = 0;
        let mut offline = 0;

        for device in &devices {
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
        println!("Total: {}", devices.len());

        Ok(())
    }

    /// Fetch first page of clients (no pagination)
    pub async fn fetch_clients(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetClients::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!(
            "\n=== Clients for site {} ({} of {}) ===",
            site_id, response.count, response.total_count
        );

        let mut wireless = 0;
        let mut wired = 0;
        let mut connected = 0;
        let mut guests = 0;

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

            if client.is_guest() {
                guests += 1;
            }
        }

        println!("\n--- Client Summary ---");
        println!("Wireless: {}", wireless);
        println!("Wired: {}", wired);
        println!("Guests: {}", guests);
        println!("Connected: {}", connected);
        println!("Total: {}", response.total_count);

        if response.has_more() {
            println!(
                "(More clients available, showing {} of {}. Use --all to fetch all pages)",
                response.count, response.total_count
            );
        }

        Ok(())
    }

    /// Fetch ALL clients using automatic pagination
    pub async fn fetch_all_clients(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "\n=== Fetching all clients for site {} (with pagination) ===",
            site_id
        );

        let clients = self.client.fetch_all_clients(site_id).await?;

        println!("Fetched {} clients total", clients.len());

        let mut wireless = 0;
        let mut wired = 0;
        let mut connected = 0;
        let mut guests = 0;

        for client in &clients {
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

            if client.is_guest() {
                guests += 1;
            }
        }

        println!("\n--- Client Summary ---");
        println!("Wireless: {}", wireless);
        println!("Wired: {}", wired);
        println!("Guests: {}", guests);
        println!("Connected: {}", connected);
        println!("Total: {}", clients.len());

        Ok(())
    }

    /// Stream clients page by page (demonstrates the streaming API)
    pub async fn stream_clients(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "\n=== Streaming clients for site {} (page by page) ===",
            site_id
        );

        let mut stream = self.client.stream_clients(site_id);
        let mut page_num = 0;
        let mut total = 0;

        while let Some(result) = stream.next().await {
            let page = result?;
            page_num += 1;
            total += page.len();

            println!("\n--- Page {} ({} clients) ---", page_num, page.len());

            for client in &page {
                println!(
                    "  {} - {} ({:?})",
                    client.id,
                    client.name.as_deref().unwrap_or("unnamed"),
                    client.client_type
                );
            }
        }

        println!("\n=== Streaming complete ===");
        println!("Total pages: {}", page_num);
        println!("Total clients: {}", total);

        Ok(())
    }

    /// Fetch client statistics aggregated by device
    pub async fn fetch_client_stats_by_device(
        &self,
        site_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "\n=== Client Stats by Device for site {} ===",
            site_id
        );

        let stats = self.client.fetch_client_stats_by_device(site_id).await?;

        if stats.is_empty() {
            println!("No clients connected to any devices.");
            return Ok(());
        }

        println!("Found clients on {} devices:\n", stats.len());

        // Sort by total clients descending
        let mut stats_vec: Vec<_> = stats.iter().collect();
        stats_vec.sort_by(|a, b| b.1.total_clients.cmp(&a.1.total_clients));

        for (device_id, device_stats) in stats_vec {
            print_device_client_stats(device_id, device_stats);
            println!();
        }

        // Print totals
        let total_clients: usize = stats.values().map(|s| s.total_clients).sum();
        let total_wireless: usize = stats.values().map(|s| s.wireless_clients).sum();
        let total_wired: usize = stats.values().map(|s| s.wired_clients).sum();
        let total_guests: usize = stats.values().map(|s| s.guest_clients).sum();

        println!("--- Total Summary ---");
        println!("Total Devices with Clients: {}", stats.len());
        println!("Total Clients: {}", total_clients);
        println!("Total Wireless: {}", total_wireless);
        println!("Total Wired: {}", total_wired);
        println!("Total Guests: {}", total_guests);

        Ok(())
    }

    pub async fn fetch_networks(&self, site_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = GetNetworks::new(site_id);
        let response = self.client.execute(&endpoint).await?;
        println!(
            "\n=== Networks for site {} ({} of {}) ===",
            site_id, response.count, response.total_count
        );

        for network in &response.data {
            print_network(network);
            println!();
        }

        if response.has_more() {
            println!(
                "(More networks available, showing {} of {})",
                response.count, response.total_count
            );
        }

        Ok(())
    }

    pub async fn fetch_device_details(
        &self,
        site_id: &str,
        device_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    /// Fetch device with details and statistics in parallel using DeviceWithInfo wrapper
    pub async fn fetch_device_with_info(
        &self,
        site_id: &str,
        device_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Fetching device with info (parallel fetch) ===");

        let device_info = self
            .client
            .fetch_device_with_info(site_id, device_id)
            .await?;

        print_device_with_info(&device_info);

        Ok(())
    }

    /// Fetch all devices with their details and statistics
    pub async fn fetch_all_devices_with_info(
        &self,
        site_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "\n=== Fetching all devices with info for site {} ===",
            site_id
        );

        let devices = self.client.fetch_all_devices_with_info(site_id).await?;

        println!("Fetched {} devices with full info\n", devices.len());

        for device_info in &devices {
            print_device_with_info(device_info);
            println!("\n{}", "=".repeat(60));
        }

        // Summary
        let online_count = devices.iter().filter(|d| d.is_online()).count();
        let total_ports: usize = devices.iter().map(|d| d.port_count()).sum();
        let total_radios: usize = devices.iter().map(|d| d.radio_count()).sum();

        println!("\n--- Summary ---");
        println!("Total Devices: {}", devices.len());
        println!("Online: {}", online_count);
        println!("Offline: {}", devices.len() - online_count);
        println!("Total Ports: {}", total_ports);
        println!("Total Radios: {}", total_radios);

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn fetch_device_statistics(
        &self,
        site_id: &str,
        device_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        let features: Vec<&str> = device
            .features
            .iter()
            .map(|f| match f {
                rustifi::models::DeviceFeature::Switching => "switching",
                rustifi::models::DeviceFeature::AccessPoint => "accessPoint",
                rustifi::models::DeviceFeature::Gateway => "gateway",
                rustifi::models::DeviceFeature::Unknown => "unknown",
            })
            .collect();
        println!("  Features: {}", features.join(", "));
    }

    if !device.interfaces.is_empty() {
        let interfaces: Vec<&str> = device
            .interfaces
            .iter()
            .map(|i| match i {
                rustifi::models::DeviceInterface::Ports => "ports",
                rustifi::models::DeviceInterface::Radios => "radios",
                rustifi::models::DeviceInterface::Unknown => "unknown",
            })
            .collect();
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

    // NEW: Show the device the client is connected to
    if let Some(device_id) = client.device_id() {
        println!("  Connected to Device: {}", device_id);
    }

    if client.is_guest() {
        println!("  [GUEST]");
    }

    if client.is_blocked() {
        println!("  [BLOCKED]");
    }
}

fn print_device_client_stats(device_id: &str, stats: &DeviceClientStats) {
    println!("Device: {}", device_id);
    println!("  Total Clients: {}", stats.total_clients);
    println!("  Wireless: {}", stats.wireless_clients);
    println!("  Wired: {}", stats.wired_clients);
    println!("  Guests: {}", stats.guest_clients);
    println!(
        "  Non-Guest: {}",
        stats.non_guest_clients()
    );
}

fn print_device_with_info(device_info: &DeviceWithInfo) {
    println!("\nDevice: {} ({})", device_info.name(), device_info.id());
    println!("  Model: {}", device_info.model());
    println!(
        "  Status: {}",
        if device_info.is_online() {
            "ONLINE"
        } else {
            "OFFLINE"
        }
    );
    println!("  Uptime: {}", device_info.uptime_formatted());

    // Capabilities
    let mut caps = Vec::new();
    if device_info.is_access_point() {
        caps.push("Access Point");
    }
    if device_info.is_gateway() {
        caps.push("Gateway");
    }
    if device_info.has_switching() {
        caps.push("Switching");
    }
    if !caps.is_empty() {
        println!("  Capabilities: {}", caps.join(", "));
    }

    // Hardware
    println!("  Ports: {}", device_info.port_count());
    println!("  Radios: {}", device_info.radio_count());

    // Statistics
    if let Some(cpu) = device_info.cpu_utilization() {
        println!("  CPU: {:.1}%", cpu);
    }
    if let Some(mem) = device_info.memory_utilization() {
        println!("  Memory: {:.1}%", mem);
    }

    // Load averages from statistics
    if let Some(load) = device_info.statistics.load_average_1_min {
        println!("  Load (1m): {:.2}", load);
    }

    // Uplink rates
    if let Some(ref uplink) = device_info.statistics.uplink {
        println!(
            "  Uplink: TX {:.2} Mbps / RX {:.2} Mbps",
            uplink.tx_rate_mbps(),
            uplink.rx_rate_mbps()
        );
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
        println!(
            "    TX Rate: {:.2} Mbps ({} bps)",
            uplink.tx_rate_mbps(),
            uplink.tx_rate_bps
        );
        println!(
            "    RX Rate: {:.2} Mbps ({} bps)",
            uplink.rx_rate_mbps(),
            uplink.rx_rate_bps
        );
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
