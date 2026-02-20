# Blockchain Security Integration Notes (Experimental)

These notes describe integration hygiene for Astryx-based hashing experiments.

## Domain separation

Always include context prefixes, e.g.:

- `ASTRYX-TXID-V1 || serialized_tx`
- `ASTRYX-BLOCKID-V1 || serialized_header`
- `ASTRYX-MERKLE-INTERNAL-V1 || left || right`

This avoids cross-context collision surprises.

## Canonical serialization

Consensus systems must ensure all hashed data is canonicalized (field order, byte encoding, varint policy).

## Migration policy

Because Astryx is experimental, deploy behind algorithm version identifiers and maintain a migration path.

## Audit path

Before production consideration:

1. independent cryptanalysis,
2. implementation review,
3. side-channel review,
4. interoperability and deterministic vector verification.

## Important limitation

This repository is not a complete blockchain security stack; it provides hashing experiments only.
