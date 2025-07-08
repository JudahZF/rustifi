#![recursion_limit = "512"]
use rustifi::{
    responses::stat::devices::{DeviceListResponse, RadioTable},
    types::{Radio, RadioPowerMode, RadioType},
};
use serde_json::json;

#[test]
fn test_device_list_response_deserialize() {
    // Create a minimal JSON representation of a DeviceListResponse
    let json_data = json!({
        "meta": {
            "rc": "ok"
        },
        "data": [
            {
                "required_version": "1.0.0",
                "port_table": [],
                "heightInMeters": 2.0,
                "has_speaker": false,
                "mesh_sta_vap_enabled": false,
                "license_state": "registered",
                "lcm_brightness_override": false,
                "type": "uap",
                "board_rev": 1,
                "setup_id": "00000000-0000-0000-0000-000000000000",
                "hw_caps": 2048,
                "snmp_contact": "",
                "reboot_duration": 0,
                "config_network": {
                    "type": "dhcp",
                    "bonding_enabled": false
                },
                "syslog_key": "0000000000000000000000000000000000000000000000000000000000000000",
                "model": "U7MSH",
                "outdoor_mode_override": "default",
                "lcm_tracker_enabled": false,
                "multi_active_antennas": false,
                "manufacturer_id": 2,
                "bandsteering_mode": "off",
                "sysid": 10000,
                "ip": "192.168.1.100",
                "fw2_caps": 0,
                "support_wifi6e": false,
                "led_override_color": "#0000ff",
                "version": "6.0.0.0000",
                "unsupported_reason": 0,
                "scan_radio_table": [],
                "adoption_completed": true,
                "x_vwirekey": "0000000000000000000000000000000000000000",
                "anon_id": "00000000-0000-0000-0000-000000000000",
                "supports_fingerprint_ml": false,
                "country_code": 0,
                "wlangroup_id_na": "000000000000000000000000",
                "countrycode_table": [],
                "antenna_table": [
                    {
                        "wifi1_gain": 4,
                        "default": true,
                        "name": "Standard (included)",
                        "id": 4,
                        "wifi0_gain": 3
                    }
                ],
                "wifi_caps": 20000000,
                "site_id": "000000000000000000000000",
                "name": "Test AP",
                "fw_caps": 123456,
                "_id": "000000000000000000000000",
                "internet": true,
                "wlangroup_id_ng": "000000000000000000000000",
                "mgmt_network_id": "000000000000000000000000",
                "gateway_mac": "00:11:22:33:44:55",
                "atf_enabled": false,
                "radio_table": [
                    {
                        "antenna_gain": 6,
                        "builtin_antenna": true,
                        "vwire_enabled": false,
                        "hard_noise_floor_enabled": false,
                        "sens_level_enabled": false,
                        "channel": 6,
                        "max_txpower": 20,
                        "min_rssi_enabled": true,
                        "builtin_ant_gain": 0,
                        "ht": 20,
                        "radio": "ng",
                        "nss": 2,
                        "tx_power_mode": "low",
                        "min_rssi": -71,
                        "name": "wifi0",
                        "min_txpower": 4,
                        "radio_caps": 4116,
                        "antenna_id": 4,
                        "radio_caps2": 27,
                        "current_antenna_gain": 3
                    },
                    {
                        "antenna_gain": 4,
                        "builtin_antenna": false,
                        "has_dfs": true,
                        "vwire_enabled": false,
                        "hard_noise_floor_enabled": true,
                        "sens_level_enabled": false,
                        "channel": 48,
                        "max_txpower": 20,
                        "min_rssi_enabled": true,
                        "builtin_ant_gain": 0,
                        "is_11ac": true,
                        "ht": 20,
                        "radio": "na",
                        "nss": 2,
                        "tx_power_mode": "low",
                        "min_rssi": -72,
                        "name": "wifi1",
                        "min_txpower": 8,
                        "has_fccdfs": true,
                        "antenna_id": 4,
                        "radio_caps": 4116,
                        "radio_caps2": 27,
                        "current_antenna_gain": 4
                    }
                ],
                "external_id": "00000000-0000-0000-0000-000000000000",
                "two_phase_adopt": false,
                "connected_at": 1613456789,
                "inform_ip": "192.168.1.1",
                "cfgversion": "0000000000000000",
                "mac": "00:11:22:33:44:55",
                "provisioned_at": 1613456780,
                "inform_url": "https://unifi.example.com:8080/inform",
                "ethernet_table": [
                    {
                        "num_port": 1,
                        "name": "eth0",
                        "mac": "00:11:22:33:44:55"
                    }
                ],
                "upgrade_duration": 0,
                "unsupported": false,
                "sys_error_caps": 0,
                "ble_caps": 0,
                "dot1x_portctrl_enabled": false,
                "map_id": "000000000000000000000000",
                "last_uplink": {
                    "port_idx": 1,
                    "uplink_mac": "00:11:22:33:44:55",
                    "uplink_remote_port": 1,
                    "type": "wire"
                },
                "led_override": "default",
                "disconnected_at": 0,
                "architecture": "mips",
                "wifi_caps2": 0,
                "x_aes_gcm": true,
                "has_fan": false,
                "lcm_idle_timeout_override": false,
                "has_eth1": false,
                "model_incompatible": false,
                "x_authkey": "0000000000000000000000000000000000000000",
                "x_ssh_hostkey_fingerprint": "00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00",
                "model_in_eol": false,
                "vwire_table": [],
                "has_temperature": false,
                "switch_caps": {
                    "vlan_caps": 7
                },
                "adopted_by_client": "legacy-web",
                "snmp_location": "",
                "model_in_lts": false,
                "kernel_version": "4.4.153",
                "serial": "0000000000",
                "x": 0,
                "y": 0,
                "fixed_ap_available": false,
                "led_override_color_brightness": 100,
                "adopted": true,
                "hash_id": "0000000000000000",
                "device_id": "000000000000000000000000",
                "uplink": {
                    "uplink_mac": "00:11:22:33:44:55",
                    "uplink_remote_port": 1,
                    "port_idx": 1,
                    "type": "wire"
                },
                "state": 0,
                "start_disconnected_millis": 0,
                "upgrade_state": 0,
                "vwireEnabled": false,
                "uplink_table": [],
                "radio_table_stats": [],
                "num_sta": 5,
                "user-num_sta": 3,
                "user-wlan-num_sta": 0,
                "guest-num_sta": 2,
                "guest-wlan-num_sta": 0,
                "x_has_ssh_hostkey": true
            }
        ]
    });

    // Deserialize the JSON into a DeviceListResponse
    let response: DeviceListResponse = serde_json::from_value(json_data).unwrap();

    // Verify the deserialized data
    assert_eq!(response.meta.rc, "ok");
    assert_eq!(response.data.len(), 1);

    let device = &response.data[0];
    assert_eq!(device.model, "U7MSH");
    assert_eq!(device.name, "Test AP");
    assert_eq!(device.type_field, "uap");
    assert_eq!(device.mac, "00:11:22:33:44:55");
    assert_eq!(device.ip, "192.168.1.100");
    assert_eq!(device.num_sta, 5);
    assert_eq!(device.user_num_sta, 3);
    assert_eq!(device.guest_num_sta, 2);

    // Check radio table
    assert_eq!(device.radio_table.len(), 2);

    // First radio
    let radio0 = &device.radio_table[0];
    assert_eq!(radio0.radio, "ng");
    assert_eq!(radio0.name, "wifi0");
    assert_eq!(radio0.antenna_gain, 6);
    assert_eq!(radio0.nss, 2);
    assert_eq!(radio0.tx_power_mode, "low");

    // Second radio
    let radio1 = &device.radio_table[1];
    assert_eq!(radio1.radio, "na");
    assert_eq!(radio1.name, "wifi1");
    assert_eq!(radio1.antenna_gain, 4);
    assert_eq!(radio1.nss, 2);
    assert_eq!(radio1.tx_power_mode, "low");
    assert!(radio1.is_11ac.unwrap());
}

