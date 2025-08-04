## How to install

```
<your-package-manager-install-command> power-profiles-daemon
cargo install power-rules-daemon
```

And enable the next services

```sh
systemctl --user daemon-reload
systemctl --user enable --now power-profiles-daemon.service
systemctl --user enable --now power-rules-daemon.service
```

Now you can configure your rules in `~/.config/power-rules/config.toml`

## How to debug
You can manually run the command `power-rules`. It will show info every time
a rule is triggered, or the config file changes.

## Example config file
```toml
[config]
default_profile = "balanced"  # Profile to use when no rules are triggered atm.
polling_interval = 5          # Amount of seconds before checking if a rule is triggered.
pause_on_manual_change = 180  # If the user manually changes the power profile (through the desktop environment gui, for example), the daemon is paused for n minutes.

[[rule]]
name = "eldenring.exe"        # A string to match in the process name.
profile = "performance"       # The power profile to switch to.

[[rule]]
name = "firefox"
profile = "balanced"
```

## Rules example
Most users will use this daemon for gaming with rules like

```toml
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

## Rule order
Rules are applied by order from above to below of the config file.

So if for example you are running `eldenring.exe` and `firefox` at the same time, the rule defined at the bottom of the file will be the one applied (firefox, on the example config). And when you close firefox, the rule from eldenring.exe will be applied, etc.

if no rules are currently triggered, `default_profile` will be the one used.

## More info
- This program requires GNU Linux.

## WIP: How to distribute
If you want to distribute this program in a linux distro, the installer should:
- Install the program with: `cargo install power-rules-daemon`
- Copy the service (the user can enable it / start it himself).

It's a good practice you make your distro installer check if rust is in the user path (meaning he will be able to execute it).

## TODOS
- The program works, but we still have to make it installable through cargo.
- Is cargo gonna be responsible of installing the service? I assume yes.
- Is cargo gonna create a default config file? I assume yes. → But create it only if the directory is empty! We don't want to smash user changes.
- Implement proper log files support.
- Tests (once features are confirmed).
- CI tests pipeline.
- Better docstrings, so nice docs can be CI pipelined.
- A man file is probably a good idea.
- Let's consider a few options for notifications, in case the user want to have them (it makes easier to visually confirm what's going on).

## Credits
This progaram was originally [a proposal](https://github.com/CachyOS/CachyOS-Settings/pull/157) to replace the program `game-performance` on CachyOS. And now it can be used on any distro!
