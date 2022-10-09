# leftwm-layout-switcher

[LeftWM](https://github.com/leftwm/leftwm) has the ability to change layout already as expected with any window manager. leftwm-layout-switcher adds notifications on top of that. This is useful when you want to know which layout you have switched to after hitting the set keybinding.

## usage

leftwm-layout-switcher usage is familiar with leftwm-command. It just consists of only two main subcommands.

```textile
USAGE:
    leftwm-layout-switcher <SUBCOMMAND>

OPTIONS:
    --help    Print help information

SUBCOMMANDS:
    help               Print this message or the help of the given subcommand(s)
    next-layout        Change to the Next layout
    previous-layout    Change to the Previous layout
```

## setup

After installing leftwm-layout-switcher binary and adding it to path, you can configure your keybinding to run leftwm-layout-switcher with the proper subcommand in the [leftwm's config](https://github.com/leftwm/leftwm/wiki/Config).



## installation

```shell
git clone https://github.com/MaarifaMaarifa/leftwm-layout-switcher
cd leftwm-layout-switcher
cargo install --force --path .
```
