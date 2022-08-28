use clap::Parser;
use notify_rust::{Hint, Notification, Urgency};
use std::{thread::sleep, time::Duration};

#[derive(Parser)]
struct Cli {
    /// Do not print anything to stdout
    #[clap(short, long, action)]
    quiet: bool,
    /// Output File to write the remaining time
    #[clap(parse(from_os_str), short, long, default_value = "")]
    output: std::path::PathBuf,
    /// Command to run when timer runs out
    #[clap(short, long)]
    command: Option<String>,
    /// Refresh Rate in miliseconds
    #[clap(short, long, default_value = "200")]
    refresh_rate: u64,
    /// Notify when this much time is remaining
    #[clap(short, long, min_values=0, parse(try_from_str=duration_sec), value_name="DURATION")]
    notify: Vec<Duration>,
    /// Duration multiplier
    #[clap(short, long, default_value = "1")]
    multiplier: u32,
    /// Duration for the timer in seconds
    #[clap(short, long, default_value = "60", parse(try_from_str=duration_sec))]
    duration: Duration,
    /// Summary for the timer
    #[clap(short, long, default_value = "You set a reminder.")]
    summary: String,
}

fn duration_sec(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let seconds = arg.parse()?;
    Ok(std::time::Duration::from_secs(seconds))
}

fn send_notification(compl: u32, time: Duration, summary: String) {
    let sum: String = format!("{} seconds remain", time.as_secs()).to_string();
    Notification::new()
        .summary(&sum)
        .body(&summary)
        .appname("Timer")
        .hint(Hint::CustomInt(
            "value".to_string(),
            compl.try_into().unwrap(),
        ))
        .hint(Hint::Custom("timer".to_string(), "time".to_string()))
        .show()
        .unwrap();
}

fn main() {
    let args = Cli::parse();
    if !args.quiet {
        println!(
            "Waiting {} seconds for: {}",
            (args.duration * args.multiplier).as_secs(),
            args.summary.clone()
        );
    }

    let mut notify_durs = args.notify.clone();
    notify_durs.sort();
    notify_durs.reverse();

    let durations: Vec<Duration> = notify_durs
        .into_iter()
        .map(|d| args.duration * args.multiplier - d)
        .collect();

    send_notification(0, args.duration * args.multiplier, args.summary.clone());

    let mut prev_sleep = Duration::ZERO;
    for dur in durations {
        sleep(dur - prev_sleep);
        send_notification(
            (100.0 - 100.0 * dur.as_secs_f32() / (args.duration * args.multiplier).as_secs_f32())
                as u32,
            args.duration * args.multiplier - dur,
            args.summary.clone(),
        );
        prev_sleep = dur;
    }

    sleep(args.duration * args.multiplier - prev_sleep);
    Notification::new()
        .summary("Time's up")
        .body(args.summary.as_str())
        .appname("Timer")
        .hint(Hint::Urgency(Urgency::Critical))
        .show()
        .unwrap();
}
