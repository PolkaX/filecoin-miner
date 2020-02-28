use std::io::Write;

use lazy_static::lazy_static;
use log::info;
use regex::Regex;

/// Initialize the logger
pub fn init_logger(pattern: &str) {
    use ansi_term::Colour;

    let mut builder = env_logger::Builder::new();
    // Disable info logging by default for some modules:
    builder.filter(Some("ws"), log::LevelFilter::Off);
    // Enable info for others.
    builder.filter(None, log::LevelFilter::Info);

    if let Ok(lvl) = std::env::var("RUST_LOG") {
        builder.parse_filters(&lvl);
    }

    builder.parse_filters(pattern);
    let isatty = atty::is(atty::Stream::Stderr);
    let enable_color = isatty;

    builder.format(move |buf, record| {
        let now = time::now();
        let timestamp =
            time::strftime("%Y-%m-%d %H:%M:%S", &now).expect("Error formatting log timestamp");

        let mut output = if log::max_level() <= log::LevelFilter::Info {
            format!(
                "{} {}",
                Colour::Black.bold().paint(timestamp),
                record.args()
            )
        } else {
            let name = ::std::thread::current()
                .name()
                .map_or_else(Default::default, |x| {
                    format!("{}", Colour::Blue.bold().paint(x))
                });
            let millis = (now.tm_nsec as f32 / 1000000.0).round() as usize;
            let timestamp = format!("{}.{:03}", timestamp, millis);
            format!(
                "{} {} {} {}  {}",
                Colour::Black.bold().paint(timestamp),
                name,
                record.level(),
                record.target(),
                record.args()
            )
        };

        if !isatty && record.level() <= log::Level::Info && atty::is(atty::Stream::Stdout) {
            // duplicate INFO/WARN output to console
            println!("{}", output);
        }

        if !enable_color {
            output = kill_color(output.as_ref());
        }

        writeln!(buf, "{}", output)
    });

    if builder.try_init().is_err() {
        info!("Not registering Substrate logger, as there is already a global logger registered!");
    }
}

fn kill_color(s: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\x1b\\[[^m]+m").expect("Error initializing color regex");
    }
    RE.replace_all(s, "").to_string()
}
