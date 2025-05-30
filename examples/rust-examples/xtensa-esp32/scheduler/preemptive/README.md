# Rust example for xtensa esp32 architecture

Presented here is a straightforward Rust example utilizing Martos with preemptive scheduler.

Two tasks are created. They take turns increasing the counter value and printing it out until it reaches 20.
Periodically, one preempts the other

## How to install dependencies

For comprehensive guidance on installing the necessary dependencies for developing applications targeting the Xtensa
ESP32 architecture,
please refer to [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).
Below is an illustrative example demonstrating the installation of building toolchains on a Linux (Ubuntu/Debian):

```
apt-get -qq update
apt-get install -y -q build-essential curl
curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install espup
espup install
```

## How to build the example

For a thorough guide on developing projects for the Xtensa ESP32 architecture across various operating systems,
we recommend
consulting [the official website](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables).
Below, you will find an illustrative example showcasing the building process on a Linux system (Ubuntu/Debian):

```
. $HOME/export-esp.sh
cargo build --release
```

## How to run the example

For detailed instructions on running projects for the Xtensa ESP32 architecture across various operating systems,
we recommend consulting [the official website](https://docs.esp-rs.org/book/tooling/espflash.html).
Below, you will find an illustrative example showcasing the running on a Linux system (Ubuntu/Debian):

```
cargo run
```
