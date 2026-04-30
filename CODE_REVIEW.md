## To implement
- [ ] Extract the service layer and add unit tests to the core.
- [ ] Inconsistent error types
- [ ] Time management can be improved

---

### Business logic too concentrated in `main.rs`
Currently, `main.rs` contains CLI parsing, orchestration, and CRUD logic.

Improvement:
- Introduce a service layer (`app.rs` / `service.rs`) with testable APIs.
- Leave only argument parsing and dispatching to `main.rs`.

### Time management can be improved
Timestamps are RFC2822 `String`s. -> Less robustness at compile-time and recurring parsing during output.

Improvement:
- Consider `DateTime<Utc>` in the model.
- Use RFC3339 and explicitly handle future dates in the relative format.

### Inconsistent error types
The code alternates between `anyhow::Result` and `Result<_, StateError>` without a clear separation between layers.

Improvement: Define a policy: typed errors in the core, `anyhow` at the CLI boundary.