mod build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [mode] if mode == "watchdog" || mode == "cohorts" => build::run(mode),
        _ => Err(
            "usage: ic-timers-pocketic watchdog|cohorts (use the corresponding Make target)".into(),
        ),
    }
}

#[cfg(test)]
mod cohorts;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod tests;
