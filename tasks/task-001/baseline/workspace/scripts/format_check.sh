set -euo pipefail
if grep -R -n $'\t' src --include='*.rs'; then
  echo "Rust task sources contain tab characters" >&2
  exit 1
fi
