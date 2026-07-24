# Specification 04: Stateless Sphinx Mixnets & Steganographic Egress 
## 1. Overview
To neutralize global passive observers, metadata harvesting, and timing correlation attacks, **The Remote Viewer** routes all peer-to-peer synchronization traffic, regional news streams, and document requests through a stateless Sphinx mixnet architecture backed by adaptive steganographic cover-traffic.
## 2. Stateless Sphinx Packet Architecture
 * **Onion-Wrapped Encapsulation:** All protocol payloads are encapsulated inside fixed-size, cryptographically unmodifiable Sphinx packets utilizing layered symmetric encryption.
 * **Stateless Relay Processing:** Intermediate mix-nodes process and unwrap routing headers using high-speed symmetric cryptographic operations without maintaining session memory, connection state tables, or routing logs, ensuring absolute forward secrecy.
## 3. Adaptive Steganographic Cover-Traffic Generation
 * **Markov-Modeled Noise:** Static or predictable dummy padding is replaced with dynamic, Markov-modeled steganographic traffic generation.
 * **Indistinguishable Egress:** Client binaries continuously inject background noise packets that mimic standard operating system network chatter, making real packet volume and query bursts indistinguishable from normal background activity to global passive observers.
## 4. Multi-Hop Bi-Directional Loop Routing
 * **Decoy Routing Loops:** Packets are forced through multi-layered, bi-directional decoy routing loops across the mix-node mesh prior to reaching their final regional destination.
 * **Metadata Destruction:** Variable-delay mixing queues and loop routing completely shatter timing correlation vectors, preventing external adversaries from linking origin IP addresses, node endpoints, or specific document CIDs.
