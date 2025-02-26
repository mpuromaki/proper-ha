# Proper Home Automation

## Overview

Proper Home Automation is standard for home automation system and nodes. It defines protocol
for communication between nodes and automation servers, service discovery and the network layout used
for home automation system.

Aim of this standard is to define easy to implement, local, open and powerful protocol for home
automation systems. Protocol is defined in Rust, but it supports implementations on hobbyists
systems like arduino or esp32.

Proper is based on IPv6 and natively supports sleepy battery-operated nodes. High level model is that
nodes poll the automation servers for pushing measurement data and also check if there's any messages
for them. This approach allows the nodes to define how much they sleep to conserver power.

The networks is fully isolated IPv6 network which connects nodes and automation servers. The automation
servers are allowed to be also connected to external networks with other interfaces, but the automation
network should not be connected to any external networks. Proper supports Thread networks via Thread
border gateways. Higher power nodes are supported via Wifi or Ethernet connectivity. IPv4 may be
supported for these higher power nodes, but IPv6 is preferred due to it's native redundant routing
capability.

Service discovery is via mDNS and DNS-SD. Proper uses well known ports and services which the nodes
utilize to locate the automation servers.

Secrets management is made to be as simple as possible for the users. One shared master secret is
defined, which is used to derive:
- Proper network id
- Proper node registration api-key
- Thread network key
- Wifi SSID and password

Node is configured with the master secret (via serial, QR-code, bluetooth, light?) and then it derives
the necessary keys for itself. Node uses that information to join the network and register itself to
automation servers. The automation servers then ask user for permission to add new node to the system.
After user approval, the automation server creates outbox message for the node which allows it to join
the automation system. While waiting for approval, the node polls the server for messages.

Redundancy is supported on multiple levels and reliability is paramount. IPv6 is used to allow multiple
routes for the nodes for reaching the automation servers. Thread network supports these redundant features
for power restricted nodes. Wifi also supports multiple access points for redundant access to the network.
Service discovery natively supports multiple servers (and even multiple proper networks). No connection
to internet should ever be required, no single point of failure should ever exist.