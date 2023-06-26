# ULID (Unique Lexicographically Sortable Identifier)

<https://github.com/ulid/spec>

- ulid contais 48 bit timestamp followed by 80 bits of random data
- timestamp allows it be sortable
- this allows b-tree structured database for indexing
- which inturn increases the speed of INSERT

```text
 01AN4Z07BY      79KA1307SR9X4MV3

|----------|    |----------------|
 Timestamp          Randomness
   48bits             80bits
```

## Components

### Timestamp

- 48 bit integer
- UNIX-time in milliseconds
- Won't run out of space 'til the year 10889 AD.

### Randomness

- 80 bits
- Cryptographically secure source of randomness, if possible

## Canonical String Representation

```text
ttttttttttrrrrrrrrrrrrrrrr
where
t is Timestamp (10 characters)
r is Randomness (16 characters)
```

## Encoding

Crockford's Base32 is used as shown. This alphabet excludes the letters I, L, O, and U to avoid confusion and abuse.

<http://www.crockford.com/base32.html>
The encoding scheme is required to

- Be human readable and machine readable.
- Be compact. Humans have difficulty in manipulating long strings of arbitrary symbols.
- Be error resistant. Entering the symbols must not -require keyboarding gymnastics.
- Be pronounceable. Humans should be able to accurately transmit the symbols to other humans using a telephone.

### Symbols

The Base 32 symbol set is a superset of the Base 16 symbol set.

We chose a symbol set of 10 digits and 22 letters. We exclude 4 of the 26 letters: I L O U.

| ExcludedLetters |                        |
| --------------- | ---------------------- |
| I               | Can be confused with 1 |
| L               | Can be confused with 1 |
| O               | Can be confused with 0 |
| U               | Accidental obscenity   |

## Monotonicity

When generating a ULID within the same millisecond, we can provide some guarantees regarding sort order. Namely, if the same millisecond is detected, the random component is incremented by 1 bit in the least significant bit position (with carrying). For example:

```js
import { monotonicFactory } from "ulid";

const ulid = monotonicFactory();

// Assume that these calls occur within the same millisecond
ulid(); // 01BX5ZZKBKACTAV9WEVGEMMVRZ
ulid(); // 01BX5ZZKBKACTAV9WEVGEMMVS0
```
