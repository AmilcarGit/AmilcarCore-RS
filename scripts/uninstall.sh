#!/usr/bin/env sh
set -eu
INSTALL_ROOT="${CARGO_INSTALL_ROOT:-$HOME/.local}"
BIN="$INSTALL_ROOT/bin/amilcar"
if [ -f "$BIN" ]; then
  rm "$BIN"
  printf '%s\n' "Se eliminó $BIN"
else
  printf '%s\n' "No se encontró el ejecutable: $BIN"
fi
