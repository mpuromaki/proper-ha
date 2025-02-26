# Proper Messaging Flows

This document is meant to be the reference for specific messaging flows.

## Registering new node

This flow is used when new node is registered to the automation system.
This can be entirely new device or factory resetted existing device.

```mermaid
sequenceDiagram
    participant Node
    participant Server
    participant User

    User-->>Server: fetch master secret
    Server-->>+User: master secret
    User-->>-Node: master secret
    Node->>+Server: Register
    Server-->>+User: inform new node
    Node-->>Server: poll for messages
    User-->>-Server: accept node
    Server->>-Node: RegisterAccepted
    note over Node,Server: ready to operate 
```

