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
- [docs] [https://doc.rust-lang.org/std/fs/struct.Metadata.html]
- [x] **Commit 6: ls (basic)**
- [x] **Commit 7: ls flags (-l, -a, -F)**
- [ ] **Commit 8: cat** [amine]
- [ ] **Commit 9: cp + mv** [amine]
- [x] **Commit 10: mkdir + rm + touch**

---

## ✨ Phase 3: Features

- [x] **Commit 11: Colorized output**
- [x] **Commit 12: Enhanced prompt**
- [x] **Commit 13: Command chaining (`;`)**
- [x] **Commit 14: clear + help**


- [ ] **ls permissions and types c l b (ls -laF /dev)**
- [ ] **ls -F (* / = / @)**
- [ ] **ls -l for files and directories -folder names** 
- [ ] **cp mv permission denied error** 
- [ ] **echo newlines**
- [ ] **'-' handeling** 
- [ ] **cd -**
- [ ] **home as ~**
- [ ] **'\' in touch and mkdir should escape**
- [ ] **ls quotes in case of special chars**
- [ ] **ls -l minor and major for disks**

---

## 🛠️ Phase 4: Final touches

- [x] **Commit 15: Error handling**
- [ ] **Commit 16: Final tests**
- [ ] **Commit 17: Documentation**
- [ ] **Commit 18: Optimizations**

---

## 🎯 Success

- [ ] All core commands implemented and working
- [ ] Unix-like behavior & error messages
- [ ] Clean, well-documented code
- [ ] Comprehensive test coverage
- [ ] Proper Git history with meaningful commits
