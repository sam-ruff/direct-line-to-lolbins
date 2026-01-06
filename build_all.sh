#!/bin/bash
set -e

cargo build --release --target x86_64-pc-windows-gnu -p safe-dll
cp target/x86_64-pc-windows-gnu/release/greeting.dll safe-greeting.dll

cargo build --release --target x86_64-pc-windows-gnu -p dodgy-dll
cp target/x86_64-pc-windows-gnu/release/greeting.dll dodgy-greeting.dll

cargo build --release --target x86_64-pc-windows-gnu -p loader
cp target/x86_64-pc-windows-gnu/release/loader.exe .
