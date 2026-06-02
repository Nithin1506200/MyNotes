# TLS Handshake Flow - Detailed Visual Guide

A comprehensive guide to TLS 1.2 and TLS 1.3 handshake processes with visual diagrams.

---

## Table of Contents

1. [What is a TLS Handshake?](#what-is-a-tls-handshake)
2. [TLS vs SSL Handshakes](#tls-vs-ssl-handshakes)
3. [When Does a TLS Handshake Occur?](#when-does-a-tls-handshake-occur)
4. [TLS 1.2 Handshake](#tls-12-handshake)
5. [TLS 1.3 Handshake](#tls-13-handshake)
6. [Side-by-Side Comparison](#side-by-side-comparison)
7. [Key Exchange Methods](#key-exchange-methods)
8. [Session Resumption](#session-resumption)
9. [Handshake Failures](#handshake-failures)
10. [Best Practices](#best-practices)
11. [Quick Debug Commands](#quick-debug-commands)
12. [TLS Mathematics](#tls-mathematics)

---

## What is a TLS Handshake?

TLS (Transport Layer Security) is an encryption and authentication protocol designed to secure Internet communications. A TLS handshake is the process that initiates a secure communication session. During this process:

- Two parties acknowledge each other
- Verify identities using certificates
- Negotiate cryptographic algorithms
- Agree on session keys for symmetric encryption

**Key Point:** TLS handshakes occur **after** a TCP connection has been established via a TCP three-way handshake.

## TLS vs SSL Handshakes

| Protocol | Status | Notes |
|----------|--------|-------|
| SSL 1.0-3.0 | Deprecated | Superseded by TLS, vulnerable to POODLE |
| TLS 1.0 | Deprecated | Officially deprecated in 2021 |
| TLS 1.1 | Deprecated | Officially deprecated in 2021 |
| TLS 1.2 | Supported | Widely deployed, slower |
| TLS 1.3 | Recommended | Latest version, faster and more secure |

While "SSL" is still widely used in terminology, modern protocols are all TLS.

## When Does a TLS Handshake Occur?

A TLS handshake occurs when:

- A user navigates to an HTTPS website
- API calls over HTTPS are initiated
- DNS over HTTPS queries are made
- Any other HTTPS communication begins

---

## TLS 1.2 Handshake

### Overview

The TLS 1.2 handshake involves **2 Round-Trip Times (RTTs)** and consists of the following phases:

1. Hello phase (key exchange negotiation)
2. Certificate exchange and authentication
3. Key generation
4. Finished phase (confirmation)

### Complete Message Flow

```
Client (Browser)                           Server (Web Server)
     |                                           |
     |  (1) ClientHello                          |
     |  ────────────────────────────────────────>│
     |  - Protocol version: TLS 1.2              |
     |  - Client random (32 bytes)               |
     |  - Session ID                             |
     |  - Cipher suites (ordered by preference)  |
     |  - Compression methods                    |
     |  - Extensions (SNI, ALPN, etc.)           |
     |                                           |
     |                          (2) ServerHello  |
     |  <────────────────────────────────────────│
     |                     - Chosen TLS version  |
     |                     - Server random       |
     |                     - Session ID          |
     |                     - Chosen cipher suite |
     |                     - Compression method  |
     |                                           |
     |                        (3) Certificate    |
     |  <────────────────────────────────────────│
     |            - Server's certificate chain   |
     |            - X.509 format                 |
     |                                           |
     |                  (4) ServerKeyExchange    |
     |  <────────────────────────────────────────│
     |                    (Optional - for DHE)   |
     |              - Diffie-Hellman parameters  |
     |              - Digital signature          |
     |                                           |
     |                  (5) CertificateRequest   |
     |  <────────────────────────────────────────│
     |                          (Optional - for  |
     |                       client auth)        |
     |                                           |
     |                    (6) ServerHelloDone    |
     |  <────────────────────────────────────────│
     |                                           |
     |- Client validates certificate             |
     |- Checks expiration                        |
     |- Verifies domain match                    |
     |- Validates certificate chain              |
     |- Generates pre-master secret              |
     |                                           |
     |  (7) ClientKeyExchange                    |
     |  ────────────────────────────────────────>│
     |  - Pre-master secret (encrypted with      |
     |    server's public key from cert)         |
     |                                           |
     |  (8) ChangeCipherSpec                     |
     |  ────────────────────────────────────────>│
     |  - Signal to switch to encrypted mode     |
     |                                           |
     |  (9) Finished                             |
     |  ────────────────────────────────────────>│
     |  - Encrypted with new session keys        |
     |  - Hash of all handshake messages         |
     |                                           |
     |- Server derives session keys              |
     |- Verifies encrypted Finished message      |
     |                                           |
     |                   (10) ChangeCipherSpec   |
     |  <────────────────────────────────────────│
     |                                           |
     |                        (11) Finished      |
     |  <────────────────────────────────────────│
     |             - Encrypted confirmation      |
     |                                           |
     |- Client verifies server's Finished        |
     |                                           |
     |  ══════════════════════════════════════   |
     |      HANDSHAKE COMPLETE - 2 RTT           |
     |  ══════════════════════════════════════   |
     |                                           |
     |  Application Data (HTTP Request)          |
     |  ────────────────────────────────────────>│
     |           [Encrypted with AES]            |
     |                                           |
     |         Application Data (HTTP Response)  |
     |  <────────────────────────────────────────│
     |           [Encrypted with AES]            |
     |                                           |
```

### Timeline Visualization

```
RTT 0    RTT 1                        RTT 2
  |        |                            |
  |- CH -->|                            |
  |        |- SH, Cert, SKE, SHD ------>|
  |        |<- CKE, CCS, Fin -----------|
  |        |-- CCS, Fin --------------->|
  |        |                            |- Data Transfer
  |        |                            |

Legend:
CH  = ClientHello
SH  = ServerHello
SKE = ServerKeyExchange
SHD = ServerHelloDone
CKE = ClientKeyExchange
CCS = ChangeCipherSpec
Fin = Finished
```

### Key Points - TLS 1.2

| Aspect | Details |
|--------|---------|
| **Round Trips** | 2 RTT |
| **Messages** | 11 messages total |
| **Encryption Starts** | After ChangeCipherSpec |
| **Key Exchange** | RSA or DHE/ECDHE |
| **Forward Secrecy** | Only with DHE/ECDHE |

---

## TLS 1.3 Handshake

### Overview

TLS 1.3 significantly improves on TLS 1.2 by:

- Reducing handshake to **1 RTT**
- Removing weak cipher suites and RSA key exchange
- Encrypting more of the handshake
- Improving forward secrecy

### Complete Message Flow

```
Client                                     Server
     |                                           |
     |  (1) ClientHello                          |
     |  ────────────────────────────────────────>│
     |  - Protocol version: TLS 1.3              |
     |  - Client random                          |
     |  - Cipher suites (reduced list)           |
     |  - Key share (Client's DH public key)     |
     |  - PSK (if resuming)                      |
     |  - Extensions                             |
     |                                           |
     |- Server processes ClientHello             |
     |- Selects cipher suite                     |
     |- Generates server DH key pair             |
     |- Derives handshake keys                   |
     |                                           |
     |                          (2) ServerHello  |
     |  <────────────────────────────────────────│
     |                     - Server random       |
     |                     - Chosen cipher       |
     |                     - Key share (Server's |
     |                       DH public key)      |
     |                                           |
     |           {Everything below encrypted}    |
     |                                           |
     |                  {3} EncryptedExtensions  |
     |  <────────────────────────────────────────│
     |                                           |
     |                     {4} Certificate       |
     |  <────────────────────────────────────────│
     |              {Server's certificate chain} |
     |                                           |
     |                {5} CertificateVerify      |
     |  <────────────────────────────────────────│
     |              {Signature over handshake}   |
     |                                           |
     |                      {6} Finished         |
     |  <────────────────────────────────────────│
     |        {MAC of entire handshake}          |
     |                                           |
     |- Client validates everything              |
     |- Derives application keys                 |
     |                                           |
     |  {7} Finished                             |
     |  ────────────────────────────────────────>│
     |        {Client's handshake MAC}           |
     |                                           |
     |  ══════════════════════════════════════   |
     |     HANDSHAKE COMPLETE - 1 RTT!           |
     |  ══════════════════════════════════════   |
     |                                           |
     |  {Application Data}                       |
     |  ────────────────────────────────────────>│
     |  <────────────────────────────────────────│
     |                                           |

{} = Encrypted with handshake keys
```

### Timeline Visualization

```
RTT 0              RTT 1
  |                  |
  |- CH + KeyShare ->|
  |                  |- SH, {Cert, CV, Fin}
  |<- {Fin} ---------|
  |                  |- Application Data!
  |                  |

Legend:
CH  = ClientHello
SH  = ServerHello
CV  = CertificateVerify
Fin = Finished
{}  = Encrypted
```

### Key Points - TLS 1.3

| Aspect | Details |
|--------|---------|
| **Round Trips** | 1 RTT (or 0-RTT with resumption) |
| **Messages** | 7 messages total |
| **Encryption Starts** | After ServerHello |
| **Key Exchange** | DHE/ECDHE only |
| **Forward Secrecy** | Always enabled |

---

## TLS 1.3 with 0-RTT (Session Resumption)

### Overview

For returning visitors, TLS 1.3 supports **0-RTT** mode, allowing application data to be sent immediately with the first message.

### Message Flow

```
Client                                     Server
     |                                           |
     |  (1) ClientHello + Early Data             |
     |  ────────────────────────────────────────>│
     |  - PSK from previous session              |
     |  - {Application Data}                     |
     |    (encrypted with PSK)                   |
     |                                           |
     |  [DATA SENT IMMEDIATELY - 0 RTT!]         |
     |                                           |
     |                          (2) ServerHello  |
     |  <────────────────────────────────────────│
     |                     {Early Data Accept}   |
     |                     {Finished}            |
     |                     {Application Data}    |
     |                                           |
     |  Full duplex communication established!   |
     |                                           |

WARNING: 0-RTT has replay attack risks for non-idempotent requests
```

### Key Points - 0-RTT

| Aspect | Details |
|--------|---------|
| **Round Trips** | 0 RTT (for early data) |
| **Requirement** | Previous connection with stored PSK |
| **Security Risk** | Replay attacks on non-idempotent requests |
| **Use Case** | Safe for GET requests, avoid for POST/PUT |

---

## Side-by-Side Comparison

### Speed Comparison

```
---------------------------------------------------------------
|                    Handshake Duration                         |
|---------------------------------------------------------------|
|                                                               |
|  TLS 1.2:  ████████████ (2 RTT = ~200ms on 100ms latency)    |
|                                                               |
|  TLS 1.3:  ██████ (1 RTT = ~100ms)     50% faster           |
|                                                               |
|  TLS 1.3   ███ (0-RTT = ~50ms)         75% faster           |
|  Resume:                                                      |
|                                                               |
---------------------------------------------------------------
```

### Message Count Comparison

| Version | Messages Exchanged | Encrypted From |
|---------|-------------------|----------------|
| TLS 1.2 | 11 messages | Last 2 only |
| TLS 1.3 | 7 messages | From ServerHello |
| TLS 1.3 0-RTT | 5 messages | Almost all |

### Feature Comparison

| Feature | TLS 1.2 | TLS 1.3 |
|---------|---------|---------|
| RSA Key Exchange | Yes | No |
| Weak Cipher Suites | Many | Removed |
| Compression | Supported | Removed |
| Custom DHE Groups | Supported | Limited |
| Forward Secrecy | Optional | Mandatory |
| Encrypted Extensions | No | Yes |
| Post-Handshake Auth | No | Yes |

---

## Key Exchange Methods

### RSA Key Exchange (TLS 1.2 Only - Deprecated)

```
Client                                Server
   |                                    |
   |- Generates random pre-master      |
   |  secret                            |
   |                                    |
   |  Pre-master secret                 |
   |  (Encrypted with server's          |
   |   public key from cert)            |
   |  ────────────────────────────────>|
   |                                    |
   |                                    |- Decrypts with
   |                                    |  private key
   |                                    |
   |- Both derive session keys from:   |
   |  - Pre-master secret               |
   |  - Client random                   |
   |  - Server random                   |
   |                                    |

[No Forward Secrecy]
If server's private key leaks, ALL past sessions compromised!
```

**Status:** Removed in TLS 1.3 due to lack of forward secrecy.

### ECDHE (Recommended - TLS 1.2 & TLS 1.3)

```
Client                                Server
   |                                    |
   |- Generate ephemeral               |- Generate ephemeral
   |  ECDH key pair                    |  ECDH key pair
   |                                    |
   |  Client's public key               |
   |  ────────────────────────────────>|
   |                                    |
   |          Server's public key       |
   |  <────────────────────────────────|
   |                                    |
   |- Compute shared secret             |- Compute shared secret
   |  using server's public +           |  using client's public +
   |  own private key                   |  own private key
   |                                    |
   |- Derive session keys               |- Derive session keys
   |                                    |
   |- Discard private keys!             |- Discard private keys!
   |                                    |

[Perfect Forward Secrecy]
Even if server's long-term key leaks, past sessions remain safe!
```

**Status:** Default and mandatory in TLS 1.3.

---

## Session Resumption

### Session IDs (TLS 1.2)

```
Initial Connection:
  Client ──── ClientHello ────> Server
  Client <─── SessionID=123 ─── Server
         ... full handshake ...

Resumption (within timeout):
  Client ──── ClientHello + SessionID=123 ──> Server
  Client <─── Abbreviated Handshake ──────── Server
         (Skips certificate exchange)
```

### Session Tickets (TLS 1.2 & TLS 1.3)

```
Initial Connection:
  Client <─── Encrypted Session State ─── Server
         (Encrypted with server's ticket key)

Resumption:
  Client ──── Ticket ──> Server
  Server ──── Decrypts ticket
         ──── Validates & resumes
```

### Pre-Shared Key (PSK - TLS 1.3)

```
After first connection:
  Client stores: PSK + PSK identity

Resumption:
  Client ──── PSK identity ──> Server
         ──── Key Share (for forward secrecy)
  Server ──── Validates PSK
         ──── Derives keys from PSK + new DH
```

---

## Handshake Failures

### Certificate Validation Failure

```
Client                                Server
   |  ClientHello                       |
   |  ────────────────────────────────>|
   |                                    |
   |  <────────────────────────────────|
   |  ServerHello + Certificate         |
   |  (expired / wrong domain)          |
   |                                    |
   |- Validation FAILED                 |
   |                                    |
   |  Alert: certificate_expired        |
   |  ────────────────────────────────>|
   |                                    |
   |  Connection Terminated             |
   X                                    X

Browser shows: "Your connection is not private"
```

### Cipher Suite Mismatch

```
Client                                Server
   |  ClientHello                       |
   |  Ciphers: [A, B, C]               |
   |  ────────────────────────────────>|
   |                                    |
   |                                    |- Server only supports
   |                                    |  ciphers [X, Y, Z]
   |                                    |
   |  Alert: handshake_failure          |
   |  <────────────────────────────────|
   |                                    |
   X  Connection Failed                 X

Error: "ERR_SSL_VERSION_OR_CIPHER_MISMATCH"
```

---

## Best Practices

### Recommended (DO)

1. **Use TLS 1.3** for all new deployments
2. **Enable ECDHE** cipher suites for forward secrecy
3. **Disable TLS 1.0/1.1** completely
4. **Enable OCSP Stapling** for performance and privacy
5. **Use session tickets** for resumption
6. **Configure HSTS** header
7. **Monitor certificate expiry** with automated renewal

### Avoid (DON'T)

1. **Don't use SSL 3.0** (vulnerable to POODLE)
2. **Don't use RC4** cipher (cryptographically broken)
3. **Don't use MD5** (collision attacks possible)
4. **Don't use RSA key exchange** (no forward secrecy)
5. **Don't enable TLS compression** (vulnerable to CRIME)
6. **Don't ignore certificate warnings**

---

## Quick Debug Commands

```bash
# Test TLS 1.3 handshake
openssl s_client -connect example.com:443 -tls1_3

# Verbose output with certificates
openssl s_client -connect example.com:443 -showcerts -debug

# Test specific cipher suite
openssl s_client -connect example.com:443 \
  -cipher 'ECDHE-RSA-AES256-GCM-SHA384'

# Check supported protocol versions
for version in ssl3 tls1 tls1_1 tls1_2 tls1_3; do
  echo -n "$version: "
  timeout 1 openssl s_client -connect example.com:443 \
    -$version 2>/dev/null && echo "supported" || echo "not supported"
done

# View certificate details
openssl s_client -connect example.com:443 2>/dev/null | \
  openssl x509 -noout -text

# Check cipher negotiation
openssl s_client -connect example.com:443 2>/dev/null | \
  grep -E "Protocol|Cipher"
```

---

## Visual Memory Aid

```
TLS 1.2  = 2 round trips  = Peace sign
TLS 1.3  = 1 round trip   = One finger
TLS 1.3  = 0 round trips  = OK hand (for returning visitors)
   0-RTT
```

---

## TLS Mathematics

### Step 1: Key Exchange (Raw Secret Material)

Before generating cryptographic keys, client and server calculate a shared secret over an insecure channel using Ephemeral Diffie-Hellman.

#### Traditional DHE (Finite Field)

Using a multiplicative cyclic group of prime p with generator g:

```
Client picks random private integer a:
  A = g^a mod p

Server picks random private integer b:
  B = g^b mod p

Shared secret Z:
  Client: Z = B^a mod p = (g^b)^a = g^(ba) mod p
  Server: Z = A^b mod p = (g^a)^b = g^(ab) mod p
```

#### ECDHE (Elliptic Curve)

Using a standardized curve (X25519 or secp256r1) with base point G:

```
Client selects private key d_C (scalar):
  Q_C = d_C * G

Server selects private key d_S:
  Q_S = d_S * G

Shared point P:
  P = d_C * Q_S = d_C * (d_S * G) = (d_C * d_S) * G

The x-coordinate of P becomes the shared secret Z.
```

### Step 2: HKDF Key Schedule

The raw shared secret Z is processed through HKDF (RFC 5869) to generate cryptographically secure keys.

#### HKDF Components

**HMAC Definition:**

```
HMAC(K, M) = H((K XOR opad) || H((K XOR ipad) || M))
```

Where H is the hash function, || is concatenation, and ipad/opad are padding constants.

**HKDF-Extract:**

```
PRK = HMAC(Salt, IKM)
```

Extracts a pseudorandom key (PRK) from Input Keying Material (IKM).

**HKDF-Expand:**

```
T(1) = HMAC(PRK, Info || 0x01)
T(i) = HMAC(PRK, T(i-1) || Info || counter)
OKM  = First L bytes of (T(1) || T(2) || ...)
```

Expands PRK into output keying material (OKM) of desired length L.

**Derive-Secret (TLS 1.3 specific):**

```
Derive-Secret(Secret, Label, Messages) = 
  HKDF-Expand(Secret, Context, HashLength)

Context = Length(Label) || "tls13 " || Label || 
          Length(Hash) || Hash(Messages)
```

#### TLS 1.3 Key Hierarchy

```
0 (All-zero vector)
|
v
IKM (PSK) -> HKDF-Extract = Early Secret
|
+---> Derive-Secret(..., "external psk binder", "")
|
v
IKM (ECDHE) -> HKDF-Extract = Handshake Secret
|
+---> Derive-Secret(..., "c hs traffic", Transcript_Hash) -> Client Handshake Key
|
+---> Derive-Secret(..., "s hs traffic", Transcript_Hash) -> Server Handshake Key
|
v
IKM (0-vector) -> HKDF-Extract = Master Secret
|
+---> Derive-Secret(..., "c ap traffic", Transcript_Hash) -> Client Application Key
|
+---> Derive-Secret(..., "s ap traffic", Transcript_Hash) -> Server Application Key
|
+---> Derive-Secret(..., "resumption master", Transcript_Hash) -> PSK Resumption
```

### Why This Mathematics Matters

1. **Cryptographic Isolation:** Compromising one key (e.g., Client Handshake Key) doesn't reveal others due to one-way hash properties.

2. **Transcript Binding:** The running hash of all handshake messages (Transcript_Hash) ensures no tampering with unencrypted negotiation messages.

3. **Forward Secrecy:** Ephemeral keys are discarded after use, protecting past sessions even if long-term keys are compromised.

---

_Last Updated: June 2026_
