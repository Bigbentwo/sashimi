# `sashimi` 刺身

# A primitive tool for encryption.

## Properties it should have
- Encrypted data indistinguishable from random noise
- Salted keys
- (Multiple files) optional for now
- (Nested containers for hidden containers) also optional

## Prototype idea (old)
0. Generate entropy from ChaCha20Rng and getrandom
1. Custom memory intensive hash function based on SHA3
2. Hash for key (256 + 256) (salted)
3. All-or-nothing transform
4. AES in GCM ?
5. (All-or-nothing transform)
6. Keccak based stream cipher
7. Store result as zip

Comment on current implementation:
- Approach to stream cipher is broken