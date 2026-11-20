#!/usr/bin/env sh
set -eu

REPO_URL="https://github.com/AmilcarGit/AmilcarCore-RS.git"
INSTALL_ROOT="${CARGO_INSTALL_ROOT:-$HOME/.local}"

say() { printf '%s\n' "$*"; }
fail() { say "Error: $*" >&2; exit 1; }

command -v rustc >/dev/null 2>&1 || fail "Rust no está instalado. Consulta docs/install.html para instalarlo en Termux o Linux."
command -v cargo >/dev/null 2>&1 || fail "Cargo no está instalado. Consulta docs/install.html para instalarlo en Termux o Linux."

RUST_VERSION="$(rustc --version | awk '{print $2}')"
RUST_MAJOR="$(printf '%s' "$RUST_VERSION" | cut -d. -f1)"
RUST_MINOR="$(printf '%s' "$RUST_VERSION" | cut -d. -f2)"
if [ "$RUST_MAJOR" -lt 1 ] || { [ "$RUST_MAJOR" -eq 1 ] && [ "$RUST_MINOR" -lt 75 ]; }; then
  fail "Se requiere Rust 1.75 o superior; versión detectada: $RUST_VERSION. Actualiza con: rustup update stable"
fi

if [ -f Cargo.toml ] && grep -q 'members' Cargo.toml 2>/dev/null; then
  PROJECT_DIR="$(pwd)"
else
  PROJECT_DIR="${AMILCAR_CORE_DIR:-}"
  if [ -z "$PROJECT_DIR" ]; then
    PROJECT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/amilcarcore.XXXXXX")"
    command -v git >/dev/null 2>&1 || fail "Git no está instalado. Instálalo y vuelve a ejecutar el instalador."
    say "Descargando AmilcarCore-RS..."
    git clone --depth 1 "$REPO_URL" "$PROJECT_DIR"
  fi
fi

[ -f "$PROJECT_DIR/Cargo.toml" ] || fail "No se encontró el workspace Cargo.toml en: $PROJECT_DIR"
say "Compilando e instalando AmilcarCore-RS desde: $PROJECT_DIR"
(cd "$PROJECT_DIR" && cargo build --release --package amilcar-cli)

BIN_DIR="$INSTALL_ROOT/bin"
mkdir -p "$BIN_DIR"
cp "$PROJECT_DIR/target/release/amilcar" "$BIN_DIR/amilcar"
chmod 755 "$BIN_DIR/amilcar"

case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) say ""; say "Añade esta ruta a tu PATH para usar 'amilcar' desde cualquier carpeta:"; say "  export PATH=\"$BIN_DIR:\$PATH\"";;
esac

if [ -x "$BIN_DIR/amilcar" ]; then
  say ""
  say "Instalación completada."
  "$BIN_DIR/amilcar" --version
  say "Prueba: $BIN_DIR/amilcar info"
else
  fail "Cargo terminó, pero no se encontró el ejecutable en $BIN_DIR."
fi
