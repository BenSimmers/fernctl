# 🌿 Fernctl

> A terminal UI where your Docker containers are office workers — and they'll die if you don't water the plants.

```
╔══════════════════════════════════════════════════════════════════╗
║  🏢 Docker Office  —  4 containers  │  Tab cycle  ↑↓←→ move     ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                  ║
║  ┌──────┐       ┌──────┐       ┌──────┐       ┌──────┐          ║
║  │ ⌨  ▪ │       │ ⌨  ▪ │       │ ⌨  ▪ │       │ ⌨  ▪ │          ║
║                                                                  ║
║    ★                                                             ║
║   [SPACE]                                                        ║
║    o    💬 watering 🌿                                           ║
║   /|\                                                            ║
║   / \                                                            ║
║  nginx                                                           ║
║                                                                  ║
║    🌿          🌿          🥀          💀                        ║
║   ████████   ████████   ████░░░░   ░░░░░░░░                      ║
╠══════════════════════════════════════════════════════════════════╣
║  🟢 3 running  🟡 0 paused  🔴 1 exited  │  Plants: 💧 🌿 🥀 💀 ║
╚══════════════════════════════════════════════════════════════════╝
```

## What is this?

Fernctl is a gamified terminal Docker monitor. Your running containers appear as little ASCII office workers roaming around an office. Four plants are placed around the room. If the plants go unwatered for too long, **a container gets stopped** — for real.

Walk your container over to a plant and press **Space** to water it. Keep them all alive.

## Install

**macOS / Linux (auto-detect)**
```sh
curl -sL https://raw.githubusercontent.com/BenSimmers/fernctl/main/install.sh | bash
```

Or manually for your platform:

**macOS (Apple Silicon)**
```sh
curl -L https://github.com/BenSimmers/fernctl/releases/latest/download/fernctl-aarch64-apple-darwin -o fernctl
chmod +x fernctl && ./fernctl
```

**macOS (Intel)**
```sh
curl -L https://github.com/BenSimmers/fernctl/releases/latest/download/fernctl-x86_64-apple-darwin -o fernctl
chmod +x fernctl && ./fernctl
```

**Linux (x86_64)**
```sh
curl -L https://github.com/BenSimmers/fernctl/releases/latest/download/fernctl-x86_64-unknown-linux-gnu -o fernctl
chmod +x fernctl && ./fernctl
```

## Build from source

Requires Rust ([rustup.rs](https://rustup.rs)) and Docker.

```sh
git clone https://github.com/BenSimmers/fernctl
cd fernctl
make run
```

## Controls

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Cycle through containers |
| `↑` `↓` `←` `→` | Move the selected container |
| `Space` | Water the plant (when close enough) |
| `r` | Refresh container list |
| `q` | Quit |

## The game

- Four plants sit around the office, each with an independent thirst bar
- Thirst fills constantly while any containers are running
- Walk near a plant — a `[SPACE]` prompt appears when you're in range
- An unwatered plant kills a random running container via `docker stop`
- Other containers wander around on their own; only yours is player-controlled
- The status bar shows all plant states: `💧` just watered · `🌿` healthy · `🥀` thirsty · `💀` dying

## Requirements

- Docker running locally (`docker ps` must work)
- A terminal with UTF-8 and 256 colour support
- At least 80×30 terminal size recommended
