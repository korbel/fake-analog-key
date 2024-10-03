# fake-analog-key

Creates a virtual device with two analog buttons and binds them to the side buttons of your mouse.

*This is a sample project, fork and modify it to your needs.*

## Development

You will need the rust toolchain to have installed. If you are new to Rust, follow these instructions: <https://rustup.rs/>

## Install

```shell
cargo build --release
sudo cp ./target/release/fake-analog-key /opt/fake-analog-key
sudo cp ./assets/systemd/fake-analog-key.service /etc/systemd/system/fake-analog-key.service
sudo systemctl enable --now fake-analog-key
```

## Uninstall

```shell
sudo systemctl disable --now fake-analog-key
sudo rm -rf /opt/fake-analog-key /etc/systemd/system/fake-analog-key.service
```