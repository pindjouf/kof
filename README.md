# Kof - A Git-Based Note-Taking Framework

Kof is a simple, terminal-based note-taking framework that leverages Git for version control. Designed for simplicity and ease of use, Kof allows you to keep your notes organized and versioned with minimal fuss.

## Features

- **Terminal-Based:** Manage your notes directly from the terminal.
- **Git Integration:** Notes are versioned and can be tracked using Git.
- **Simple:** simple -_-
- **Chronological:** Journal entries are created and named after the current day.
- **Extra Memory:** The `main.md`/`main.txt` file serves as your brain's RAM.

## Installation

1. **Download the Latest Release**\
Download the latest release archive from the [Releases](https://github.com/pindjouf/kof/releases) page.

2. **Extract the Archive**\
`tar -xzvf kof-v1.0.tar.gz`

3. **Run the Installer**

Navigate to the extracted directory and run the installer script:

```
cd kof/
./installer
```

The installer will set up the necessary file structure and create a configuration file for you.

## Configuration

The configuration file (~/.config/kof/config.txt) allows you to choose between Markdown (md) and plain text (txt) files for your notes. You can adjust this setting based on your preference.

## Usage

- **Create/access your daily note:** Use the `create` command to start/open a new note. The note will be opened in your default editor, with a timestamp for the current entry.

- **Main Entry:** Access the main note with the `main` command.

## Roadmap

### Planned Features

- [ ] **Sync Notes Between Computers:** Add a --sync flag to allow users to sync their notes to a server they can self-host.

- [ ] **Terminal User Interface (TUI):** Implement a TUI to create and manage notes with different categories.

## Contributing

Feel free to contribute to the project by opening issues or submitting pull requests. Your feedback and contributions are welcome!
