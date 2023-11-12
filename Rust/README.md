# tp-wik-dps-tp01 - Rust version
## Build & Run
### Software needed
```
rust, cargo, git
```
1. Clone the repository
```
git clone https://github.com/arpaflow/tp-wik-dps-tp01.git
```
2. Change directory
```
cd Rust
```
4. Export the PING_LISTEN_PORT environnement variable with the port of your choice
```
export PING_LISTEN_PORT=<port>
```
5. Build & Run
```
cargo run
```
6. Test
```
curl http://localhost:8000/ping
```