mod build;
mod report;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = clap::Command::new("ic-timers-pocketic")
        .about("Build and run timer qualification through the corresponding Make target")
        .arg(
            clap::Arg::new("worker")
                .long("worker")
                .hide(true)
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand_required(true)
        .subcommand(clap::Command::new("watchdog"))
        .subcommand(
            clap::Command::new("cohorts")
                .arg(
                    clap::Arg::new("report")
                        .long("report")
                        .default_value("testing/target/cohorts.json")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                )
                .arg(
                    clap::Arg::new("baseline")
                        .long("baseline")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
        .get_matches();
    let (mode, options) = args.subcommand().expect("required subcommand");
    if args.get_flag("worker") {
        build::run(
            mode,
            if mode == "cohorts" {
                options.get_one::<std::path::PathBuf>("report")
            } else {
                None
            },
            if mode == "cohorts" {
                options.get_one::<std::path::PathBuf>("baseline")
            } else {
                None
            },
        )
    } else {
        let mut worker = std::process::Command::new(std::env::current_exe()?);
        worker.args(["--worker", mode]);
        if mode == "cohorts" {
            for flag in ["report", "baseline"] {
                if let Some(path) = options.get_one::<std::path::PathBuf>(flag) {
                    worker.arg(format!("--{flag}")).arg(path);
                }
            }
        }
        build::qualify(&mut worker, std::time::Duration::from_secs(30 * 60))?;
        Ok(())
    }
}

#[cfg(test)]
mod cohorts;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod tests;
