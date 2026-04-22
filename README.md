<div align="center">

<img src="linux-ui/data/icons/dev.kromodo.app.svg" alt="KromoDo" width="96" />

# KromoDo

**A fast, native cross-platform To-Do app with a shared Rust core.**

![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)
![Status](https://img.shields.io/badge/status-pre--alpha-red)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey)

</div>

---

## About

KromoDo is a task management app inspired by [Planify](https://github.com/alainm23/planify) and the Todoist workflow, built from the ground up as a **native, local-first** application — no Electron, no web runtime, no telemetry.

Everything that is not platform-specific — data model, queries, filters, change events, and (eventually) sync — lives in a single Rust core library. Each platform gets a thin native UI on top: GTK4/libadwaita on Linux today, with macOS, Windows and mobile targets planned once the core matures.

## Screenshot

<!-- TODO: replace with an actual screenshot -->
<p align="center"><em>Screenshot coming soon.</em></p>

## Status

KromoDo is **pre-alpha**. Linux is the only supported platform at the moment. Expect the schema, the UI, and the API to change until the first tagged release.

**Roadmap:**

1. **Phase 1 — Feature parity with Planify** (in progress)
2. **Phase 2 — Cross-device sync** (planned)
3. **Phase 3 — macOS, Windows, and mobile ports** (planned)

## Features

Currently implemented:

- [x] Inbox, Today, Upcoming and Completed views
- [x] Four-level task priority (Low / Medium / High / Urgent)
- [x] Due dates with quick-set shortcuts (today, tomorrow, calendar picker)
- [x] Inline task editor with live updates
- [x] Right-click context menu
- [x] Task duplication
- [x] Local SQLite database (WAL mode, migrations)
- [x] Automatic light/dark mode following the system theme

Planned for Phase 1:

- [ ] Labels
- [ ] Projects and sections
- [ ] Sub-tasks
- [ ] Overdue tasks in the Today view
- [ ] Recurring tasks
- [ ] Reminders and desktop notifications
- [ ] Natural-language quick add (e.g. `Buy milk tomorrow !p2 #shopping`)
- [ ] Drag-and-drop reordering
- [ ] Search

## Tech stack

| Layer | Technology |
| --- | --- |
| Shared core | Rust 2024 edition, [rusqlite](https://crates.io/crates/rusqlite) (bundled SQLite), [chrono](https://crates.io/crates/chrono), [serde](https://crates.io/crates/serde) |
| Linux UI | [GTK4](https://www.gtk.org/), [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita), [relm4](https://relm4.org/) |

## Building from source

### Prerequisites

- **Rust 1.85 or newer** (for the 2024 edition). Install via [rustup](https://rustup.rs/).
- **GTK 4** and **libadwaita** development packages.

Install the system dependencies for your distribution:

```bash
# Debian / Ubuntu
sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev

# Fedora
sudo dnf install gcc pkgconf-pkg-config gtk4-devel libadwaita-devel

# Arch Linux
sudo pacman -S base-devel pkgconf gtk4 libadwaita
```

### Run

```bash
git clone https://github.com/jukromer/KromoDo.git
cd KromoDo
cargo run -p kromodo-linux --release
```

The SQLite database is created on first launch under your platform's user data directory (`~/.local/share/kromodo/tasks.db` on Linux).

## Project structure

```
KromoDo/
├── core/                # Shared, platform-agnostic Rust library
│   └── src/
│       ├── db/              # SQLite persistence
│       ├── models/          # Task, Priority, DueBucket
│       ├── events.rs        # Change-event bus (TaskCreated / Updated / Deleted)
│       ├── filter.rs        # TaskFilter (Inbox / Today / Upcoming / Completed)
│       └── migration.rs     # Schema migrations
└── linux-ui/            # GTK4/libadwaita frontend (relm4)
    ├── src/
    │   ├── app.rs           # Root component
    │   └── components/      # Sidebar, quick-add dialog, task row
    └── data/                # Desktop file, icons, AppStream metainfo
```

The `core` crate exposes `AppState` with a synchronous API and publishes `CoreEvent`s via a built-in `std::sync::mpsc`-based bus. Platform UIs subscribe to this bus and patch their views in place.

## License

KromoDo is licensed under the [MIT License](LICENSE).

## Acknowledgments

- [Planify](https://github.com/alainm23/planify) by Alain M. — conceptual and UX reference
- [relm4](https://relm4.org/), [GTK4](https://www.gtk.org/), and [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita) for the Linux UI stack
