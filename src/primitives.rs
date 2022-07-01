use std::collections::BTreeMap;
use std::error::Error;

use rustbreak::backend::FileBackend;
use rustbreak::deser::Yaml;
use rustbreak::Database;
use serenity::prelude::TypeMapKey;

pub const DEFAULT_PREFIX: &str = "sudo";

pub type ErrorBox<T> = Result<T, Box<dyn Error>>;

pub struct Prefixes;

impl TypeMapKey for Prefixes {
    type Value = Database<BTreeMap<u64, String>, FileBackend, Yaml>;
}

/// Returns a formated String in a codeblock format
pub trait ToCodeBlock {
    fn to_code_block(&self, t: &str) -> String;
}

pub trait ToClapCommand {
    fn to_clap_command(&self, prefix: String) -> Vec<String>;
}

impl ToClapCommand for String {
    fn to_clap_command(&self, prefix: String) -> Vec<String> {
        self.replace(&prefix, "")
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect()
    }
}

impl ToCodeBlock for String {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
    }
}

impl ToCodeBlock for str {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
    }
}

pub mod commands {
    use clap::{Arg, Command};

    pub fn ping() -> Command<'static> {
        Command::new("NAME: Ping")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Calculates API ping")
    }

    pub fn prefix() -> Command<'static> {
        Command::new("NAME: prefix")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Swaps the Guild prefix")
            .args([
                Arg::new("prefix")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("The new prefix to swap for"),
                Arg::new("show")
                    .help("Shows the current prefix")
                    .long("show")
                    .short('s'),
            ])
    }

    pub fn kick() -> Command<'static> {
        Command::new("NAME: kick")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Kicks a member from the guild.")
            .args([
                Arg::new("@mention/ID")
                    .required(true)
                    .index(1)
                    .help("The user that may be kicked."),
                Arg::new("reason")
                    .required(false)
                    .multiple_values(true)
                    .index(2)
                    .help("The reason for the kick."),
            ])
    }

    pub fn ban() -> Command<'static> {
        Command::new("NAME: ban")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Bans a member from the guild.")
            .args([
                Arg::new("@mention/ID")
                    .required(true)
                    .index(1)
                    .help("The user that may be banned."),
                Arg::new("reason")
                    .index(2)
                    .multiple_values(true)
                    .help("The reason for the ban."),
                Arg::new("delete-messages")
                    .long("delete-messages")
                    .short('d')
                    .takes_value(true)
                    .validator(|v| v.parse::<u8>().map_err(|_| "Not a valid number"))
                    .help("Select a number of days to delete messages."),
            ])
    }

    pub fn unban() -> Command<'static> {
        Command::new("NAME: unban")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Unbans a member from the guild.")
            .args([Arg::new("ID")
                .required(true)
                .index(1)
                .help("The user's that may be banned ID.")])
    }

    pub fn clear() -> Command<'static> {
        Command::new("NAME: clear")
            .disable_colored_help(true)
            .disable_version_flag(true)
            .about("\nABOUT: Clears messages in current channel")
            .args([
                Arg::new("amount")
                    .required(true)
                    .help("The amount of messages to clear (<=100)")
                    .index(1)
                    .validator(|v|
                        if v.parse::<u64>().is_ok() && v.parse::<u64>().unwrap() <= 1000 {
                            Ok(())
                        } else {
                            Err("Amount must be <=1000".to_string())
                        }
                    ),

                Arg::new("@mention/ID")
                    .long("user")
                    .short('u')
                    .takes_value(true)
                    .help("Specify a user to delete messages"),
                Arg::new("from_message")
                    .long("message")
                    .short('m')
                    .takes_value(true)
                    .help("Specify a message to start from (non-inclusive)"),
                Arg::new("after")
                    .conflicts_with("before")
                    .long("after")
                    .short('a')
                    .requires("from_message")
                    .help("Selects messages before the starting point"),
                Arg::new("before")
                    .conflicts_with("after")
                    .long("before")
                    .short('b')
                    .requires("from_message")
                    .help("Default selection from starting message, this flag exists for logic purposes")
            ])
    }
}
