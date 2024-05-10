## CryptoPay CLI
### Description 📜
A simple and powerfull CLI for modular or complex testing CryptoPay.

```sh
CLI Test framework for cryptopay

Usage: ctf [OPTIONS] <COMMAND>

Commands:
  evm   EVM scope command
  api   API scope command
  db    Datebase scope command
  help  Print this message or the help of the given subcommand(s)

Options:
  -s             Skip all service check
  -h, --help     Print help
  -V, --version  Print version
```

### Dependency 🛠️

1. Cryptopay - [gitea](https://git.topg.systems/cryptopay/cryptopay)
2. Foundry (only anvil) - [github](https://github.com/foundry-rs)
3. postgres (may use dockerfile)

### How to use 🕹️

####  First time ⤵️
1. Clone `Cryptopay`
   - git clone ...
   - save env
   - cargo run
2. Clone `e2e`
   - git clone ...
3. Make db server with `dockerfile` or native `pg_ctl`
4. build release `e2e`
   - `cargo build --release`
5. Spawn anvil with `e2e` release build
   - `./ctf -s evm spawn`
6. Make migrations
   - `./ctf -s db drop`
7. Launch `cryptopay`
   - `cargo run`
8. Launch test service `e2e`
   - `./ctf service full`
9. Relaunch `cryptopay`
10. Press `enter` in `e2e` termina; session
11. Done


####  Next time ⤵️
1. Drop db
   - `./ctf -s db drop`
2. Respawn anvil with `e2e` release build
   - `./ctf -s evm spawn`
3. Launch `cryptopay`
   - `cargo run`
4. Launch test service `e2e`
   - `./ctf service full`
5. Relaunch `cryptopay`
6. Press `enter` in `e2e` termina; session
7. Done