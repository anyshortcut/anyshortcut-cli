![](./anyshortcut-cli.jpg)
# Anyshortcut Command Line Interface

[![Travis Build Status](https://travis-ci.com/anyshortcut/anyshortcut-cli.svg?branch=master)](https://travis-ci.com/anyshortcut/anyshortcut-cli) 
[![license-mit](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/anyshortcut/anyshortcut-cli/blob/master/LICENSE-MIT)
[![license-apache](https://img.shields.io/badge/license-Apache-yellow.svg)](https://github.com/anyshortcut/anyshortcut-cli/blob/master/LICENSE-APACHE)
[![Version info](https://img.shields.io/crates/v/anyshortcut.svg)](https://crates.io/crates/anyshortcut)

A blaze fast way to launch your favorite website in Terminal.

## Features

- **Open primary shortcut**

> `as [a~z|0~9]`

- **Open secondary shortcut**

> `as [a~z|0~9] [a~z|0~9]`

- **Open compound shortcut**

> `as [a~z][a~z]`

- **Login with token**

> `as login [token]`

- **Sync shortcuts**

> `as sync`

- **List shortcuts**

> `as list [-p|-s|-c] [-v]`

- `-p` list primary shortcuts
- `-s` list secondary shortcuts
- `-c` list compound shortcuts

- **Logout**

> `as logout`

Any data will be cleaned.



## Installation

#### Cargo
`cargo install anyshortcut`

Recommend give it an alias name such as **as** by adding following line to your environment:

`alias as='~/.cargo/bin/anyshortcut'`

then source your profile to make it works.


#### Homebrew

`brew install anyshortcut`