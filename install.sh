#!/bin/sh

set -e

cargo build --release

LOCAL_BIN="$HOME/.local/bin"
SHELL_NAME=$(basename "$SHELL")

mkdir -p "$LOCAL_BIN"

install -Dm755 target/release/rpkg "$LOCAL_BIN/rpkg"

case "$SHELL_NAME" in
    fish)
        if command -v fish_add_path >/dev/null 2>&1; then
            fish -c "fish_add_path '$LOCAL_BIN'"
            echo "Added $LOCAL_BIN to fish PATH"
        else
            echo "fish_add_path not found"
        fi
        ;;

    bash)
        CONFIG="$HOME/.bashrc"
        touch "$CONFIG"

        if ! grep -Fxq 'export PATH="$HOME/.local/bin:$PATH"' "$CONFIG"; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$CONFIG"
        fi

        echo "Added $LOCAL_BIN to bash PATH"
        ;;

    zsh)
        CONFIG="$HOME/.zshrc"
        touch "$CONFIG"

        if ! grep -Fxq 'export PATH="$HOME/.local/bin:$PATH"' "$CONFIG"; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$CONFIG"
        fi

        echo "Added $LOCAL_BIN to zsh PATH"
        ;;

    *)
        echo "Unknown shell: $SHELL_NAME"
        echo "Add $LOCAL_BIN manually to your PATH"
        ;;
esac
export PATH="$LOCAL_BIN:$PATH"
rpkg update
