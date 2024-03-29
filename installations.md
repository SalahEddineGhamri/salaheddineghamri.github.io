# Installs
The following installs are needed to start developing the single page app.

## install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
choose option 1 for default and then source the .bachrc to expose cargo.

## install webassembly
```
rustup target add wasm32-unknown-unknown
```

## install trunk
trunk is used to deploy the website
```
cargo install --locked trunk
```

## other installs
```
sudo apt install libssl-dev
cargo install cargo-generate --locked cargo
```

# Create the project
Run this command the first time:
```
cargo generate --git https://github.com/yewstack/yew-trunk-minimal-template
```
then you can change the project later on.
