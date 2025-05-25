# Dark Molecule | Framework for Penetration Testers

## Deploy

### via Docker (NOT READY YET)

### without Docker (hard way)

For backend:
Install diesel and run this commands:

```shell
cd backend 
diesel setup
cargo run --release
```

At the first launch, an administrator account will be generated and displayed in the logs of backend.

For frontend (in directory with frontend folder):
```shell
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve --release
```

## Configuration

For backend in backend/.env

## Configuration (for Docker Deploy way)


## TODO
- [ ] Modules with some useful tools (lua)
- [ ] Functionality for adding helpers for handlebar
- [ ] Change synchronous postgres client to asynchronous
