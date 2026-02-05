# Encryption in System Design

## Table of Contents
1. [Fundamentals](#fundamentals)
2. [Symmetric Encryption](#symmetric-encryption)
3. [Asymmetric Encryption](#asymmetric-encryption)
4. [Key Exchange Mechanisms](#key-exchange-mechanisms)
5. [Hybrid Encryption](#hybrid-encryption)
6. [Use Cases & Best Practices](#use-cases--best-practices)
7. [Comparison Tables](#comparison-tables)

---

## Fundamentals

### What is Encryption?
Encryption is the process of converting plaintext data into ciphertext (encrypted data) using an algorithm and a key, making it unreadable to unauthorized parties. Only those with the correct decryption key can convert it back to plaintext.

### Types of Encryption
1. **Symmetric Encryption**: Same key for encryption and decryption
2. **Asymmetric Encryption**: Different keys for encryption (public key) and decryption (private key)

### Key Concepts
- **Plaintext**: Original, readable data
- **Ciphertext**: Encrypted, unreadable data
- **Key**: Secret value used in encryption/decryption
- **Algorithm/Cipher**: Mathematical procedure for encryption
- **Key Size**: Length of the key in bits (larger = more secure)
- **Block Cipher**: Encrypts fixed-size blocks of data
- **Stream Cipher**: Encrypts data bit-by-bit or byte-by-byte

---

## Symmetric Encryption

Symmetric encryption uses the **same key** for both encryption and decryption. It's fast and efficient for large amounts of data.

### AES (Advanced Encryption Standard)

#### Overview
- **Type**: Block cipher (symmetric)
- **Block Size**: 128 bits
- **Key Sizes**: 128, 192, or 256 bits
- **Standard**: Adopted by NIST in 2001
- **Algorithm**: Based on Rijndael cipher

#### How AES Works
```
1. Key Expansion: Generate round keys from the cipher key
2. Initial Round: AddRoundKey
3. Main Rounds (9, 11, or 13 depending on key size):
   - SubBytes: Non-linear substitution
   - ShiftRows: Transposition
   - MixColumns: Mixing operation
   - AddRoundKey: XOR with round key
4. Final Round: SubBytes, ShiftRows, AddRoundKey (no MixColumns)
```

#### AES Modes of Operation
- **ECB (Electronic Codebook)**: ❌ Not recommended - same plaintext = same ciphertext
- **CBC (Cipher Block Chaining)**: ✅ Each block depends on previous block
- **CTR (Counter)**: ✅ Turns block cipher into stream cipher
- **GCM (Galois/Counter Mode)**: ✅✅ Provides both encryption and authentication
- **CCM (Counter with CBC-MAC)**: ✅ Similar to GCM

#### AES-GCM (Recommended)
```
Features:
- Authenticated encryption with associated data (AEAD)
- Provides confidentiality AND integrity
- Parallel processing capable
- Used in TLS 1.2 and 1.3

Components:
- Encryption: AES in CTR mode
- Authentication: GMAC (Galois Message Authentication Code)
- Nonce/IV: 96 bits (must be unique per encryption)
- Authentication Tag: 128 bits
```

#### Use Cases
- File encryption (disk encryption, archives)
- Database encryption
- VPN protocols (IPsec)
- TLS/SSL connections
- Wireless security (WPA2, WPA3)

#### Example
```python
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
import os

# AES-GCM encryption
key = os.urandom(32)  # 256-bit key
nonce = os.urandom(12)  # 96-bit nonce
plaintext = b"Secret message"

cipher = Cipher(
    algorithms.AES(key),
    modes.GCM(nonce),
    backend=default_backend()
)
encryptor = cipher.encryptor()
ciphertext = encryptor.update(plaintext) + encryptor.finalize()
tag = encryptor.tag

# Decryption
decryptor = cipher.decryptor()
decrypted = decryptor.update(ciphertext) + decryptor.finalize()
```

---

### ChaCha20-Poly1305

#### Overview
- **Type**: Stream cipher with authentication (AEAD)
- **Designer**: Daniel J. Bernstein
- **Key Size**: 256 bits
- **Nonce Size**: 96 bits
- **Authentication Tag**: 128 bits (Poly1305)

#### How ChaCha20-Poly1305 Works
```
ChaCha20: Stream cipher component
- Uses 256-bit key and 96-bit nonce
- Generates pseudorandom keystream
- XOR keystream with plaintext

Poly1305: MAC component
- One-time authenticator
- Provides message integrity
- Uses part of ChaCha20 keystream for MAC key

Combined: Authenticated Encryption
1. Encrypt plaintext with ChaCha20
2. Generate MAC tag with Poly1305
3. Output: ciphertext + authentication tag
```

#### Advantages over AES
```
✅ Software Performance: 
   - Faster than AES on devices without AES-NI
   - Better on ARM processors, mobile devices
   
✅ Constant-Time:
   - Resistant to timing attacks by design
   - No table lookups
   
✅ Simpler Implementation:
   - Easier to implement correctly
   - Fewer opportunities for bugs

⚠️  Hardware Performance:
   - Slower than AES on CPUs with AES-NI
```

#### Use Cases
- TLS 1.3 (preferred cipher suite)
- Mobile applications
- IoT devices
- VPN (WireGuard uses ChaCha20-Poly1305)
- SSH connections
- Signal Protocol

#### Example
```python
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
import os

key = ChaCha20Poly1305.generate_key()
chacha = ChaCha20Poly1305(key)
nonce = os.urandom(12)

plaintext = b"Secret message"
associated_data = b"metadata"  # Optional, authenticated but not encrypted

# Encryption
ciphertext = chacha.encrypt(nonce, plaintext, associated_data)

# Decryption
decrypted = chacha.decrypt(nonce, ciphertext, associated_data)
```

---

## Asymmetric Encryption

Asymmetric encryption uses a **pair of keys**: a public key for encryption and a private key for decryption.

### RSA (Rivest-Shamir-Adleman)

#### Overview
- **Type**: Asymmetric encryption
- **Key Sizes**: 2048, 3072, 4096 bits (2048 minimum recommended)
- **Invented**: 1977
- **Security**: Based on difficulty of factoring large prime numbers

#### How RSA Works
```
Key Generation:
1. Choose two large prime numbers p and q
2. Calculate n = p × q (modulus)
3. Calculate φ(n) = (p-1)(q-1)
4. Choose public exponent e (commonly 65537)
5. Calculate private exponent d: d × e ≡ 1 (mod φ(n))
6. Public key: (n, e)
7. Private key: (n, d)

Encryption:
ciphertext = plaintext^e mod n

Decryption:
plaintext = ciphertext^d mod n
```

#### RSA Operations

**1. Encryption/Decryption**
```
Public Key Encryption:
- Anyone can encrypt with public key
- Only private key holder can decrypt

Private Key Signing:
- Private key holder creates signature
- Anyone can verify with public key
```

**2. Digital Signatures**
```
Signing:
1. Hash the message
2. Encrypt hash with private key
3. Signature = encrypted hash

Verification:
1. Decrypt signature with public key
2. Hash the message independently
3. Compare decrypted hash with computed hash
```

#### RSA Padding Schemes
```
⛔ PKCS#1 v1.5: Older, vulnerable to attacks

✅ OAEP (Optimal Asymmetric Encryption Padding):
   - More secure
   - Randomized padding
   - Used for encryption

✅ PSS (Probabilistic Signature Scheme):
   - For digital signatures
   - Provably secure
```

#### Limitations
```
❌ Slow: Much slower than symmetric encryption
❌ Size Limits: Can only encrypt small amounts of data
   - For RSA-2048: max ~245 bytes with OAEP
❌ Key Size: Requires larger keys (2048+ bits)
```

#### Use Cases
- Key exchange (establishing symmetric keys)
- Digital signatures
- Certificate authorities (SSL/TLS certificates)
- SSH authentication
- Email encryption (PGP/GPG)
- Code signing

#### Example
```python
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes

# Generate RSA key pair
private_key = rsa.generate_private_key(
    public_exponent=65537,
    key_size=2048
)
public_key = private_key.public_key()

# Encryption (small data only)
plaintext = b"Secret message"
ciphertext = public_key.encrypt(
    plaintext,
    padding.OAEP(
        mgf=padding.MGF1(algorithm=hashes.SHA256()),
        algorithm=hashes.SHA256(),
        label=None
    )
)

# Decryption
decrypted = private_key.decrypt(
    ciphertext,
    padding.OAEP(
        mgf=padding.MGF1(algorithm=hashes.SHA256()),
        algorithm=hashes.SHA256(),
        label=None
    )
)

# Digital Signature
signature = private_key.sign(
    plaintext,
    padding.PSS(
        mgf=padding.MGF1(hashes.SHA256()),
        salt_length=padding.PSS.MAX_LENGTH
    ),
    hashes.SHA256()
)

# Verify Signature
public_key.verify(
    signature,
    plaintext,
    padding.PSS(
        mgf=padding.MGF1(hashes.SHA256()),
        salt_length=padding.PSS.MAX_LENGTH
    ),
    hashes.SHA256()
)
```

---

## Key Exchange Mechanisms

Key exchange protocols allow two parties to establish a shared secret key over an insecure channel.

### RSA Key Exchange (Legacy)

#### How It Works
```
1. Server sends public key to client
2. Client generates random pre-master secret
3. Client encrypts pre-master secret with server's public key
4. Client sends encrypted pre-master secret to server
5. Server decrypts with private key
6. Both derive session keys from pre-master secret
```

#### Drawbacks
```
❌ No Forward Secrecy:
   - If private key compromised, all past sessions can be decrypted
   - Recorded traffic can be decrypted later

❌ Passive Vulnerability:
   - Attacker can record traffic and decrypt later

Status: Deprecated in TLS 1.3
```

---

### DHE (Diffie-Hellman Ephemeral)

#### Overview
- **Type**: Key exchange protocol
- **Variant**: Uses ephemeral (temporary) keys
- **Forward Secrecy**: ✅ Yes
- **Based On**: Discrete logarithm problem

#### How DHE Works
```
Setup:
- Agree on prime number p and generator g (public)

Client:
1. Generate random private key: a
2. Calculate public key: A = g^a mod p
3. Send A to server

Server:
1. Generate random private key: b
2. Calculate public key: B = g^b mod p
3. Send B to client

Both Calculate Shared Secret:
Client: s = B^a mod p
Server: s = A^b mod p
Result: Both have same shared secret s = g^(ab) mod p
```

#### Advantages
```
✅ Forward Secrecy:
   - New keys for each session
   - Past sessions safe even if long-term key compromised

✅ No Key Transmission:
   - Shared secret never transmitted
   - Resistant to passive eavesdropping
```

#### Disadvantages
```
❌ Vulnerable to Man-in-the-Middle:
   - Needs authentication (certificates/signatures)
   
❌ Computational Cost:
   - More expensive than RSA key exchange
   
❌ Larger Keys:
   - 2048-3072 bits recommended
```

---

### ECDHE (Elliptic Curve Diffie-Hellman Ephemeral)

#### Overview
- **Type**: Key exchange protocol using elliptic curves
- **Forward Secrecy**: ✅ Yes
- **Key Size**: 256 bits ≈ 3072-bit RSA security
- **Standard**: Preferred in TLS 1.2 and mandatory in TLS 1.3

#### How ECDHE Works
```
Setup:
- Agree on elliptic curve parameters (public)
- Common curves: P-256, P-384, Curve25519, Curve448

Client:
1. Generate random private key: a
2. Calculate public key: A = a × G (point on curve)
3. Send A to server

Server:
1. Generate random private key: b
2. Calculate public key: B = b × G
3. Send B to client

Both Calculate Shared Secret:
Client: S = a × B
Server: S = b × A
Result: Both have same point S = (ab) × G
```

#### Advantages Over DHE
```
✅ Smaller Keys:
   - 256-bit ECDHE ≈ 3072-bit DHE security
   - Less bandwidth, faster computation

✅ Better Performance:
   - Faster key generation
   - Faster shared secret computation

✅ Forward Secrecy:
   - Same as DHE

✅ Modern Standard:
   - Required in TLS 1.3
   - Supported by all modern systems
```

#### Common Curves
```
NIST Curves:
- P-256 (secp256r1): Most common, good performance
- P-384 (secp384r1): Higher security
- P-521 (secp521r1): Maximum security

Modern Curves:
- Curve25519 (X25519): Fast, secure, simple
- Curve448 (X448): Higher security than X25519

Recommendation: X25519 or P-256
```

#### Use Cases
- TLS 1.2 and 1.3 (primary key exchange)
- SSH key exchange
- VPN protocols
- Signal Protocol
- WhatsApp encryption

#### Example (Conceptual)
```python
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF

# Server side
server_private_key = ec.generate_private_key(ec.SECP256R1())
server_public_key = server_private_key.public_key()

# Client side
client_private_key = ec.generate_private_key(ec.SECP256R1())
client_public_key = client_private_key.public_key()

# Both parties exchange public keys, then:
# Server computes shared secret
server_shared_secret = server_private_key.exchange(
    ec.ECDH(), 
    client_public_key
)

# Client computes shared secret
client_shared_secret = client_private_key.exchange(
    ec.ECDH(), 
    server_public_key
)

# Both secrets are identical!
# Derive encryption key from shared secret
shared_key = HKDF(
    algorithm=hashes.SHA256(),
    length=32,
    salt=None,
    info=b'handshake data',
).derive(server_shared_secret)
```

---

## Hybrid Encryption

Real-world systems combine symmetric and asymmetric encryption for optimal security and performance.

### How Hybrid Encryption Works

```
Step 1: Key Exchange (Asymmetric)
- Use RSA or ECDHE to establish shared secret
- Authenticate with digital signatures/certificates
- Provides forward secrecy (with ECDHE)

Step 2: Session Key Derivation
- Derive symmetric key from shared secret
- Use KDF (Key Derivation Function)

Step 3: Data Encryption (Symmetric)
- Use AES-GCM or ChaCha20-Poly1305
- Encrypt bulk data with session key
- Fast and efficient

Step 4: Authentication
- AEAD (AES-GCM/ChaCha20-Poly1305) provides integrity
- Or use separate MAC (HMAC-SHA256)
```

### TLS 1.3 Example Flow
```
Client Hello:
- Supported cipher suites
- ECDHE key share (public key)

Server Hello:
- Selected cipher suite: TLS_AES_256_GCM_SHA384
- ECDHE key share (public key)
- Server certificate (RSA or ECDSA signature)

[Both derive shared secret via ECDHE]
[Both derive session keys from shared secret]

Application Data:
- Encrypted with AES-256-GCM
- Uses derived session keys
- Each direction has unique keys
```

### Why Hybrid?
```
Asymmetric (RSA/ECDHE):
✅ Solves key distribution problem
✅ Provides authentication
❌ Slow for large data

Symmetric (AES/ChaCha20):
✅ Fast encryption
✅ Efficient for bulk data
❌ Key distribution problem

Hybrid = Best of Both Worlds
```

---

## Use Cases & Best Practices

### When to Use Each Algorithm

#### AES-256-GCM
```
✅ Use When:
- Hardware has AES-NI support
- Maximum compatibility needed
- Government/compliance requirements (FIPS 140-2)
- Database encryption
- File system encryption

Use Cases:
- Banking applications
- Healthcare data (HIPAA)
- Government systems
- Enterprise disk encryption
```

#### ChaCha20-Poly1305
```
✅ Use When:
- Software-only implementation
- Mobile/IoT devices
- ARM processors
- Performance critical on non-AES-NI hardware
- Need constant-time operation

Use Cases:
- Mobile apps (Android, iOS)
- VPN (WireGuard)
- TLS on mobile devices
- Embedded systems
```

#### RSA
```
✅ Use When:
- Digital signatures needed
- Certificate-based authentication
- Email encryption (PGP/GPG)
- Legacy system compatibility

⚠️  Do NOT Use For:
- Bulk data encryption (use AES instead)
- Key exchange (use ECDHE instead)
```

#### ECDHE
```
✅ Use When:
- Establishing secure connections
- Need forward secrecy
- Modern protocol implementation
- TLS/SSL connections

Always Use:
- In TLS 1.3 (mandatory)
- For new implementations
```

---

### Best Practices

#### Key Management
```
1. Key Generation:
   ✅ Use cryptographically secure random number generator
   ✅ Proper entropy source
   ❌ Never use weak PRNGs (Math.random(), etc.)

2. Key Storage:
   ✅ Use hardware security modules (HSM)
   ✅ Encrypt keys at rest
   ✅ Use key management services (AWS KMS, Azure Key Vault)
   ❌ Never hardcode keys in source code
   ❌ Never store keys in version control

3. Key Rotation:
   ✅ Regular rotation schedule
   ✅ Automate rotation process
   ✅ Support multiple active keys during transition

4. Key Derivation:
   ✅ Use HKDF, PBKDF2, or Argon2
   ✅ Proper salt and iteration count
```

#### Algorithm Selection
```
2024+ Recommendations:

Symmetric Encryption:
✅ First choice: AES-256-GCM (with AES-NI)
✅ Alternative: ChaCha20-Poly1305 (without AES-NI)
❌ Avoid: AES-CBC without HMAC, RC4, DES, 3DES

Asymmetric Encryption:
✅ Signatures: RSA-PSS (3072+ bits) or Ed25519
✅ Encryption: RSA-OAEP (3072+ bits) (only for key exchange)
❌ Avoid: RSA < 2048 bits, DSA

Key Exchange:
✅ First choice: ECDHE with X25519 or P-256
✅ Alternative: DHE with 3072+ bits
❌ Avoid: Static RSA, DHE < 2048 bits

Hashing:
✅ SHA-256, SHA-384, SHA-512, SHA-3, BLAKE2
❌ Avoid: MD5, SHA-1
```

#### Implementation Guidelines
```
1. Use Standard Libraries:
   ✅ OpenSSL, LibSodium, Bouncy Castle
   ✅ Language crypto libraries (cryptography.io, Go crypto)
   ❌ Never roll your own crypto

2. Authenticated Encryption:
   ✅ Always use AEAD (AES-GCM, ChaCha20-Poly1305)
   ✅ Or Encrypt-then-MAC with separate HMAC
   ❌ Never use encryption without authentication

3. Nonce/IV Management:
   ✅ Always unique per encryption operation
   ✅ Random for CBC/CTR, counter for GCM
   ❌ Never reuse nonces with same key

4. Padding Oracle Protection:
   ✅ Use AEAD modes (GCM, ChaCha20-Poly1305)
   ✅ Constant-time comparison for MACs
   ❌ Avoid CBC mode if possible

5. Timing Attack Prevention:
   ✅ Constant-time operations
   ✅ Use timing-safe comparison functions
   ❌ Don't branch on secret data
```

#### Common Pitfalls
```
❌ ECB Mode:
   - Same plaintext blocks = same ciphertext blocks
   - Reveals patterns in data
   
❌ Weak Keys:
   - Short keys
   - Predictable keys
   - Derived from passwords without KDF
   
❌ IV/Nonce Reuse:
   - Catastrophic for CTR mode
   - Breaks authentication in GCM
   
❌ No Authentication:
   - Allows tampering
   - Subject to bit-flipping attacks
   
❌ Hardcoded Secrets:
   - Keys in source code
   - Secrets in version control
```

---

## Comparison Tables

### Symmetric Encryption Comparison

| Feature | AES-256-GCM | ChaCha20-Poly1305 |
|---------|-------------|-------------------|
| **Type** | Block cipher (AEAD) | Stream cipher (AEAD) |
| **Key Size** | 256 bits | 256 bits |
| **Nonce Size** | 96 bits | 96 bits |
| **Block Size** | 128 bits | N/A (stream) |
| **Authentication** | GMAC (128-bit tag) | Poly1305 (128-bit tag) |
| **Speed (AES-NI)** | ⚡⚡⚡ Very Fast | ⚡⚡ Fast |
| **Speed (Software)** | ⚡⚡ Fast | ⚡⚡⚡ Very Fast |
| **Security** | ✅ Excellent | ✅ Excellent |
| **Standardization** | NIST, FIPS | RFC 8439 |
| **Timing Attacks** | Resistant (with AES-NI) | Resistant (by design) |
| **Mobile Performance** | Good (with hardware) | ⚡⚡⚡ Excellent |
| **Use in TLS 1.3** | ✅ Yes | ✅ Yes (preferred) |

### Key Exchange Comparison

| Feature | RSA | DHE | ECDHE |
|---------|-----|-----|-------|
| **Type** | Asymmetric encryption | Key agreement | Key agreement (ECC) |
| **Forward Secrecy** | ❌ No | ✅ Yes | ✅ Yes |
| **Key Size** | 2048-4096 bits | 2048-3072 bits | 256-384 bits |
| **Performance** | ⚡ Slow | ⚡⚡ Moderate | ⚡⚡⚡ Fast |
| **Bandwidth** | High | High | Low |
| **Security Level (256-bit)** | N/A | ~3072-bit equiv | 256-bit key |
| **TLS 1.2** | ✅ Supported | ✅ Supported | ✅ Supported |
| **TLS 1.3** | ❌ Removed | ❌ Removed | ✅ Required |
| **MITM Resistant** | Only with auth | Only with auth | Only with auth |
| **Recommendation** | ⛔ Avoid for key exchange | ✅ OK (legacy) | ✅✅ Preferred |

### Algorithm Security Levels

| Algorithm | Key Size | Security Level | Quantum Safe? |
|-----------|----------|----------------|---------------|
| **AES** | 128 bits | ~128-bit | Partial (64-bit) |
| **AES** | 256 bits | ~256-bit | Partial (128-bit) |
| **ChaCha20** | 256 bits | ~256-bit | Partial (128-bit) |
| **RSA** | 2048 bits | ~112-bit | ❌ No |
| **RSA** | 3072 bits | ~128-bit | ❌ No |
| **RSA** | 4096 bits | ~140-bit | ❌ No |
| **ECDHE (P-256)** | 256 bits | ~128-bit | ❌ No |
| **ECDHE (P-384)** | 384 bits | ~192-bit | ❌ No |
| **X25519** | 256 bits | ~128-bit | ❌ No |

### Real-World Protocol Usage

| Protocol | Encryption | Key Exchange | Authentication |
|----------|-----------|--------------|----------------|
| **TLS 1.3** | AES-GCM, ChaCha20-Poly1305 | ECDHE | RSA, ECDSA, Ed25519 |
| **TLS 1.2** | AES-GCM, AES-CBC | RSA, DHE, ECDHE | RSA, ECDSA |
| **SSH** | AES-GCM, ChaCha20-Poly1305 | ECDH, DH | RSA, Ed25519 |
| **WireGuard** | ChaCha20-Poly1305 | X25519 | Curve25519 |
| **Signal** | AES-CBC, ChaCha20-Poly1305 | X25519 | Ed25519 |
| **WhatsApp** | AES-CBC | ECDHE | ECDSA |
| **IPsec** | AES-GCM, ChaCha20-Poly1305 | ECDHE, DHE | RSA, ECDSA |

---

## Summary

### Quick Reference

**For New Projects:**
```
✅ Encryption: AES-256-GCM or ChaCha20-Poly1305
✅ Key Exchange: ECDHE with X25519 or P-256
✅ Signatures: Ed25519 or RSA-PSS (3072+ bits)
✅ Hashing: SHA-256 or BLAKE2
```

**Key Takeaways:**
1. **Symmetric** (AES, ChaCha20) for bulk data encryption
2. **Asymmetric** (RSA) for key exchange and digital signatures (being phased out for key exchange)
3. **ECDHE** is the modern standard for key exchange
4. **Always use authenticated encryption** (AEAD)
5. **Forward secrecy** is essential (use ephemeral keys)
6. **Never implement crypto yourself** - use trusted libraries
7. **Key management** is as important as the algorithms

### Migration Path

```
Legacy (Avoid):
- AES-CBC without HMAC → AES-GCM
- RC4, 3DES → AES or ChaCha20
- RSA key exchange → ECDHE
- MD5, SHA-1 → SHA-256+
- Static keys → Ephemeral keys

Modern (Current):
- AES-256-GCM
- ChaCha20-Poly1305
- ECDHE (X25519, P-256)
- RSA-3072+ (signatures only)
- SHA-256, SHA-384

Future (Post-Quantum):
- CRYSTALS-Kyber (key exchange)
- CRYSTALS-Dilithium (signatures)
- SPHINCS+ (signatures)
- AES-256 (still secure with Grover's algorithm)
```

---

## Additional Resources

### Standards & Specifications
- RFC 8439: ChaCha20 and Poly1305
- RFC 8446: TLS 1.3
- FIPS 197: AES Standard
- NIST SP 800-175B: Key Management Guidelines

### Recommended Libraries
- **Python**: cryptography, PyNaCl
- **JavaScript/Node.js**: crypto (built-in), libsodium.js
- **Go**: crypto package
- **Rust**: ring, rust-crypto
- **Java**: Bouncy Castle
- **C/C++**: OpenSSL, libsodium

### Testing & Validation
- SSL Labs (ssllabs.com/ssltest/)
- CryptCheck (cryptcheck.fr)
- Qualys SSL Server Test
- testssl.sh

---

*Last Updated: January 2026*
