# Command-Line Timer 

## Description

A simple command-line countdown timer written in Rust.

## Features

- Accepts human-friendly duration formats (e.g. `10s`, `5m`, `2h`, `1d`).
- Defaults to 10 seconds if no duration is provided.
- Caps maximum duration at 10 years.
- Displays time remaining in `HH:MM:SS`.
- Clears and updates the line dynamically.
- Handles *most* errors gracefully.
- Can execute command on completion.

## Usage

```bash
timer <duration> <*command>
```
**command is optional*

### Duration format

- `10s`, `30sec`, `60seconds`
- `5m`, `2min`, `10minutes`
- `1h`, `2hr`, `3hours`
- `1d`, `2days`

If no duration is provided, defaults to **10 seconds**.  
Maximum supported duration: **10 years**.

## Install

### Cargo Install

```bash 
cargo install timer-cli-rs
```

### Package Manager (Arch)

```bash
git clone https://github.com/Andrew-McCall/timer-cli-rs.git
cd timer-cli-rs
makepkg -si
```

## Build from Source

```bash
git clone https://github.com/Andrew-McCall/timer-cli-rs.git
cd timer-cli-rs
cargo build --release
./target/release/timer 30s 'echo "Hello World"'
```

## Examples

```bash
timer 10s   'echo "10 seconds"'  
timer 5m    'echo "5 minutes"'  
timer 2h    'echo "2 hours'  
timer 1d    'echo "1 day"'  
```


## License

MIT License
Copyright (c) 2025 Andrew McCall

