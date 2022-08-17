use clap::Parser;

#[macro_use]
mod helper_macros;
mod win32_sound;
mod misc_defs;
mod cli_def;
mod command_logic;

use command_logic::*;
use cli_def::*;

fn main() {
    let args = CliDef::parse();

    match args.command {
        CommandName::Mute(mute_args) => {
            mute_logic(mute_args);
        },
        CommandName::Eq(eq_args) => {
            eq_logic(eq_args);
        },
    }

}

