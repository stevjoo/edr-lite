# EDR-lite

EDR-lite is a lightweight **Windows endpoint detection tool** written in Rust.  
It detects suspicious **process execution behavior** using simple, explainable heuristics and produces structured security logs.

> This project is **host-based (endpoint detection)** and does **not** inspect network traffic.

---

## What It Detects
- Living-off-the-Land Binaries (e.g. `powershell.exe`, `cmd.exe`)
- Executables launched from suspicious paths (`Temp`, `Downloads`, `AppData`)
- Basic integrity signals via optional SHA-256 hashing

## What It Does NOT Do
- No packet sniffing
- No network IDS / firewall functionality
- No kernel drivers

---

## How It Works
1. Enumerates running processes on Windows
2. Applies rule-based behavioral checks
3. Assigns severity (Low / Medium / High)
4. Logs results in JSONL format for analysis

---

## Usage
```powershell
# One-time scan
cargo run -- --once

# Continuous monitoring (every 5 seconds)
cargo run -- --interval 5

# Logs are written to:
logs/edr-log.jsonl