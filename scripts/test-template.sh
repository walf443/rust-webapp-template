#!/bin/bash
set -euo pipefail

TEMPLATE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PROJECT_NAME="${1:-test-project}"
WORK_DIR="$TEMPLATE_DIR/target/tmp"

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

echo "==> Generating project from template..."
cargo generate --path "$TEMPLATE_DIR" --name "$PROJECT_NAME" --destination "$WORK_DIR" --silent

echo ""
echo "==> Running cargo test..."
cd "$WORK_DIR/$PROJECT_NAME"
cargo test

echo ""
echo "==> Template test passed!"
