# ATHN reference browser for the Gnome ecosystem
[![We don't want this number over 10K](https://tokei.rs/b1/github/itzgoldenleonard/reference-browser-gnome?category=code)](https://github.com/itzgoldenleonard/reference-browser-gnome)

gtk4-rs and libadwaita-rs based browser for the gnome ecosystem. Compatible with ATHN markup language v0.1.5 (alpha) and ATHN over https v0.1.1 (alpha), read more about project ATHN at <https://athn.online/>. You can even visit the ATHN site in this browser at <https://athn.online/index.athn>, try it out.

# Installation

## Linux

The primary installation method for linux is flatpak ([flathub link](https://flathub.org/apps/online.athn.browser.gnome)). Install in you graphical package manager or with this command

```sh
flatpak install online.athn.browser.gnome
```

This app is currently not packaged for any other linux distros, but a binary can be found in [releases](https://github.com/itzgoldenleonard/reference-browser-gnome/releases)


## Windows

Pre-built binaries for windows can be found in [releases](https://github.com/itzgoldenleonard/reference-browser-gnome/releases)

Make sure to also download the gschemas.compiled file and place it in C:/ProgramData/glib-2.0/schemas

If someone wants to package this app for chocolatey or another real windows package manager that would be great

## With crates.io (MacOS and others)

See [CONTRIBUTING.md](./CONTRIBUTING.md)

Pre-built binaries are only available for linux and windows, I dont plan on building binaries for other systems. It should still be compatible with MacOS and BSD, but you'll have to build it from source.


# Roadmap and current status

This browser is still in its early stages of development. The development is closely tied to project ATHN itself, see the roadmap at: <https://athn.online/software.html>. 

The master branch is not guaranteed to work properly or even compile, if you want a tested version use one of the release tags. 

Currently the browser is useable for everyday use, but very sparsely featured. There's no support for tabs, bookmarks, history, search engines or other things that you might take for granted in a web browser. But the core functionality of rendering pages and using forms is mostly there and mostly stable.

The goal is to eventually create a rock solid, fully featured and polished ATHN browser for gnome systems. But that will depend on project ATHN being rather stable, or someone motivated and skilled with rust and GTK spearheading the development. For now the focus is on testing out the newest ATHN specifications with a real world app, meaning that new features are prioritized over polish (although polish is still very welcome).

See [todo.md](./todo.md) for a more specific plan

# License

The license is CC0-1.0 because this codebase is meant to be reference material for others wanting to build software for project ATHN
