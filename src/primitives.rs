use std::{collections::BTreeMap, error::Error};

use rustbreak::{backend::FileBackend, deser::Yaml, Database};
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
                Arg::new("@Mention")
                .required(true)
                .index(1)
                .help("The member that may be kicked.")
            ])
    }
}
