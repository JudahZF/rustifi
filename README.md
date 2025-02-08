# rustifi

WORK IN PROGRESS

A rust API library for Unifi Controller.


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
