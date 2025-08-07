# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The Countdown Numbers game is a number-based puzzle game where players use six given numbers and basic arithmetic operations (addition, subtraction, multiplication, division) to reach a target three-digit number.

### Game Rules
* Six numbers are provided: n "large" numbers and (6-n) "small" numbers
* Large numbers are drawn from {25, 50, 75, 100}
* Small numbers are drawn from 1-10, inclusive
* Target number is any three-digit number from 101-999

### Implementation Goals
* Flexible equation model for potential game variations
* Brute force solver with permutation tracking
* Minimax solver for performance comparison

## Development Commands

### Build and Run
```bash
~/.cargo/bin/cargo build          # Build the project
~/.cargo/bin/cargo run            # Run the application
~/.cargo/bin/cargo build --release # Release build
```

### Testing
```bash
~/.cargo/bin/cargo test           # Run all tests
~/.cargo/bin/cargo test -- --nocapture # Run tests with output
```

### Code Quality
```bash
~/.cargo/bin/cargo check          # Fast compilation check
~/.cargo/bin/cargo clippy         # Linting
~/.cargo/bin/cargo fmt            # Code formatting
```

## Architecture Notes

This is a Rust project using Cargo 1.89.0 with Rust edition 2024. The project is in early development stage with minimal initial code in `src/main.rs`.

## Code Style
* Minimal comments - code should be self-documenting
* Follow Rust conventions and idioms