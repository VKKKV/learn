#!/usr/bin/env bash
set -euo pipefail

package_dir="${XDG_DATA_HOME:-$HOME/.local/share}/typst/packages/local/math-note/0.1.0"
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "$package_dir"
cp "$script_dir/template.typ" "$package_dir/lib.typ"

cat > "$package_dir/typst.toml" <<'EOF'
[package]
name = "math-note"
version = "0.1.0"
entrypoint = "lib.typ"
authors = ["Arch User"]
description = "Reusable math note template"
EOF

printf 'Installed @local/math-note:0.1.0 to %s\n' "$package_dir"
