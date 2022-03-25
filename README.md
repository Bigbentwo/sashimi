# `sashimi` 刺身

# A primitive tool for encryption.

## Prototype idea
0. Generate entropy from ChaCha20Rng and getrandom
1. Custom memory intensive hash function based on SHA3
2. Hash for key (256 + 256) (salted)
3. All-or-nothing transform
4. AES in GCM ?
5. (All-or-nothing transform)
6. Keccak based stream cipher
7. Store result as zip

## Custom `sashimi-hash` function
Use SHA3-512 of `input` to fill a buffer of a give size `c_cost` (probably in
the range of 500 MB). Mix contents of the buffer. 

```
   ┌───────────────────┐┌────────────┐┌────────┐┌───────────────┐      
   │     Plaintext     ││    Key     ││ Config ││    Entropy    │──┐   
   └───────────────────┘└────────────┘└────────┘└───────────────┘  │   
             ┃                 │                                   │   
             ┃              ┌──┘            ╔═════════════════╗    │   
             ┃              │               ║   ChaCha20Rng   ║◀───┘   
             ┃              ▼               ╚═════════════════╝        
             ┃     ╔═════════════════╗               │                 
             ┃     ║  sashimi-hash   ║◀────────┐     └─────────┐       
             ┃     ╚═════════════════╝         │               ▼       
             ┃              │                  │          ┌────────┐   
             ┃              ▼                  └──────────│  Salt  │──┐
             ┃     ┌─────────────────┐                    └────────┘  │
             ┃     │   Salted key    │─────────────────┐              │
             ┃     └─────────────────┘                 ▼              │
             ┃              │                 ╔═════════════════╗     │
             ┃              ▼                 ║  sashimi-hash   ║     │
             ┃     ╔═════════════════╗        ║  stream cipher  ║     │
             ┣────▶║  sashimi-hash   ║        ╚═════════════════╝     │
             ┃     ╚═════════════════╝                 │              │
             ┃              │                          │              │
             ▼              ▼                          ▼              │
   ┌───────────────────┬─────────┐           ┌──────────────────┐     │
   │     Plaintext     │   MAC   │━━━━┳──────│     Bitmask      │     │
   └───────────────────┴─────────┘    ┃      └──────────────────┘     │
                                      ┃                               │
                                      ┃                               │
                                      ▼                               │
                                  ╔═══════╗                           │
                                  ║  Xor  ║                           │
                                  ╚═══════╝                           │
                                      ┃                               │
                        ┏━━━━━━━━━━━━━┛                               │
                        ┃                                             │
                        ▼                                             │
            ┌───────────────────────┬────────┬───────┐                │
            │      Ciphertext       │ Config │ Salt  │◀───────────────┘
            └───────────────────────┴────────┴───────┘                 
```