# anyshortcut-cli
CLI (Commnad Line Interface) version of Anyshortcut write with **Rust**.

## Usages

### Open primary shortcut
> `as [a~z|0~9]`

### Open secondary shortcut
> `as [a~z|0~9] [a~z|0~9]`

### Open compound shortcut
> `as [a~z][a~z]`

### Login with token
> `as login [token]`

### Sync shortcuts
> `as sync`

### List shortcuts
> `as list [-p|-s|-c] [-v]`

- `-p` list primary shortcuts
- `-s` list secondary shortcuts
- `-c` list compound shortcuts

`-v` to display verbose detail of shortcut list

### Logout
> `as logout`

Any data will be cleaned.

## Stack

### JSON, yaml serilize and deserilize
> https://github.com/serde-rs/serde

### HTTP client

> https://github.com/alexcrichton/curl-rust


### Reference
```
https://github.com/getsentry/sentry-cli
```

