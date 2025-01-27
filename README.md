# rustifi

WORK IN PROGRESS

A rust API library for Unifi Controller.


## To Do

- [ ] Implement some access points
- [ ] Implement some switches
- [ ] Ensure compliance with Rust API Guidelines Checklist, Including traits
    - Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Display, Default
    - From, TryFrom, AsRef, AsMut
    - Errors
- [ ] Tests
- [ ] Documentation
- [ ] Examples

## Layout & Features

- [ ] Devices
    - [ ] Basic Information
    - [x] Get
    - [ ] Update
    - [ ] AP
    - [ ] Group
    - [x] Model
    - [x] Name
    - [ ] Radio
        - [ ] Channel
        - [ ] Width
        - [ ] Power
        - [ ] Meshing
    - [ ] Number of Clients
    - [x] CPU
    - [x] Memory
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
- [ ] U6-Mesh
- [ ] U6-Mesh-Pro
- [ ] UWB-XG
- [ ] UAP-XG

### Switch

- [ ] USW-Pro-Aggregation
- [ ] USW-Aggregation
