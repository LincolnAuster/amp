[![Build Status](https://travis-ci.org/jmacdonald/amp.svg?branch=master)](https://travis-ci.org/jmacdonald/amp)

# An Amp Fork
This repo houses a fork of amp with a few small extra features (all of which are
merged into `all`):

+ [`selection::justify`](https://github.com/lincolnauster/amp/tree/master)
  command to [reflow text](https://github.com/jmacdonald/amp/issues/219)

+ If a non-existant file path is opened, [it's
  created](https://github.com/lincolnauster/amp/tree/fcreate).

This all uses a [fork of scribe](/lincolnauster/scribe) with similarly
incermental additions (at least, until the fork [gets
merged](https://github.com/jmacdonald/scribe/pull/19)).

---

# Amp: A text editor for your terminal.

Heavily inspired by Vi/Vim. Amp aims to take the core interaction model of Vim,
simplify it, and bundle in the essential features required for a modern text
editor.

Written with :heart: in [Rust](http://rust-lang.org).

Amp's internals (data structures, syntax highlighting, workspace management, etc.)
have been built as a separate crate: [scribe](https://github.com/jmacdonald/scribe).

For a full overview, along with documentation and installation instructions, visit [amp.rs](https://amp.rs).

![Amp screenshot](screenshot.png?raw=true "Amp")

# Installation

## Cargo

If you have Rust >= 1.38.0 installed:

```
cargo install amp
```

## Homebrew

You can install Amp using Homebrew:

```
brew install amp
```
