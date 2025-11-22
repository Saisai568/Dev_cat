# rust_generate_log

This small Rust program generates a large log file (default 1GB) similar to the provided Python script.

## Build

Open PowerShell in the project folder (`rust_generate_log`) and run:

```powershell
cargo build --release
```

## Run

Run the compiled binary (Release build) or use `cargo run --release`:

```powershell
# Using cargo
cargo run --release

# Or run the produced executable
.\target\release\rust_generate_log.exe
```

The program writes `large_test_log.log` in the current directory and prints progress to the console.

## Notes
- You can modify `TARGET_SIZE`, `FILENAME` and `BUFFER_LINES` in `src/main.rs` to change behavior.
- Building in release mode (`--release`) is recommended for performance.
