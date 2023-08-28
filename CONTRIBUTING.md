# Contributing

Contributions are always very welcome. This is my first time using rust and GTK, so if you're more knowledgeable on those subjects you could probably make this codebase much nicer. The [todo.md](./todo.md) file is where I'm keeping track of the development.

# Building from source
## 1. Installing C dependencies

This project needs the gtk4 and libadwaita C development headers, as well as a C compiler to build. A stable rust toolchain is of course also neccessary, see <https://rustup.rs>. OpenSSL is not required, it uses rustls. Here is how to install them:

**For fedora**

```sh
sudo dnf install gtk4-devel libadwaita-devel gcc pkg-config     # pkg-config is also needed to build
```

**For NixOS**

```sh
nix-shell  # run in the base of the project directory, this does not install rust yet
```

**For debian**

```sh
sudo apt install libadwaita-1-dev libgtk-4-dev build-essential
```

**For arch**

```sh
sudo pacman install -S libadwaita gtk4 base-devel
```

**MacOS**

```sh
brew install gtk4 libadwaita
```

**Windows**
Refer to the gtk4-rs book <https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html> for gtk4 installation and <https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html#windows> for libadwaita installation

## 2. Install glib settings schema

**On real dev machines**

This step wont be neccessary on NixOS in the future

```sh
mkdir -p ~/.local/share/glib-2.0/schemas
cp src/resources/online.athn.browser.gnome.gschema.xml ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

**On windows**

```cmd
mkdir C:/ProgramData/glib-2.0/schemas/
cp src/resources/online.athn.browser.gnome.gschema.xml C:/ProgramData/glib-2.0/schemas/
glib-compile-schemas C:/ProgramData/glib-2.0/schemas/
```

## 3.alt Install from crates.io

```sh
cargo install reference-browser-gnome
```

## 3. Clone this repository

```sh
git clone https://github.com/itzgoldenleonard/reference-browser-gnome
cd reference-browser-gnome
cargo run # Test that everything works
```
