#!/usr/bin/env bash
set -euo pipefail

REPO="noldee/rnpkill-rs"
BIN_NAME="rnpkill-rs"
INSTALL_DIR="${RNPKILL_INSTALL_DIR:-$HOME/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)  os_tag="unknown-linux-gnu" ;;
  Darwin) os_tag="apple-darwin" ;;
  *) echo "Sistema operativo no soportado: $os" >&2; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64) arch_tag="x86_64" ;;
  arm64|aarch64) arch_tag="aarch64" ;;
  *) echo "Arquitectura no soportada: $arch" >&2; exit 1 ;;
esac

target="${arch_tag}-${os_tag}"

# macOS x86_64 no tiene build aarch64 nativo listado si no lo agregaste al matrix,
# ajusta aquí si sumas más targets.
asset="${BIN_NAME}-${target}.tar.gz"
url="https://github.com/${REPO}/releases/latest/download/${asset}"

echo "⬇️  Descargando ${asset}..."
tmp_dir="$(mktemp -d)"
curl -fsSL "$url" -o "${tmp_dir}/${asset}"

echo "📦 Extrayendo..."
tar xzf "${tmp_dir}/${asset}" -C "${tmp_dir}"

mkdir -p "$INSTALL_DIR"
mv "${tmp_dir}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"
rm -rf "${tmp_dir}"

echo "✅ rnpkill-rs instalado en ${INSTALL_DIR}/${BIN_NAME}"

if ! command -v "${BIN_NAME}" >/dev/null 2>&1; then
  echo ""
  echo "⚠️  ${INSTALL_DIR} no está en tu PATH. Agrega esto a tu ~/.bashrc o ~/.zshrc:"
  echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
fi