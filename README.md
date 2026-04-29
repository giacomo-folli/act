# Grind

A simple, file-based CLI todo manager written in Rust. Tasks persist in a human-readable TOML file.

## Overview

Grind stores tasks in `tasks.toml` in the current directory. Each task has a title, optional description, and one of three statuses: `todo`, `doing`, or `done`.

## Requirements

### Functional
- Initialize a new `tasks.toml` file
- Add tasks with title and optional description
- Edit task title/description
- Update task status (todo → doing → done)
- Delete tasks
- List tasks with filtering by status
- Show task details

### Non-Functional
- Single binary, no daemon
- Human-readable/editable TOML storage
- Clean terminal output with color coding
- Idempotent operations where possible
- No external runtime dependencies

## Data Model

```toml
# tasks.toml
[[task]]
id = "UHLaIonm"
title = "Implement add command"
description = "Support adding tasks via CLI"
status = "doing"
created_at = "2026-04-21T09:00:00Z"
updated_at = "2026-04-21T11:00:00Z"

[[task]]
id = "wVarLogf"
title = "Write tests"
status = "todo"
created_at = "2026-04-21T10:00:00Z"
updated_at = "2026-04-21T10:00:00Z"
```

## CLI Interface


- [x] grind init                          # Create tasks.toml
- [x] grind add "Title" -d "Description"  # Add task
- [x] grind list                          # List all tasks
- [x] grind list --status todo            # Filter by status
- [x] grind show <id>                     # Show task details
- [x] grind edit <id> -t "New Title"      # Edit task
- [x] grind status <id> doing             # Update status
- [x] grind delete <id>                   # Remove task


## Roadmap

### Milestone 1: Core (Week 1)
- [x] Project scaffolding (Cargo, CLI parsing with clap)
- [x] TOML serialization/deserialization (serde)
- [x] `init` and `add` commands
- [x] `list` with basic table output

### Milestone 2: CRUD (Week 2)
- [x] `show`, `edit`, `status`, `delete` commands
- [x] UUID generation for task IDs
- [x] Timestamps (created_at, updated_at)
- [x] Input validation

### Milestone 3: Polish (Week 3)
- [x] Status-based filtering
- [x] Error handling with anyhow/thiserror
- [x] Refactor filtering methods
- [x] Fix `save` ignoring errors

### Milestone 3: Polish (Week 3)
- [x] Global config file (~/.config/grind/)
- [ ] Implement codefixed specified in [CODE_REVIEW.md](CODE_REVIEW.md)

### Milestone 4: Nice-to-Haves (Backlog)
- [ ] Multiple storage files support
- [x] Quick add `grind add`
- [ ] Consider switching to `ratatui`
- [ ] Task priorities
- [ ] Due dates
- [ ] Export to JSON/CSV
- [ ] Tab Completion for Task Ids

## Tech Stack

| Component     | Crate            |
| ------------- | ---------------- |
| CLI parsing   | `clap`           |
| Serialization | `serde` + `toml` |
| IDs           | `uuid`           |
| Datetime      | `chrono`         |
| Colors        | `colored`        |
| Errors        | `anyhow`         |
| Tables        | `comfy-table`    |

## Project Structure

```
src/
├── main.rs       # CLI entry point
├── cli.rs        # Argument definitions (clap)
├── commands.rs   # Command handlers
├── storage.rs    # TOML read/write
├── errors.rs     # Custom errors
└── models.rs     # Task struct, Status enum
```

## Success Criteria

- [ ] All CRUD operations work via CLI
- [ ] `tasks.toml` is human-readable and editable
- [ ] List view is scannable with color-coded statuses
- [ ] No panics on invalid input
- [ ] Single `cargo install` deployment

## Mockup
![alt text](image.png)
