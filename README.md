# Blawgd-Solana
Solana implementation of blawgd protocol



### Usage
Run a solana validator in a seperate terminal
```
solana-test-validator
```

Compile the solana program to bpf bytecode and deploy it to the solana cluster
```
cargo build-bpf
solana program deploy ./target/deploy/blawgd_solana.so
```




### Test
```
cargo test -- --nocapture
```

