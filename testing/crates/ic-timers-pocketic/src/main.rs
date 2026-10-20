mod build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = clap::Command::new("ic-timers-pocketic")
        .about("Build and run timer qualification through the corresponding Make target")
        .arg(
            clap::Arg::new("mode")
                .required(true)
                .value_parser(["watchdog", "cohorts"]),
        )
        .get_matches();
    build::run(args.get_one::<String>("mode").expect("required mode"))
}

#[cfg(test)]
mod cohorts;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod tests;
