#!/bin/bash
# MMLAB Scanner Bridge Launcher
DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

echo "🦀 Starting MMLAB Scanner Bridge Desktop App..."
"$DIR/target/release/hr-scanner-bridge"
