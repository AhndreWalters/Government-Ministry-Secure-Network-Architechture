# Government-Ministry-Secure-Network-Architecture

## Overview

This repository contains the secure network architecture design report for the Ministry
of Education, Youth, Sports & Culture of Grenada. The design was built using Cisco
Packet Tracer and implements VLAN segmentation, firewall rules, redundancy, and
scalable infrastructure to protect against threats identified in prior OSINT and risk
assessment capstones. This project was completed as the Network Design Capstone for the
Cyber Nation x Protexxa Cybersecurity Bootcamp.

<img src="https://github.com/AhndreWalters/Government-Ministry-Secure-Network-Architechture/blob/main/AhndreWalters_NETWORK_Capstone.png"/>

## Ministry Designed For

- **Ministry:** Ministry of Education, Youth, Sports & Culture
- **Government:** Government of Grenada
- **Tool Used:** Cisco Packet Tracer

## Objectives

- Design a secure and resilient network architecture for a government ministry.
- Isolate each department using VLANs to enforce the principle of least privilege.
- Fully separate guest wireless access from the internal ministry network.
- Build redundancy into the design to eliminate single points of failure.
- Provide centralized access to core infrastructure services.
- Define clear firewall and ACL rules to control and restrict traffic flow.
- Balance strong security controls with smooth day-to-day ministry operations.

## What is Network Segmentation and Why Does it Matter?

Network segmentation is the practice of dividing a network into smaller, isolated
sections so that devices in one section cannot freely communicate with devices in
another unless explicitly permitted. In a government ministry environment, this is
critical because it prevents a threat from spreading across the entire network if one
department is compromised.

For example, if ransomware infects a workstation in the Public Relations department,
segmentation ensures it cannot automatically reach financial records or the Minister's
Office systems. This directly addresses one of the key vulnerabilities identified in
the risk assessment: poor network segmentation that enabled lateral movement.

## Network Topology

The Ministry's network follows a redundant star topology. Each department connects
through access switches to dual core switches, which manage inter-VLAN traffic routing.
This layout ensures departments operate independently while remaining securely
interconnected.

| Department | VLAN ID | Subnet | Purpose |
|---|---|---|---|
| Minister's Office | 10 | 192.168.10.0/24 | Executive staff access |
| Finance & Administration | 20 | 192.168.20.0/24 | Financial and administration services |
| IT Department | 30 | 192.168.30.0/24 | Network and server management |
| Public Relations | 40 | 192.168.40.0/24 | Media and communications |
| Internal Services | 50 | 192.168.50.0/24 | HR, logistics, and procurement |
| Guest Wi-Fi | 60 | 192.168.60.0/24 | Internet-only access for visitors |
| Servers | 99 | 192.168.99.0/24 | Critical servers and services |

## Key Network Devices

| Device Type | Model | Purpose |
|---|---|---|
| Router | Cisco 2911 | Handles inter-VLAN routing and ACLs |
| Core Switches | Cisco 2960-24TT | Provides redundancy and traffic distribution |
| Access Switches | Cisco 2960-24TT | Connects departments to core switches |
| Servers | Server-PT | Hosts DHCP, DNS, Mail, and App services |
| End Devices | PC-PT, Laptop-PT | User endpoints across departments |
| Wireless Router | Cisco WRT300N | Provides isolated guest Wi-Fi access |

## Core Infrastructure Services

| Service | Hostname | IP Address | Purpose |
|---|---|---|---|
| DHCP Server | DHCP-Server | 192.168.99.10 | Assigns IP addresses to all departments |
| DNS Server | DNS-Server | 192.168.99.13 | Resolves internal and external domain names |
| Mail Server | Mail-Server | 192.168.99.11 | Manages ministry email communications |
| App Server | App-Server | 192.168.99.12 | Runs internal ministry applications |

All servers are housed within VLAN 99, a dedicated server subnet that is isolated from
department traffic and only accessible through explicitly permitted firewall rules.

## Firewall and ACL Rules

The default firewall policy is deny all. Only traffic that is explicitly permitted
by the rules below is allowed through. This approach minimizes the attack surface and
ensures that no unauthorized communication can occur between segments.

| Rule | Protocol | Port(s) | Source | Destination | Action | Reason |
|---|---|---|---|---|---|---|
| 1 | UDP | 53 | VLANs 10-50 | DNS Server | Allow | DNS lookups for all departments |
| 2 | UDP | 67-68 | VLANs 10-50 | DHCP Server | Allow | Automatic IP address assignment |
| 3 | TCP | 25 | Internal VLANs | Mail Server | Allow | Internal email traffic |
| 4 | TCP | 8080 | Internal VLANs | Application Server | Allow | Access to ministry web applications |
| 5 | ICMP | Any | VLAN 30 (IT only) | Any | Allow | Network testing and troubleshooting |
| 6 | ANY | Any | Guest VLAN | Internal VLANs | Deny | Full guest isolation from internal network |

## Redundancy and High Availability

Two key redundancy mechanisms were implemented to ensure the network remains
operational even during hardware failures.

**Dual Core Switches**
If one core switch fails, the second takes over immediately, ensuring uninterrupted
traffic flow between departments and servers.

**EtherChannel Links**
Multiple physical links are bundled between core switches, increasing bandwidth and
providing automatic failover if a single link goes down.

## Scalability

The network was designed to grow alongside the Ministry without requiring major
reconfiguration. New VLANs can be added for additional departments, access switches
can be connected to existing core switches to accommodate more devices, and the IP
addressing scheme leaves sufficient address space for future allocation.

## Security and Operational Balance

Strong security does not have to come at the cost of usability. This design reflects
that principle throughout.

- VLAN separation keeps sensitive departments isolated from one another without
  preventing authorized collaboration.
- Guest Wi-Fi provides internet access to visitors without any path to internal systems.
- IT staff retain the ability to run diagnostic tools such as ping and traceroute across
  the network for troubleshooting purposes.
- Access control rules ensure that no user or device has more access than their role
  requires, directly applying the principle of least privilege.

## Architecture Justification

This design was built in direct response to the risks identified in the preceding OSINT
and risk assessment capstones. Poor network segmentation was a critical finding that
enabled lateral movement during the ransomware scenario. The VLAN-based architecture
closes that gap entirely. The guest isolation rule and the deny-all firewall policy
address the social engineering threat by ensuring that even if a visitor's device is
compromised, it cannot reach any internal ministry resource. Redundancy ensures that
operational continuity is maintained even under attack or hardware failure.

## License

This project is for academic purposes only and is not affiliated with the Government of
Grenada or the Ministry of Education, Youth, Sports & Culture. All network designs,
configurations, and recommendations are theoretical applications created for educational
use.

<strong>[&copy; 2025 Ahndre Walters](https://github.com/AhndreWalters/Government-Ministry-Secure-Network-Architechture/blob/main/LICENSE) · Created as part of the Cyber Nation x Protexxa Cybersecurity Bootcamp (Cohort 1 - Grenada) · Network Design Capstone Assignment · Ministry of Education, Youth, Sports & Culture</strong>
