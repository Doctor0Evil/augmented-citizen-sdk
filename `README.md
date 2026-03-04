# Augmented Citizen SDK

**SDK for Augmented-Citizen applications with built-in sovereignty, Bostrom DID authentication, and NDM-aware session management**

[![License: ASL-1.0](https://img.shields.io/badge/License-ASL--1.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/augmented-citizen-sdk.svg)](https://crates.io/crates/augmented-citizen-sdk)
[![Docs](https://docs.rs/augmented-citizen-sdk/badge.svg)](https://docs.rs/augmented-citizen-sdk)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xef0f6e9d8c5b1a3f2e7d6c5b4a3f2e1d0c9b8a79-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`augmented-citizen-sdk` is the **application development layer** for the ALN Sovereign Stack. It provides multi-language SDKs (Rust, JavaScript, Kotlin, Mojo) that enable Augmented-Citizen applications to interact with the ALN ecosystem while maintaining sovereignty, security, and NDM awareness.

This guarantees:
- **Built-in Sovereignty** - All apps inherit ALN security by default
- **Bostrom DID Auth** - Passwordless, cryptographic identity management
- **NDM-Aware Sessions** - Privilege escalation gated by NDM scores
- **Offline-First** - Apps work without network, anchor later
- **AI-Chat Security** - Secure AI integrations against intrusion

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    AUGMENTED-CITIZEN APPS                        │
│         (Healthcare / Ecology / Diagnostics / Care)              │
└────────────────────────────┬────────────────────────────────────┘
                             │ SDK Calls
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    augmented-citizen-sdk                         │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Multi-Language SDK (Rust / JS / Kotlin / Mojo)           │  │
│  └───────────────────────────────────────────────────────────┘  │
│          │                  │                  │                │
│          ▼                  ▼                  ▼                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │DID Auth      │  │Session Mgmt  │  │NDM Awareness │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│          │                  │                  │                │
│          └──────────────────┼──────────────────┘                │
│                             ▼                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Sovereignty Core Integration (eval_aln_envelope)         │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    ALN ECOSYSTEM                                 │
│         (sovereigntycore → ROW/RPM → Googolswarm)                │
└─────────────────────────────────────────────────────────────────┘
```

## Supported Languages

| Language | Package | Status |
|----------|---------|--------|
| Rust | `augmented-citizen-sdk` (crates.io) | ✅ Stable v1.0 |
| JavaScript/TypeScript | `@aln/augmented-citizen-sdk` (npm) | ✅ Stable v1.0 |
| Kotlin | `io.aln:augmented-citizen-sdk` (Maven) | ✅ Stable v1.0 |
| Mojo | `aln.augmented_citizen` (magic) | 🔄 Beta v0.9 |

## Key Components

| Component | Description |
|-----------|-------------|
| `AuthManager` | Bostrom DID authentication with multi-sig support |
| `SessionManager` | NDM-aware session lifecycle management |
| `CapabilityClient` | Request and verify ALN capabilities |
| `OfflineSync` | Offline-first data synchronization |
| `AIChatBridge` | Secure AI-Chat platform integration |
| `NDMMonitor` | Real-time NDM score visibility |

## Quick Start

### Rust

```rust
use augmented_citizen_sdk::{CitizenApp, AppConfig, AuthManager};

let config = AppConfig::new("my-app", "v1.0.0");
let mut app = CitizenApp::new(config)?;

// Authenticate with Bostrom DID
let auth = AuthManager::new();
let session = auth.authenticate("bostrom1citizen")?;

// Make sovereignty-aware API call
let result = app.call_sovereign_api("/api/ecological/mission", &session)?;
```

### JavaScript/TypeScript

```javascript
import { CitizenApp, AuthManager } from '@aln/augmented-citizen-sdk';

const app = new CitizenApp({ name: 'my-app', version: '1.0.0' });
const auth = new AuthManager();

// Authenticate with Bostrom DID
const session = await auth.authenticate('bostrom1citizen');

// Make sovereignty-aware API call
const result = await app.callSovereignAPI('/api/ecological/mission', session);
```

### Kotlin (Android)

```kotlin
import io.aln.sdk.CitizenApp
import io.aln.sdk.AuthManager

val app = CitizenApp("my-app", "1.0.0")
val auth = AuthManager()

// Authenticate with Bostrom DID
val session = auth.authenticate("bostrom1citizen")

// Make sovereignty-aware API call
val result = app.callSovereignAPI("/api/ecological/mission", session)
```

## NDM-Aware Session Management

| NDM Score | Session State | Capabilities Available |
|-----------|---------------|------------------------|
| 0.0-0.3 | Normal | Full capabilities |
| 0.3-0.6 | Monitoring | Read-only, no new privileges |
| 0.6-0.8 | ObserveOnly | View only, no actions |
| 0.8-1.0 | Frozen | Session suspended |

## Security Properties

- **Passwordless Auth** - Bostrom DID cryptographic authentication
- **NDM-Gated Privileges** - Session capabilities scale with NDM score
- **Offline-First** - Apps work without network connectivity
- **Zes-Encrypted** - All communications quantum-safe encrypted
- **Ledger-Anchored** - All actions logged to ROW/RPM

## Governance

All SDK operations require:
1. **Bostrom DID** - Cryptographic identity for all users
2. **NDM Awareness** - Session privileges respect NDM scores
3. **ROW/RPM Logging** - All actions logged for audit
4. **Non-Weaponization** - SDK prevents weapon-like app patterns

**Hex-Stamp Attestation:** `0xef0f6e9d8c5b1a3f2e7d6c5b4a3f2e1d0c9b8a79f8e7d6c5b4a3928170f6e5d4`  
**Ledger Reference:** `row:augmented-citizen-sdk:v1.0.0:2026-03-04`  
**Organichain Anchor:** `org:pending`

## License

ALN Sovereign License (ASL-1.0) - See LICENSE for details.

---

**⚠️ Citizen Notice:** This SDK enforces sovereignty by default. Apps built with this SDK inherit ALN security properties and cannot bypass NDM or capability checks.
```
