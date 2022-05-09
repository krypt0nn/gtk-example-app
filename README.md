# GTK ❤️ Rust - Example app

This repo contains example gtk4 + libadwaita + rust application

For UI declarations I use [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler)

## Clone repo

```sh
git clone --recursive https://github.com/krypt0nn/gtk-example-app
```

| Folder | Description |
| - | - |
| ui | Blueprint UI files |
| ui/.dist | UI files compiled by the blueprint |
| src | Rust source code |
| target/release | Release build of the app |
| blueprint-compiler | Blueprint compiler |

## Run app

```sh
cargo run
```

## Build app

```sh
cargo build --release
```
