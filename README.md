# Killswitch

A fast, cross-platform desktop app to view, search, and kill processes by port.

![Killswitch Screenshot](https://img.shields.io/badge/platforms-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/github/v/release/magicianjarden/killswitch)

## Features

- **Port Listing** - View all active network connections with PID, process name, port, protocol, state, and address
- **Real-time Search** - Filter by port number, process name, or PID instantly
- **Kill Processes** - Terminate processes with a single click (with confirmation)
- **Auto-refresh** - Toggle automatic refresh to monitor connections in real-time
- **Dark Theme** - Easy on the eyes with a minimal dark interface

## Installation

### Download

Grab the latest release for your platform:

| Platform | Download |
|----------|----------|
| macOS (Apple Silicon) | [Killswitch_x.x.x_aarch64.dmg](https://github.com/magicianjarden/killswitch/releases/latest) |
| macOS (Intel) | [Killswitch_x.x.x_x64.dmg](https://github.com/magicianjarden/killswitch/releases/latest) |
| Windows | [Killswitch_x.x.x_x64-setup.exe](https://github.com/magicianjarden/killswitch/releases/latest) |
| Linux (Debian/Ubuntu) | [Killswitch_x.x.x_amd64.deb](https://github.com/magicianjarden/killswitch/releases/latest) |
| Linux (Fedora/RHEL) | [Killswitch_x.x.x.x86_64.rpm](https://github.com/magicianjarden/killswitch/releases/latest) |
| Linux (Universal) | [Killswitch_x.x.x_amd64.AppImage](https://github.com/magicianjarden/killswitch/releases/latest) |

### macOS Note

The app is not notarized. After installation, remove the quarantine attribute:

```bash
xattr -cr /Applications/Killswitch.app
```

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [Rust](https://rustup.rs/) stable
- [Tauri CLI](https://tauri.app/)

### Setup

```bash
# Clone the repository
git clone https://github.com/magicianjarden/killswitch.git
cd killswitch

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Tech Stack

- **Frontend**: Svelte 5, TypeScript, Vite
- **Backend**: Rust, Tauri v2
- **Styling**: CSS (dark theme)

## How It Works

- **macOS**: Uses `lsof -i -P -n` to list network connections
- **Windows**: Uses `netstat -ano` combined with `tasklist` for process names
- **Kill**: Sends `kill -9` (macOS/Linux) or `taskkill /F` (Windows)

## License

MIT
