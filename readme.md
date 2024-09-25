isw
===

![screenshot](repo_assets/screenshot.png)

a simple terminal stopwatch application.

features:
  
  - a basic stopwatch in a nice tui
  - support for intervals
    - per-interval stopwatch colour
    - notifications on interval boundaries
    - counting up or down to interval boundaries
    - displaying the number of intervals or cycles elapsed
  - writing the final time to stdout

installation
------------

### crates.io

the app can be installed from [crates.io](https://crates.io) using the following command:

``` fish
cargo install isw
```

if cargo is configured correctly, the command should then be available in your path.

### cargo

to build the application yourself, clone the source code and run the following command:

``` fish
cargo build --release
```

then copy/move the resulting executable at `./target/release/isw` into your path.

### nix

a `default.nix` file is available in this repo to build the application. it needs access to [nixpkgs](https://github.com/NixOS/nixpkgs/) to build, which can be provided by the channel on your system (if configured) with:

``` fish
nix-build -E 'with import <nixpkgs> {}; callPackage ./default.nix {}'
```

then copy/move the resulting executable at `./result/bin/isw` into your path.

usage
-----

a few options are available in the cli. these can be listed using the help flag:

``` fish
isw -h
```
```
a simple terminal stopwatch application

Usage: isw [OPTIONS]

Options:
  -i, --intervals <INTERVALS>  Intervals to cycle colour on (comma-separated seconds)
  -c, --colours <COLOURS>      Colours to represent each interval (comma-separated ANSI colours (0-7))
  -d, --descending             Count down to each interval boundary
  -p, --pause                  Pause on interval boundaries (p or space to unpause)
  -s, --shell <SHELL>          Execute a shell command at the end of intervals ("%i" for interval and "%c" for cycle)
      --show-interval          Show the number of intervals elapsed
      --show-cycle             Show the current number of interval cycles elapsed
  -h, --help                   Print help
  -V, --version                Print version
```

keybindings while running the tui are as follows:

| key   | action |
|-------|--------|
| q     | quit   |
| p     | pause  |
| space | pause  |
| r     | reset  |

examples
--------

### pomodoro

a typical 25 minutes on, 5 minutes off pomodoro timer that counts down to and sends a notification on interval boundaries. it also pauses to allow time to finish up tasks etc.:

``` fish
isw \
  --intervals 1500,300 \
  --colours 2,1 \
  --shell 'notify-send isw interval' \
  --pause \
  --descending \
  --show-cycle
```

### interval training

cycles of 1 minute off, 1 minute on for nasty interval training on an exercise bike:

``` fish
isw \
  --intervals 60,60 \
  --colours 2,1 \
  --show-cycle
```

### shortcut

for simple, quick, easy to read timing, clicking the clock in my system status bar launches the following command:

``` fish
foot --font 'CaskaydiaCove NF:size=96' isw
```
