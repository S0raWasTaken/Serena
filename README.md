<p>
  <img src="./serena.png" width="100" align="left">
  </br>
</p>

# Serena
An utility bot meant to be pretty overpowered (currently WIP).

## About
Serena aims to serve basic utility for server control and moderation.<br>
Most of today's bots lack a lot of functionality on a ton of their utilities and that's why Serena exists.

## Goals and TODOs
- Add basic commands (ping, prefix) ✅
- Add help command 🕐
- Add moderation commands (clear, ban, kick, mute/timout...) 🕐
- Add server editing commands (massive actions, like editing a bunch of channels in a single go) 🕐
- Make pancakes while driving ❌

## Help
### Commands
Every command can be called with `-h` or `--help` flag to show what its usage is.<br>
The commands are made using POSIX standards, the same that most Linux CLI commands use,
so if you're a unix enthusiast, you'll for sure like this bot.

Here's a quick command list (and their aliases):
```ini
[General]
ping
prefix
```
### Unusual Behaviour
Issues should be reported [here](https://github.com/S0raWasTaken/Threadripper-bot/issues)

## Adding Serena to your server
Serena isn't ready for production yet

## Contributing
Pull requests, issues and ideas are always welcome. Ideas should be sent in the [issues page](https://github.com/S0raWasTaken/Threadripper-bot/issues)

## Nerdy curiosities
- Serena's name is both based on the [Serenity](http://crates.io/crates/serenity) Rust crate and Luythna(bot image's artist)'s AMP Serena
- [Clap](http://crates.io/crates/clap) is included as an argument handler for commands. Most of errors aren't even written by me, but thrown by Clap itself
- Double `RwLock`s are hell
