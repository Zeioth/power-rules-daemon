Daemon written in Rust to automatically change your power profile while a program is executed.

## Table of contents

- [How to install](#how-to-install)
  - [Arch Linux](#arch-linux)
  - [Other distros](#other-distros)
- [Config file](#config-file)
- [How to distribute this program](#how-to-distribute-this-program)

## How to install
### Arch Linux
Install with
```sh
paru -S power-profiles-daemon
```

And enable the services
```sh
systemctl enable --now power-profiles-daemon.service
systemctl enable --now power-rules-daemon.service
```

### Other distros
You MUST have `power-profiles-daemon` installed in your system, and its service MUST be running before using the `power-rules-daemon`.

Install with
```sh
cargo install power-rules-daemon
```

Create the service in `~/.config/systemd/user/power-rules-daemon.service` with 

```sh
echo "[Unit]
Description=Power Rules Daemon
After=graphical-session.target

[Service]
ExecStart=%h/.cargo/bin/power-rules-daemon
Restart=on-failure

[Install]
WantedBy=default.target" > ~/.config/systemd/user/power-rules-daemon.service
```

And enable it with

```sh
systemctl --user daemon-reload && systemctl --user enable --now power-rules-daemon.service
```

## Config file
You must manually create your config file in `~/.config/power-rules/config.toml`

```toml
# Some tips!
# - Changes in this file will be applied in real time.
# - Rule process name matching can be partial.
# - Rules will be applied in natural order.

[config]
default_profile = "balanced"  # Profile to use when no rules are triggered atm.
polling_interval = 5          # Amount of seconds before checking if a rule is triggered.
pause_on_manual_change = 180  # If the user manually changes the power profile (through the desktop environment gui, for example), the daemon is paused for n minutes.

# While a steam game is executed
[[rule]]
name = "steamapps/common"
profile = "performance"

# While launchers are executed
[[rule]]
name = "lutris"
profile = "performance"

[[rule]]
name = "heroic"
profile = "performance"

[[rule]]
name = "gamehub"
profile = "performance"

[[rule]]
name = "retroarch"
profile = "performance"
```

## 🌟 Get involved
If you wanna help the project leave a Star!
[![Stargazers over time](https://starchart.cc/Zeioth/power-rules-daemon.svg)](https://starchart.cc/Zeioth/power-rules-daemon)
  
## How to distribute this program
Distributing this program in your linux distro is very easy! The installer should just:

- Install the program with: `cargo install power-rules-daemon`
- Copy `power-rules-daemon.service` from the repo (or this readme).
- Add the program `power-profiles-daemon` as dependency.

Let the users enable/start the services for `power-rules-daemon`/`power-profiles-daemon`.

## TODOS
- Implement proper log files support.
- Tests (once features are confirmed).
- CI tests pipeline.
- Better docstrings, so nice docs can be CI pipelined.

- Let's consider a few options for notifications, in case the user want to have them (it makes easier to visually confirm what's going on).
- ~~A man file is probably a good idea.~~ → Installing man pages require sudo permissions, and we don't want that, so let's go with only README.md for now.

## Credits
This program was originally [a proposal](https://github.com/CachyOS/CachyOS-Settings/pull/157) to replace the program `game-performance` on CachyOS. And now it can be used on any distro!
