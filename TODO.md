# ✅ Zero-Shell TODO List

Track what’s done, what’s pending, and what’s next.

---

## 📍 Phase 1: Foundation

- [x] **Commit 1: Initial project setup**
  - [x] `cargo new zero-shell`
  - [x] Create `src/main.rs` with module declaration
  - [x] Create empty `src/shell.rs`, `src/commands/mod.rs`, `src/utils.rs`
  - [x] Update `Cargo.toml` (NO external dependencies!)
- [x] **Commit 2: Core shell loop and input handling**
  - [x] Define `Shell` struct in `src/shell.rs`
  - [x] Implement `run()` loop with stdin reading
  - [x] Handle Ctrl+D
- [x] **Commit 3: Command parsing and execution framework**
  - [x] Implement `parse_command()` in `src/utils.rs`
  - [x] Add `CommandExecutor` in `src/commands/mod.rs`
  - [x] Command storing with `HashMap`
  - [x] Handle unknown commands

---

## ⚙️ Phase 2: Commands

- [x] **Commit 4: echo**
- [x] **Commit 5: pwd + cd**
- [ ] **Commit 6: ls (basic)**
- [ ] **Commit 7: ls flags (-l, -a, -F)**
- [ ] **Commit 8: cat**
- [ ] **Commit 9: cp + mv**
- [ ] **Commit 10: mkdir + rm + touch**

---

## ✨ Phase 3: Features

- [ ] **Commit 11: Colorized output**
- [ ] **Commit 12: Enhanced prompt**
- [ ] **Commit 13: Command chaining (`;`)**
- [ ] **Commit 14: clear + help**

---

## 🛠️ Phase 4: Final touches

- [ ] **Commit 15: Error handling**
- [ ] **Commit 16: Final tests**
- [ ] **Commit 17: Documentation**
- [ ] **Commit 18: Optimizations**

---

## 🎯 Success

- [ ] All core commands implemented and working
- [ ] Unix-like behavior & error messages
- [ ] Clean, well-documented code
- [ ] Comprehensive test coverage
- [ ] No external dependencies
- [ ] Proper Git history with meaningful commits