#[test]
fn test_radio_table_value_handling() {
    // Test that the RadioTable correctly handles different value types
    let json_data = json!({
        "ht": 20,  // integer value
        "channel": "36",  // string value
        "antenna_gain": 6,
        "builtin_antenna": true,
        "vwire_enabled": false,
        "max_txpower": 23,
        "min_rssi_enabled": false,
        "builtin_ant_gain": 6,
        "radio": "na",
        "nss": 2,
        "tx_power_mode": "medium",
        "name": "wifi1",
        "min_txpower": 8,
        "radio_caps": 1234,
        "antenna_id": 1,
        "radio_caps2": 5678,
        "current_antenna_gain": 6
    });

    // Deserialize the JSON into a RadioTable
    let radio: RadioTable = serde_json::from_value(json_data).unwrap();

    // Check that the Value types are handled correctly
    assert_eq!(radio.ht.as_u64().unwrap(), 20);
    assert_eq!(radio.channel.as_str().unwrap(), "36");
}

#[test]
fn test_radio_from_radio_table() {
    // Create a RadioTable instance from JSON
    let json_data = json!({
        "ht": 20,
        "channel": 36,
        "antenna_gain": 6,
        "builtin_antenna": true,
        "vwire_enabled": false,
        "max_txpower": 23,
        "min_rssi_enabled": false,
        "builtin_ant_gain": 6,
        "radio": "na",
        "nss": 2,
        "tx_power_mode": "medium",
        "name": "wifi1",
        "min_txpower": 8,
        "radio_caps": 1234,
        "antenna_id": 1,
        "radio_caps2": 5678,
        "current_antenna_gain": 6,
        "is_11ac": true,
        "max_chan_cntr_frq": 5500,
        "min_chan_cntr_frq": 5180,
        "channel_optimization_enabled": true,
        "has_dfs": true,
        "has_fccdfs": false,
        "min_rssi": -90
    });

    let radio_table: RadioTable = serde_json::from_value(json_data).unwrap();

    // Convert RadioTable to Radio
    let radio = Radio::from(radio_table);

    // Verify Radio fields
    assert_eq!(radio.antenna_gain, 6);
    assert!(radio.builtin_antenna);
    assert_eq!(radio.builtin_antenna_gain, 6);
    assert_eq!(radio.channel_width, 20);
    assert!(radio.channel_optimization_enabled);
    assert_eq!(radio.current_channel, 36);
    assert_eq!(radio.current_gain, 6);
    assert!(radio.has_dfs);
    assert!(!radio.has_fccdfs);
    assert_eq!(radio.antenna_id, 1);
    assert_eq!(radio.radio_type, RadioType::AC5);
    assert_eq!(radio.max_channel, 100); // 5500 MHz corresponds to channel 100
    assert_eq!(radio.max_power, 23);
    assert_eq!(radio.min_channel, 36); // 5180 MHz corresponds to channel 36
    assert_eq!(radio.min_power, 8);
    assert_eq!(radio.min_rssi, -90);
    assert_eq!(radio.name, "wifi1");
    assert_eq!(radio.power_mode, RadioPowerMode::Medium);
    assert_eq!(radio.spacial_streams, 2);
}
