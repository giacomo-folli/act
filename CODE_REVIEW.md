## Recommended roadmap
- [x] Immediately fix `save` error handling.
- [x] Introduce `TaskNotFound` and consistent feedback.
- [x] Fix `edit` to support multiple updates.
- [x] Remove panic risk in ID slicing.
- [ ] Extract the service layer and add unit tests to the core.
- [x] Align README and the actual data model.
- [ ] Rust methods and robustness
- [ ] Inconsistent error types
- [x] Description default in `Task::new`
- [x] Destructive `init` on an existing file
- [ ] Time management can be improved


### 1) Ignored save errors
In several functions, `let _ = storage::save(&tasks);` is used.

**Risk**: If saving fails, the user receives no error and believes the operation was successful.

**Improvement**:
- Always propagate the error using `?`.
- Add context to the error (`anyhow::Context`) in the outer layers.

### 2) `edit` does not update both title and description in the same command
The `edit_task` function interrupts the flow (`break`) immediately after updating one of the two fields.

**Risk**: Unexpected behavior if the user passes `--title` and `--description` together.

**Improvement**:
- Apply both updates in the same match.
- Call `update_time()` only once if at least one field changes.

### 3) Silent operations when ID is not found
`status`, `edit`, and `delete` terminate successfully even if they don't find the task.

**Risk**: Ambiguous UX, difficult debugging.

**Improvement**:
- Introduce a domain error (`TaskNotFound`).
- Print explicit feedback to the user.

### 4) Possible panic on ID slicing
In the list rendering, `&task.id[..8]` is used.

**Risk**: panic if the TOML file contains a shorter ID (e.g., manual edit).

**Improvement**:
- Use `task.id.get(..8).unwrap_or(&task.id)` or a Unicode-safe approach with `chars().take(8)`.

---

## Architecture and design

### 5) Business logic too concentrated in `main.rs`
Currently, `main.rs` contains CLI parsing, orchestration, and CRUD logic.

**Improvement**:
- Introduce a service layer (`app.rs` / `service.rs`) with testable APIs.
- Leave only argument parsing and dispatching to `main.rs`.

### 6) Inconsistencies between README and implementation
The documentation is not perfectly aligned with the serialized data model.

**Improvement**:
- Align naming and schema (`state/status`, TOML keys, example formats).
- Also align the dependencies/ID generator section.

### 7) Time management can be improved
Timestamps are RFC2822 `String`s.

**Limitation**: Less robustness at compile-time and recurring parsing during output.

**Improvement**:
- Consider `DateTime<Utc>` in the model.
- Use RFC3339 and explicitly handle future dates in the relative format.

---

## Rust methods and robustness

### 8) Inconsistent error types
The code alternates between `anyhow::Result` and `Result<_, StateError>` without a clear separation between layers.

**Improvement**:
- Define a policy: typed errors in the core, `anyhow` at the CLI boundary.

### 9) Description default in `Task::new`
The description is initialized with `Some(String::new())`.

**Improvement**:
- Prefer `None` as the absence of a value.
- Show the description in the list only if present and not empty.

### 10) Destructive `init` on an existing file
`File::create` truncates the file if it is already present.

**Improvement**:
- Make `init` idempotent or make the behavior explicit with `--force`.
