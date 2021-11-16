# Amp: A Fork
This repo houses a fork of amp with a few small extra features (checked are
merged into jmacdonald's main):

+ [x] [`selection::justify`](https://github.com/lincolnauster/amp/tree/master)
  command to [reflow text](https://github.com/jmacdonald/amp/issues/219)

- [ ] `selection::justify` transparently handles indents.

+ [ ] If a non-existent file path is opened, [it's
  created](https://github.com/lincolnauster/amp/tree/fcreate).

+ [ ] line_length_guide can be configured differently for different file types

+ [ ] Percentage through the file is displayed in the modeline.

Some fixes:

+ [x] Adding a trailing newline no longer breaks on weird unicode.
- [ ] Text reflowing is treated as a single operation in the undo/redo chain

Some internal stuff:

+ [ ] Don't depend on the [`pad`](https://github.com/ogham/rust-pad) crate.

This all uses a [fork of scribe](https://github.com/lincolnauster/scribe) with
similarly incremental additions.

And some unstable hacks:

+ [ ] Nix flake packaging (unchecked not because I'm looking to merge but
     because I need to remind myself that most of what I did for this is, indeed,
     a very ugly hack).

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
