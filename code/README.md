# ABE Cubed

## Scheme Variants

| Module | Description                                                                                 |
|---------|--------------------------------------------------------------------------------------------|
| `opt0` | The scheme as it is specified in Figure 1                                                   |
| `opt1` | Same as `opt0` but with optimized decryption (i.e. epsilons are grouped to reduce pairings) |
| `opt2` | Same as `opt1` but with randomness reuse in the keys                                        |
| `opt3` | Same as `opt2` but with randomness reuse in the ciphertexts                                 |
| `opt4` | Same as `opt3` but with positive randomness splitting                                       |
| `opt5` | Same as `opt3` but with negative randomness splitting                                       |
| `opt6` | Same as `opt3` but with postive and negative randomness splitting for negative parts        |
