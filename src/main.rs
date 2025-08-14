use std::env;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_SECONDS: u64 = 10;
const MAX_SECONDS: u64 = 10 * 365 * 24 * 3600; // 10 years cap

fn parse_duration_arg(arg: &str) -> Result<Duration, String> {
    let mut s = arg.trim().to_ascii_lowercase();
    s.retain(|c| !c.is_whitespace());

    let digits_end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (num_part, unit_part) = s.split_at(digits_end);

    if num_part.is_empty() {
        return Err("no numeric portion found".into());
    }

    let number = num_part
        .parse::<u64>()
        .map_err(|_| format!("invalid number '{}'", num_part))?;

    let seconds = match unit_part {
        "" | "s" | "sec" | "secs" | "second" | "seconds" => number,
        "m" | "min" | "mins" | "minute" | "minutes" => number.saturating_mul(60),
        "h" | "hr" | "hrs" | "hour" | "hours" => number.saturating_mul(60 * 60),
        "d" | "day" | "days" => number.saturating_mul(60 * 60 * 24),
        other => return Err(format!("unknown unit '{}'", other)),
    };

    if seconds == 0 {
        return Err("duration resolves to zero seconds".into());
    }

    if seconds > MAX_SECONDS {
        return Err(format!("duration too large (max {} seconds)", MAX_SECONDS));
    }

    Ok(Duration::from_secs(seconds))
}

fn format_hms(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

fn clear_line<W: Write>(out: &mut W) -> io::Result<()> {
    out.write_all(b"\r\x1B[2K")
}

fn countdown(d: Duration) -> io::Result<()> {
    if d.as_secs() == 0 {
        eprintln!("zero duration, nothing to do");
        return Ok(());
    }

    let total = d.as_secs();
    let start = Instant::now();
    let mut out = io::stdout().lock();

    loop {
        let elapsed = start.elapsed().as_secs();
        let remaining = total.saturating_sub(elapsed);
        if remaining == 0 {
            break;
        }

        clear_line(&mut out)?;
        write!(out, "Time remaining: {} ", format_hms(remaining))?;
        out.flush()?;

        thread::sleep(Duration::from_secs(1));
    }

    clear_line(&mut out)?;
    writeln!(out, "Time's up!")?;
    out.flush()?;
    Ok(())
}

fn print_usage_and_default() {
    eprintln!("Usage: timer <duration> <*command>");
    eprintln!("Examples: `timer 10s`, `timer 5m`, `timer 2h`, `timer 1d /path/to/script.sh`");
    eprintln!(
        "No duration provided — defaulting to {} seconds.",
        DEFAULT_SECONDS
    );
}

fn run_command(args: &[String]) -> std::io::Result<()> {
    let program = match args.first() {
        Some(p) => p,
        None => return Ok(()),
    };

    Err(Command::new(program).args(&args[1..]).exec())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let dur = if args.len() < 2 {
        print_usage_and_default();
        Duration::from_secs(DEFAULT_SECONDS)
    } else {
        match parse_duration_arg(&args[1]) {
            Ok(dur) => dur,
            Err(e) => {
                eprintln!(
                    "Error parsing duration: {}. Defaulting to {} seconds.",
                    e, DEFAULT_SECONDS
                );
                Duration::from_secs(DEFAULT_SECONDS)
            }
        }
    };

    if let Err(e) = countdown(dur) {
        eprintln!("I/O error during countdown: {}", e);
    } else if args.len() > 2 {
        let cmd_args = &args[2..];
        println!("{}", cmd_args.join(" "));
        if let Err(e) = run_command(cmd_args) {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
