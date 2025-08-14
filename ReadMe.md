# Timer CLI

## Description

A simple command-line countdown timer written in Rust.

## Features

- Accepts human-friendly duration formats (e.g., `10s`, `5m`, `2h`, `1d`, `10 seconds`).
- Defaults to 10 seconds if no duration is provided.
- Caps maximum duration at 10 years.
- Displays time remaining in `HH:MM:SS`.
- Clears and updates the line dynamically.
- Handles *most* errors gracefully.

## Usage

```bash
timer <duration>
```

### Duration format

- `10s`, `30sec`, `45seconds`
- `500ms`, `1500msec`
- `5m`, `2min`, `10minutes`
- `1h`, `2hr`, `3hours`
- `1d`, `2days`

If no duration is provided, defaults to **10 seconds**.
Maximum supported duration: **10 years**.
Ignores whitespace between time and duration (`5 s`).

## Examples

```bash
timer 10s     # 10 seconds
timer 5m      # 5 minutes
timer 2h      # 2 hours
timer 1d      # 1 day
```

## Build

```bash
git clone https://github.com/Andrew-McCall/timer-cli-rs.git
cd timer-cli-rs
cargo build --release
./target/release/timer 30s
```

## License

MIT License
Copyright (c) 2025 Andrew McCall

