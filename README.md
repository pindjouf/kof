# Kof - Version controlled note management for the terminal

![Rust](https://img.shields.io/badge/language-Rust-blue?style=flat-square)
![Version](https://img.shields.io/badge/version-0.1.3-orange?style=flat-square)

Kof is a simple, terminal-based, Git-powered note management system. The core idea behind Kof is to help users reorganize their thoughts through writing. With `main.md` as your brain's RAM, Kof provides an efficient, streamlined way to manage notes via Git.

## Features

- **Create Notes** (`--create`): Use this command to create new notes.
- **Main Notes Management** (`--main`): This feature manages your `main.md` file, the centerpiece of your notes system.
- **Find Notes** (`--find`) [New Feature]: Easily search and locate specific notes in your repository.
  
## Usage

1. **Create a New Note**:
   ```bash
   kof --create
   ```
   This command allows you to start creating a new note file within your repository.

2. **Manage Your Main Notes**:
   ```bash
   kof --main
   ```
   The `main.md` file serves as the central hub for your notes, allowing you to organize key ideas, thoughts, and details in one place.

3. **Search for Notes**:
   ```bash
   kof --find
   ```
   A powerful search feature that lets you quickly locate a note from your repository by searching for keywords or patterns.

## Syncing Notes Between Machines

Coming soon: A feature allowing users to sync notes across devices. This will be achieved by wrapping Git functionality for file syncing, utilizing a `--sync` flag that allows self-hosting your note repository.

## Roadmap

- [x] Implement `--create` function for creating notes.
- [x] Implement `--main` function for managing the main note file.
- [x] Implement `--find` function for finding notes.
- [ ] Add `--sync` functionality for syncing notes between machines.

## Supporting the Project

If you'd like to support the development of Kof, consider donating via Ko-fi.

[![Ko-Fi](https://img.shields.io/badge/Donate-Ko--fi-red?style=flat-square)](https://ko-fi.com/pindjouf)
