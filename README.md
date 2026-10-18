# Flashi

Flashi is a free desktop app for studying with active recall. You sort what you're learning into **subjects** and **decks**, quiz yourself in focused **study sessions**, and track which questions you keep getting wrong. Everything is stored in a local database on your computer: no account, and no internet connection after the first launch.

Flashi runs on Windows, macOS and Linux.

## Features

- **Subjects & decks**: put questions into decks and decks into subjects, so each topic has its own place.
- **Study sessions**: add decks from any subject to a session and answer the questions one at a time.
- **Several question types**:
  - *Multiple choice*: pick the correct answer from a set of options.
  - *Open ended*: write your answer from memory, then compare it to the expected answer.
  - *Ordered sequence*: put steps, events or stages in the right order.
- **Markdown, math & code**: write questions in Markdown, with KaTeX formulas and syntax-highlighted code blocks.
- **Images**: attach images to questions.
- **Metrics**: see accuracy per session and per question so you know what to practise more.
- **Import & export**: share decks or back them up as portable Parquet bundles, and import decks other people have made.

## Download

Prebuilt installers are attached to each [GitHub release](https://github.com/shaysingh818/flashi/releases/latest):

| Platform | Files |
| -------- | ----- |
| Windows  | `*-setup.exe`, `*.msi` |
| macOS    | `*.dmg` |
| Linux    | `*.AppImage`, `*.deb`, `*.rpm` |

## Tech stack

- **Frontend**: Vue 3, TypeScript, Vite, Tailwind CSS, Chart.js
- **Desktop shell / backend**: [Tauri 2](https://tauri.app/) (Rust)
- **Storage**: [DuckDB](https://duckdb.org/), with schema migrations managed by [Flyway](https://flywaydb.org/)

## Project structure

```
.
├── src/                 # Vue frontend (pages, components, API bindings to Tauri commands)
├── src-tauri/           # Rust backend and Tauri config
│   ├── src/             # Tauri commands, domain models and SQL queries
│   └── database/        # Flyway SQL migrations and the prebuilt flashi.db
└── site/                # Standalone landing / download page (see site/README.md)
```

On first launch, Flashi creates its data directory (`<data dir>/flashi`), downloads the prebuilt empty database from `src-tauri/database/build/flashi.db` on the `main` branch, and writes a default `settings.json`.

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) and npm
- [Rust](https://www.rust-lang.org/tools/install)
- The [Tauri system dependencies](https://tauri.app/start/prerequisites/) for your platform

### Run the app

```sh
npm install
npm run tauri dev
```

### Build installers

```sh
npm run tauri build
```

The bundles are written to `src-tauri/target/release/bundle/`.

### Publish a release

Bump the version in `src-tauri/tauri.conf.json`, then push a matching tag:

```sh
git tag v0.1.0
git push origin v0.1.0
```

The `Release` workflow runs the backend tests, builds installers for Windows, macOS (Apple Silicon and Intel) and Linux, and publishes them as a GitHub release once every platform has finished.

### Run backend tests

```sh
cd src-tauri
cargo test
```

Some tests download the database from GitHub, so they need network access.

### Database migrations

The schema lives in `src-tauri/database/sql/` as Flyway migrations. The `Makefile` in that directory installs Flyway and the DuckDB JDBC driver and runs migrations:

```sh
cd src-tauri/database
make flyway-install   # one-time setup (Linux)
make migrate          # apply migrations to build/flashi.db
```

Update `flyway.url` in `flyway.conf` so it points at your local copy of `build/flashi.db`.

### Landing site

The marketing / download page is a separate Vite project in `site/`:

```sh
npm run site:dev
npm run site:build
```

See [`site/README.md`](site/README.md) for details.

## Recommended IDE setup

[VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
