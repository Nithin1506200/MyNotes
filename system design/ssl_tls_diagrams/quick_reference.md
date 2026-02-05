# SSL/TLS Quick Reference Card

**🎯 Perfect for Last-Minute Interview Prep**

## TLS 1.2 vs TLS 1.3: Key Differences

TLS 1.3 is a significant upgrade over TLS 1.2, offering faster performance (1-RTT handshake, optional 0-RTT) and stronger security by removing outdated/vulnerable algorithms (like RC4, MD5) and mandating Perfect Forward Secrecy (PFS). Key differences include TLS 1.3's streamlined, encrypted handshake, simpler cipher suites (only AEAD), and reliance on ephemeral key exchanges (ECDHE), while TLS 1.2's complex, two-round-trip handshake and support for weaker ciphers created vulnerabilities.

### Performance & Handshake

- **TLS 1.2:** Two round trips (2-RTT) for handshake, adding latency
- **TLS 1.3:** One round trip (1-RTT), with 0-RTT for faster resumption on repeat visits, significantly reducing setup time

**📹 Video Resource:** [TLS 1.2 and TLS 1.3 handshakes efficiency](https://www.youtube.com/watch?v=...) (58s) - Cyrill Gössi

### Security & Ciphers

- **TLS 1.2:** Supports many cipher suites, including older, weaker ones (RC4, CBC modes) and optional PFS
- **TLS 1.3:** Only allows strong AEAD ciphers (like AES-GCM, ChaCha20-Poly1305) and requires PFS, eliminating known vulnerabilities

**📹 Video Resource:** [Changes in TLS 1.3 cipher suites](https://www.youtube.com/watch?v=...) (1m) - Practical Networking

### Key Exchange & Features

- **TLS 1.2:** Supports RSA, DHE, ECDHE; key exchange details are exposed during handshake
- **TLS 1.3:** Mandates ephemeral Diffie-Hellman (ECDHE) for PFS and encrypts more handshake messages, including certificate validation, preventing eavesdropping

### Vulnerabilities & Legacy

- **TLS 1.2:** Susceptible to attacks like BEAST, POODLE, Heartbleed due to legacy algorithms and design
- **TLS 1.3:** Eliminates these vulnerabilities by dropping support for insecure features and algorithms, though legacy systems might still need TLS 1.2 support during transition

### Summary Table

| Feature          | TLS 1.2                           | TLS 1.3                              |
| ---------------- | --------------------------------- | ------------------------------------ |
| Handshake        | 2-RTT (slower)                    | 1-RTT, 0-RTT option (faster)         |
| Security         | Weaker ciphers, optional PFS      | Strong AEAD ciphers, mandatory PFS   |
| Algorithms       | RC4, MD5, etc. (vulnerable)       | Only modern, secure algorithms       |
| Handshake Details| Encrypted less (e.g., certificate)| Encrypted more (incl. certificate)   |
| Key Exchange     | RSA, DHE, ECDHE                   | ECDHE (ephemeral only)               |
| Key Benefit      | Still widely used, legacy support | Faster, more secure, simpler         |

---

## Core Concepts (30 seconds)

```
┌────────────────────────────────────────────┐
│  SSL/TLS = Secure Communication Protocol   │
├────────────────────────────────────────────┤
│  SSL  → Deprecated (last: SSL 3.0, 1996)  │
│  TLS  → Current (active: 1.2, 1.3)        │
│                                            │
│  Purpose: 🔐 Encryption                    │
│          ✓ Authentication                  │
│          ✅ Integrity                       │
└────────────────────────────────────────────┘
```

---

## Version Timeline ⏱️

```
1995 → SSL 2.0
1996 → SSL 3.0
1999 → TLS 1.0
2006 → TLS 1.1
2008 → TLS 1.2  ← Minimum acceptable
2018 → TLS 1.3  ← Recommended
2021 → TLS 1.0/1.1 deprecated
```

---

## Key Differences: SSL vs TLS

| Aspect  | SSL                  | TLS                            |
| ------- | -------------------- | ------------------------------ |
| Speed   | Slow (2-3 RTT)       | Fast (1-2 RTT, TLS 1.3: 1 RTT) |
| MAC     | MD5 (weak)           | HMAC (strong)                  |
| Alerts  | 2 types, unencrypted | 3 types, encrypted             |
| Ciphers | Vulnerable           | Modern & secure                |
| Status  | ❌ Deprecated        | ✅ Active                      |

---

## TLS Handshake (Memorize This!)

### TLS 1.2 (2 Round Trips)

```
1. Client → Server: ClientHello (ciphers, version)
2. Server → Client: ServerHello (cert, selected cipher)
3. Client → Server: Key Exchange + Finished
4. Server → Client: Finished
5. 🔐 Encrypted Communication
```

### TLS 1.3 (1 Round Trip!) ⚡

```
1. Client → Server: ClientHello + KeyShare
2. Server → Client: ServerHello + Cert + Finished
3. 🔐 Encrypted Communication (0-RTT possible)
```

---

## Certificate Chain (Critical!)

```
Root CA (Browser Trust Store)
    ↓ signs
Intermediate CA
    ↓ signs
Server Certificate (example.com)
```

**Validation Steps:**

1. ✓ Not expired
2. ✓ Domain matches
3. ✓ Chain valid
4. ✓ Not revoked (OCSP/CRL)
5. ✓ Signature valid

---

## Cipher Suite Anatomy

**Example:** `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384`

```
TLS        → Protocol
ECDHE      → Key Exchange (Ephemeral = Forward Secrecy)
RSA        → Authentication
AES_256    → Encryption (Symmetric)
GCM        → Mode of operation
SHA384     → Hash/MAC
```

---

## Port Numbers

- **HTTP:** 80 (unencrypted)
- **HTTPS:** 443 (TLS encrypted)

---

## Must-Know Security Concepts

### 1. **Forward Secrecy (PFS)**

- Session keys are temporary
- Even if server key leaked → past traffic safe
- Use: DHE or ECDHE

### 2. **Certificate Revocation**

| Method        | Speed | Privacy | Best? |
| ------------- | ----- | ------- | ----- |
| CRL           | Slow  | Poor    | ❌    |
| OCSP          | Fast  | Poor    | ⚠️    |
| OCSP Stapling | Fast  | Good    | ✅    |

### 3. **Asymmetric vs Symmetric**

```
Handshake → Asymmetric (RSA/ECDH) → Slow but secure
Data      → Symmetric (AES)        → Fast encryption
```

---

## Common Interview Questions (Quick Answers)

**Q: Why not use SSL?**

- Deprecated, has vulnerabilities (POODLE, BEAST)

**Q: What's encrypted in HTTPS?**

- HTTP headers, body, cookies
- NOT: DNS queries, IP addresses, SNI (unless eSNI)

**Q: How does browser verify certificate?**

- Checks: expiration, domain, chain, revocation, signature

**Q: TLS 1.3 benefits?**

- 50% faster (1-RTT), mandatory PFS, encrypted handshake

**Q: What if handshake fails?**

- Browser shows warning ("Not Secure")
- Connection terminated
- Common causes: expired cert, hostname mismatch, untrusted CA

---

## Quick Commands

```bash
# View certificate
openssl s_client -connect example.com:443 -showcerts

# Check expiration
echo | openssl s_client -connect example.com:443 2>/dev/null | \
  openssl x509 -noout -dates

# Test TLS version
openssl s_client -connect example.com:443 -tls1_3

# Generate CSR
openssl req -new -newkey rsa:2048 -nodes \
  -keyout server.key -out server.csr
```

---

## Configuration Checklist

**Nginx Example:**

```nginx
ssl_protocols TLSv1.2 TLSv1.3;
ssl_ciphers 'ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384';
ssl_prefer_server_ciphers on;
ssl_stapling on;
add_header Strict-Transport-Security "max-age=31536000";
```

**Security Headers:**

```
✓ HSTS: Strict-Transport-Security
✓ X-Frame-Options: SAMEORIGIN
✓ X-Content-Type-Options: nosniff
```

---

## Common Attacks & Mitigations

| Attack     | Target               | Mitigation             |
| ---------- | -------------------- | ---------------------- |
| POODLE     | SSL 3.0              | Disable SSL, use TLS   |
| BEAST      | TLS 1.0              | Use TLS 1.2+           |
| Heartbleed | OpenSSL              | Update OpenSSL         |
| MITM       | Any                  | Certificate validation |
| Downgrade  | Protocol negotiation | TLS_FALLBACK_SCSV      |

---

## Memory Aids 🧠

**Three Goals:** **E.A.I.**

- **E**ncryption
- **A**uthentication
- **I**ntegrity

**Handshake Order:** **"Charlie Says Kinda Fun!"**

- **C**lientHello
- **S**erverHello
- **K**ey Exchange
- **F**inished

**Certificate Validation:** **"Every Domain Checks Revocation Signals"**

- **E**xpiration
- **D**omain match
- **C**hain valid
- **R**evocation check
- **S**ignature verify

---

## Things NOT to Say in Interview ❌

❌ "SSL and TLS are the same"
✅ "TLS is SSL's successor"

❌ "Certificates encrypt data"
✅ "Certificates authenticate; session keys encrypt"

❌ "HTTPS is 100% private"
✅ "HTTPS encrypts content, but DNS/IP may leak"

---

## Key Numbers to Remember

- **TLS 1.2:** 2 round trips (RTT)
- **TLS 1.3:** 1 round trip (RTT), 0-RTT resumption
- **Ports:** HTTP=80, HTTPS=443
- **Min Key Size:** RSA 2048-bit, ECDSA 256-bit
- **Deprecation:** TLS 1.0/1.1 (2021), AWS requires 1.2+ (2023)

---

## Visual Mnemonic: TLS Flow

```
   Browser              Server
      |                   |
   🤝 |---- Hello -------->| 🤝
      |<--- Cert ---------| 📜
   🔑 |---- Key --------->| 🔑
   ✓  |<--- OK ----------| ✓
      |                   |
   🔐 |<==== Data =======>| 🔐
```

---

## Test Yourself (5 Questions)

1. **What port does HTTPS use?** `443`
2. **TLS 1.3 uses how many RTT?** `1 (one)`
3. **What does PFS prevent?** `Past traffic decryption if key compromised`
4. **Name 3 things a certificate contains** `Domain, Public Key, Expiration`
5. **What's the top of certificate chain?** `Root CA`

---

## Resources for Deep Dive

- **Test SSL:** https://www.ssllabs.com/ssltest/
- **Config Generator:** https://ssl-config.mozilla.org/
- **RFCs:** TLS 1.2 (RFC 5246), TLS 1.3 (RFC 8446)

---

**💡 Pro Tip:** Draw the handshake diagram during the interview to show you understand the flow visually!

**⏰ Study Time:** Review this card 15 minutes before your interview for maximum retention.

---

_Last Updated: January 2026_
