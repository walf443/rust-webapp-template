#!/bin/bash
set -euo pipefail

TEMPLATE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PROJECT_NAME="test-project"
WORK_DIR="$TEMPLATE_DIR/target/tmp"

COMMANDS=("${@:-test}")

cleanup() {
    if [ -d "$WORK_DIR/$PROJECT_NAME" ]; then
        echo "Cleaning up: $WORK_DIR/$PROJECT_NAME"
        rm -rf "$WORK_DIR/$PROJECT_NAME"
    fi
}
trap cleanup EXIT

mkdir -p "$WORK_DIR"

echo "==> Template: $TEMPLATE_DIR"
echo "==> Project name: $PROJECT_NAME"
echo "==> Output: $WORK_DIR/$PROJECT_NAME"
echo ""

if [ ! -d "$WORK_DIR/$PROJECT_NAME" ]; then
    echo "==> Generating project from template..."
    cargo generate --path "$TEMPLATE_DIR" --name "$PROJECT_NAME" --destination "$WORK_DIR" --silent
    echo ""
fi

cd "$WORK_DIR/$PROJECT_NAME"

for cmd in "${COMMANDS[@]}"; do
    echo "==> Running cargo $cmd..."
    case "$cmd" in
        fmt)
            cargo fmt --all -- --check
            ;;
        clippy)
            cargo clippy --all-targets -- -D warnings
            ;;
        *)
            cargo "$cmd"
            ;;
    esac
    echo ""
done

echo "==> All checks passed!"
