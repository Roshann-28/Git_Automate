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

Follow these steps to set up and use the tool:

1. **Create a repository on GitHub** (with any name you like).

2. **Initialize git locally** in your project folder:

   ```bash
   git init
   ```

3. **Add your remote origin** (replace with your actual GitHub repo URL):

   ```bash
   git remote add origin <your-repo-url>
   ```

4. **Build the project** in release mode:

   ```bash
   cargo build --release
   ```

   If there are any errors, they will show up in the terminal. Fix them before moving to the next step.

5. **Install the tool** once the build is error-free:

   ```bash
   cargo install --path .
   ```

6. **Run it** by typing the project/binary name in the terminal:
   ```bash
   name of the repo
   ```

That's it. It will add, commit, and push automatically.

## Status

This project is newly created. It works at a basic level, but it still has some rough edges
(e.g. hardcoded branch name, simple error handling, random commit messages).
These will be fixed and improved in future updates.
