#!/usr/bin/env bash
# Builds Tasker-x86_64.AppImage from scratch.
# Needs appimagetool on PATH: https://github.com/AppImage/appimagetool/releases
set -euo pipefail          # stop on the first error
cd "$(dirname "$0")"       # run from the repo root

# 1. Compile the program.
cargo build --release

# 2. Assemble a fresh AppDir (the folder that becomes the AppImage).
rm -rf AppDir
mkdir -p AppDir/usr/bin
cp target/release/tasker AppDir/usr/bin/
cp packaging/tasker.desktop assets/tasker.png AppDir/
ln -s usr/bin/tasker AppDir/AppRun

# 3. Pack the AppDir into a single executable file.
ARCH=x86_64 appimagetool AppDir Tasker-x86_64.AppImage
