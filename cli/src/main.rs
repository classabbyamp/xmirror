use std::{path::PathBuf, process::{ExitCode, Termination}, fmt::{Debug, Display}, ptr::read, fs::File, io::BufReader, error::Error, collections::HashMap, hash::Hash};

use clap::Parser;
use log::{error, debug};
use nix::errno::Errno;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use an alternative mirror list file
    #[arg(short = 'l', long, value_name = "LIST", default_value = "/usr/share/xmirror/mirrors.json")]
    mirror_list: Option<PathBuf>,
    /// Don't attempt to update the mirror list
    #[arg(short, long)]
    no_fetch: bool,
    /// Use an alternative rootdir (acts like xbps's -r flag)
    #[arg(short, long, value_name = "DIR")]
    rootdir: Option<PathBuf>,
    /// Set the current mirror to MIRROR and exit, skipping the interactive TUI
    #[arg(short, long, conflicts_with = "default", value_name = "MIRROR")]
    set: Option<String>,
    /// Reset the current mirror to the default, skipping the interactive TUI
    #[arg(short, long)]
    default: bool,
}

fn main() -> ExitCode {
    env_logger::Builder::from_default_env().format_timestamp(None).init();

    let cli = Cli::parse();

    if ! nix::unistd::Uid::effective().is_root() {
        error!("must be run as root");
        return ExitCode::from(Errno::EPERM as u8);
    }

    if cli.default {
        reset_mirrors(cli.rootdir)
    } else if cli.set.is_some() {
        set_mirrors(cli.set.unwrap(), cli.rootdir)
    } else {
        // interactive
        todo!()
    }

    ExitCode::SUCCESS
}

fn reset_mirrors(rootdir: Option<PathBuf>) {
    todo!()
}

fn set_mirrors(baseurl: String, rootdir: Option<PathBuf>) {
    todo!()
}

fn fetch_mirrors(url: String) {
    todo!()
}

fn read_mirrors(path: PathBuf) -> Result<Vec<Mirror>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}

#[derive(Deserialize)]
struct Mirror {
    name: String,
    url: String,
    location: String,
    region: Region,
    tier: u32,
    enabled: bool,
    archs: HashMap<String, HashMap<String, Option<String>>>,
    paths: HashMap<String, String>,
}

// uses names and abbreviations from https://planetarynames.wr.usgs.gov/Abbreviations
#[derive(Deserialize)]
enum Region {
    AF,
    AN,
    AS,
    EU,
    NA,
    OC,
    SA,
    World,
    Space,
    Unknown(String),
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AF => write!(f, "Africa"),
            Self::AN => write!(f, "Antarctica"),
            Self::AS => write!(f, "Asia"),
            Self::EU => write!(f, "Europe"),
            Self::NA => write!(f, "North America"),
            Self::OC => write!(f, "Oceania"),
            Self::SA => write!(f, "South and Central America"),
            Self::World => write!(f, "Globally Available"),
            Self::Space => write!(f, "Universally Available"),
            Self::Unknown(x) => write!(f, "{x}"),
        }
    }
}

//- Utility

struct XError(String);

impl From<&str> for XError {
    fn from(msg: &str) -> Self {
        Self(msg.into())
    }
}

impl Debug for XError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("xmirror: {}", self.0))
    }
}
