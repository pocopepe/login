// This file is part of the uutils login package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use clap::crate_version;
use clap::{Arg, Command};
use std::process;
use uucore::libc::EXIT_FAILURE;
use uucore::{error::UResult, format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("nologin.md");
const USAGE: &str = help_usage!("nologin.md");

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let _matches = uu_app().try_get_matches_from(args)?;

    //default behaviour
    println!("This account is currently not available.\n");
    process::exit(EXIT_FAILURE);
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new("command")
                .short('c')
                .long("command")
                .value_name("command")
                .num_args(1)
                .help("does nothing (for compatibility with su -c)"),
        )
        .arg(
            Arg::new("init-file")
                .long("init-file")
                .value_name("file")
                .num_args(1),
        )
        .arg(Arg::new("interactive").short('i').long("interactive"))
        .arg(Arg::new("login").short('l').long("login"))
        .arg(Arg::new("noprofile").long("noprofile"))
        .arg(Arg::new("norc").long("norc"))
        .arg(Arg::new("posix").long("posix"))
        .arg(
            Arg::new("rcfile")
                .long("rcfile")
                .value_name("file")
                .num_args(1),
        )
        .arg(Arg::new("restricted").short('r').long("restricted"))
}
