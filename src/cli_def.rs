use clap::{Parser, Args, Subcommand, ValueEnum, Command, Arg, ArgAction};
use std::fmt::Debug;

struct UnmuteArgs {

}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliDef {
    #[clap(subcommand)]
    pub command: CommandName,
}

#[derive(Subcommand)]
pub enum CommandName {
    Mute(MuteArgs),
    Eq(EqArgs),
}

/// stfu program!
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct MuteArgs {
    /// partial of exe name to search for
    #[clap(index(1), value_parser)]
    pub title: String,
    /// duration to mute the program
    #[clap(index(2), value_parser)]
    pub duration: u64,
}

/// equalize all audio sessions under a device to the device's volume
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct EqArgs {
}
