#!/bin/bash

# Requiem Debug Script

echo "🔍 Requiem Debug Tool"
echo "===================="

case "$1" in
    "run")
        echo "Running in debug mode with logging..."
        RUST_LOG=debug RUST_BACKTRACE=1 cargo run
        ;;
    "check")
        echo "Checking for errors..."
        cargo check --all-targets
        ;;
    "test")
        echo "Running tests..."
        cargo test -- --nocapture
        ;;
    "clean")
        echo "Cleaning build artifacts..."
        cargo clean
        ;;
    "build")
        echo "Building debug version..."
        cargo build
        ;;
    "trace")
        echo "Running with full trace logging..."
        RUST_LOG=trace RUST_BACKTRACE=full cargo run
        ;;
    "mem")
        echo "Checking memory usage..."
        if command -v heaptrack &> /dev/null; then
            cargo build
            heaptrack ./target/debug/requiem
        else
            echo "heaptrack not found. Install with: sudo pacman -S heaptrack"
        fi
        ;;
    "perf")
        echo "Running performance profiler..."
        if command -v perf &> /dev/null; then
            cargo build --release
            perf record -g ./target/release/requiem
            perf report
        else
            echo "perf not found. Install with: sudo pacman -S perf"
        fi
        ;;
    *)
        echo "Usage: $0 {run|check|test|clean|build|trace|mem|perf}"
        echo ""
        echo "Commands:"
        echo "  run    - Run with debug logging"
        echo "  check  - Check code for errors"
        echo "  test   - Run tests"
        echo "  clean  - Clean build artifacts"
        echo "  build  - Build debug version"
        echo "  trace  - Run with trace-level logging"
        echo "  mem    - Memory profiling (requires heaptrack)"
        echo "  perf   - Performance profiling (requires perf)"
        exit 1
        ;;
esac
