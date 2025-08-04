## Requirements
You MUST have `power-profiles-daemon` installed, and its service MUSt be running before using the `power-rules-daemon`.

## How to install
Install with
```sh
cargo install power-rules-daemon
```

Create the service in `~/.config/systemd/user/power-rules-daemon.service` with 

```systemd
echo "[Unit]
Description=Power Rules Daemon
After=graphical-session.target

[Service]
ExecStart=%h/.cargo/bin/power-rules-daemon
Restart=on-failure

[Install]
WantedBy=default.target" > ~/.config/systemd/user/power-rules-daemon.service
```

And enable it with:

```sh
systemctl --user daemon-reload && systemctl --user enable --now power-rules-daemon.service
```



## Rules example
Now you can create your rules in `~/.config/power-rules/config.toml`

```toml
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

- Rules are applied in order.
- If no rules are currently triggered, `default_profile` will be the one used.
- Changes in the config file are applied in real time, so you don't need to reboot. 

## How to distribute this program
Distributing this program in your linux distro is very easy! The installer should just:

- Install the program with: `cargo install power-rules-daemon`
- Copy the service (the user can enable it / start it himself).
- Add the program `power-profiles-daemon` as dependency.

## TODOS
- Implement proper log files support.
- Tests (once features are confirmed).
- CI tests pipeline.
- Better docstrings, so nice docs can be CI pipelined.
- A man file is probably a good idea.
- Let's consider a few options for notifications, in case the user want to have them (it makes easier to visually confirm what's going on).

## Credits
This progaram was originally [a proposal](https://github.com/CachyOS/CachyOS-Settings/pull/157) to replace the program `game-performance` on CachyOS. And now it can be used on any distro!
