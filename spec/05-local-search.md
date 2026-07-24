# Specification 05: Edge-Native Local Search & FTS5 Indexing 
## 1. Overview
To ensure absolute user privacy and offline operational capability, **The Remote Viewer** executes all document parsing, full-text indexing, and search queries entirely on-device. No search queries, telemetry, index metrics, or match frequencies are ever transmitted to external servers or peers.
## 2. Local PDF Text Extraction Engine
 * **Agnostic Binary Processing:** Utilizes embedded, open-source C/Rust bindings (MuPDF/Poppler libraries compiled statically into the client binary) to extract raw text streams directly from local declassified document stores.
 * **Air-Gapped OCR Fallback:** Integrates local Tesseract-based optical character recognition for scanned image-only PDF pages, ensuring complete ingestion capability without reliance on external web services or APIs.
## 3. Edge-Native Full-Text Indexing (SQLite FTS5 / Tantivy)
 * **Structured Local Database:** Extracted text fragments, metadata tags, document CIDs, and publication timestamps are written directly to a local, encrypted SQLite database utilizing the FTS5 extension (or embedded Tantivy search engine indices).
 * **Sub-Millisecond Querying:** Inverted index structures enable rapid, Boolean-compliant phrase matching across millions of unredacted pages locally on resource-constrained edge hardware.
## 4. Zero-Telemetry Search Guarantees
 * **Complete Isolation:** Search execution happens in a sandboxed local memory space. Query strings, autocomplete patterns, and result selection logs remain strictly ephemeral or local.
 * **Zero-Knowledge Retrieval:** Searching archival blocks requires zero network requests, entirely shielding the user's research interests and query history from global passive observers and network-layer monitors.
