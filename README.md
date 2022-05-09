# GTK ❤️ Rust - Example app

This repo contains example gtk4 + libadwaita + rust application

For UI declarations I use [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler)

## Useful links

- [Libadwaita documentation](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/index.html)
- [GTK4 documentation](https://docs.gtk.org/gtk4/index.html)
  - [GTK4 classes](https://docs.gtk.org/gtk4/index.html#classes)
- [Blueprint documentation](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/examples.html)
- [gtk-rs documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html)
- [gtk-rs examples](https://github.com/gtk-rs/gtk4-rs/tree/master/examples)

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
