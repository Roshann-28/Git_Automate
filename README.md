# Git Automate

A small Rust tool that runs `git add`, `git commit`, and `git push` for you in one command.
Instead of typing a commit message yourself, it generates a random one automatically.

## Why

Sometimes you just want to save and push your changes fast, without thinking of a commit message.
This tool does that for you.

## How it works

1. Runs `git add -A` to stage all changes.
2. Generates a random commit message (using the `names` crate) and commits with it.
3. Runs `git push origin master` to push the changes.

## Requirements

- Rust installed
- Git installed
- A git repository with a remote named `origin`

## Usage

```bash
cargo run
```

That's it. It will add, commit, and push automatically.

## Status

This project is newly created. It works at a basic level, but it still has some rough edges
(e.g. hardcoded branch name, simple error handling, random commit messages).
These will be fixed and improved in future updates.
