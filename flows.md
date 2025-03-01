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
    Node->>+Server: ProperFrame::Register
    Server->>Node: ProperFrame::AckStatus(Good)
    Server-->>+User: inform new node
    Node->>Server: ProperFrame::Poll
    Server->>Node: ProperFrame::AckStatus(Good)
    User-->>-Server: accept node
    Node->>Server: ProperFrame::Poll
    Server->>-Node: ProperFrame::RegisterAccepted(mid:123)
    Node->>Server: ProperFrame(Ack:123)
    note over Node,Server: ready to operate 
```

## Node pushing and polling

This flow describes how Nodes push data to server and poll for pending messages
if server indicates such messages are available via pnd boolean flag. Also this
demonstrates how ack field is used in ProperFrame and how Server responds with
AckStatus messages.

```mermaid
sequenceDiagram
    participant Node
    participant Server


    note over Node: Sleep
    Node->>+Server: ProperFrame::Push
    Server->>Node: ProperFrame::AckStatus(Good), pnd=false
    note over Node: Sleep
    Node->>+Server: ProperFrame::Push
    Server->>Node: ProperFrame::AckStatus(Good), pnd=true
    Node->>+Server: ProperFrame::Poll
    Server->>Node: ProperFrame::_, pnd=true, mid=123
    Node->>+Server: ProperFrame::Poll, ack=123
    Server->>Node: ProperFrame::_, pnd=false, mid=124
    note over Node: Sleep
    Node->>+Server: ProperFrame::Push, ack=124
```