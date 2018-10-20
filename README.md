![](./anyshortcut-cli.jpg)
# Anyshortcut Command Line Interface

[![Travis Build Status](https://travis-ci.com/anyshortcut/anyshortcut-cli.svg?branch=master)](https://travis-ci.com/anyshortcut/anyshortcut-cli) 
[![license-mit](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/anyshortcut/anyshortcut-cli/blob/master/LICENSE-MIT)
[![license-apache](https://img.shields.io/badge/license-Apache-yellow.svg)](https://github.com/anyshortcut/anyshortcut-cli/blob/master/LICENSE-APACHE)
[![Version info](https://img.shields.io/crates/v/anyshortcut.svg)](https://crates.io/crates/anyshortcut)

A blaze fast way to launch your favorite website in Terminal.

## Installation

#### Cargo

`cargo install anyshortcut`

#### Homebrew

`brew install anyshortcut`

> Unmerged: https://github.com/Homebrew/homebrew-core/pull/33198

#### Install binary file from Github release

https://github.com/anyshortcut/anyshortcut-cli/releases

**Recommend**

> Give it an alias name such as **as** by adding following line to your 
> **.bashrc** or **.zshrc** file:
>
> ```shell
> alias as=$(which anyshortcut)
> ```
> 
> then source your profile to make it works.


## Usage

```
$ anyshortcut
A blaze fast way to launch your favorite website in Terminal.

USAGE:
    anyshortcut [ARGS] [SUBCOMMAND]

ARGS:
    <PRIMARY_KEY | COMPOUND_KEY>
            Using primary shortcut key (A~Z|0~9) or compound shortcut key (AA~ZZ) to open the url.

    <SECONDARY_KEY>
            Use secondary shortcut key (A~Z|0~9) to open the url.


SUBCOMMANDS:
    list      List shortcuts.
    login     Login with the token.
    logout    Logout and clean local data.
    sync      Sync all shortcuts after login.

```

- `as <PRIMARY_KEY>`

Using the primary shortcut to launch the website. 

**PRIMARY_KEY** is in the form of a case-insensitive alphanumeric letter range **A ~ Z** or **0 ~ 9**.

For example:
```
$ as g
Url: https://www.google.com/
```

- `as <COMPOUND_KEY>`

Using the compound shortcut to launch the website.

**COMPOUND_KEY** is in the form of two case-insensitive alphabet letters range **AA ~ ZZ**.

For example:
```
$ as db
Url: https://www.dropbox.com/
```

- `as <PRIMARY_KEY>|<COMPOUND_KEY> <SECONDARY_KEY>`

Using the secondary shortcut to launch the website.

**SECONDARY_KEY** is in the form of an case-insensitive alphanumeric letter range **A ~ Z** or **0 ~ 9**.

For example:
```
$ as g t
Url: https://translate.google.com/
```

- `as login [TOKEN]` or `as login` to prompt input `TOKEN`

Login with access token then sync your shortcuts automatically. You can find the access token at 
official website [Account Profile](https://anyshortcut.com/account#/profile/) -> **API Access**.

- `as sync`

Sync your shortcuts to local manually.

```
$ as sync
Syncing your shortcut data...

Shortcuts synced success!
Primary shortcut number: 120
Secondary shortcut number: 150

All your data stored at ~/.anyshortcut directory.
```

- `as list`

List your shortcuts.

```
USAGE:
    anyshortcut list [OPTIONS]

OPTIONS:
    -c, --compound     List all compound shortcuts.
    -p, --primary      List all primary shortcuts.
    -s, --secondary    List all secondary shortcuts.
```

- `as logout`

Logout and all local data will be cleaned.

## Future plans

- [ ] Support bind shortcut
- [ ] Support delete shortcut
