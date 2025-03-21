#!/bin/bash
set -e

# Colors for output
GREEN="\033[0;32m"
BLUE="\033[0;34m"
RED="\033[0;31m"
RESET="\033[0m"

# GitHub repository info
REPO_OWNER="kolarski"
REPO_NAME="s"

# Determine install path (prefer /usr/local/bin if writable, else $HOME/.local/bin)
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    # Create the directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
fi

# Ensure $INSTALL_DIR is in PATH
case ":$PATH:" in
    *":$INSTALL_DIR:"*) : ;; # Already in PATH
    *)
        echo -e "${BLUE}Adding $INSTALL_DIR to your PATH in your profile...${RESET}"
        PROFILE_FILE=""
        for file in ~/.bash_profile ~/.profile ~/.bashrc ~/.zshrc; do
            if [ -f "$file" ]; then
                PROFILE_FILE="$file"
                break
            fi
        done
        if [ -n "$PROFILE_FILE" ]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$PROFILE_FILE"
            echo -e "${BLUE}Added $INSTALL_DIR to your PATH in $PROFILE_FILE${RESET}"
            echo -e "${BLUE}Run 'source $PROFILE_FILE' to update your current session${RESET}"
        else
            echo -e "${RED}Could not find a profile file to update PATH. Please add $INSTALL_DIR to your PATH manually.${RESET}"
        fi
        ;;
esac

# Check if s is already installed
CURRENT_VERSION=""
if command -v s &> /dev/null; then
    CURRENT_VERSION=$(s --version | awk '{print $3}')
    echo -e "${BLUE}Current s version: ${GREEN}$CURRENT_VERSION${RESET}"
fi

# Get latest release info from GitHub API
echo -e "${BLUE}Fetching latest release information...${RESET}"
API_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
LATEST_RELEASE_INFO=$(curl -s $API_URL)
LATEST_VERSION=$(echo $LATEST_RELEASE_INFO | grep -Po '"tag_name": "v\K[^"]*')

if [ -z "$LATEST_VERSION" ]; then
    echo -e "${RED}Failed to fetch latest version information.${RESET}"
    exit 1
fi

echo -e "${BLUE}Latest version: ${GREEN}$LATEST_VERSION${RESET}"

# Skip if already on latest version
if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ]; then
    echo -e "${GREEN}You already have the latest version of s installed!${RESET}"
    exit 0
fi

# Determine system architecture
OS=""
ARCH=$(uname -m)
case $(uname -s) in
    Linux*)  OS="linux" ;;
    Darwin*) OS="macos" ;;
    *)
        echo -e "${RED}Unsupported operating system: $(uname -s)${RESET}"
        exit 1
        ;;
esac

# Determine download URL
ASSET_NAME="s-$OS-amd64"
DOWNLOAD_URL=""
for url in $(echo "$LATEST_RELEASE_INFO" | grep -Po '"browser_download_url": "\K[^"]*'); do
    if [[ "$url" == *"$ASSET_NAME"* ]]; then
        DOWNLOAD_URL="$url"
        break
    fi
done

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}Could not find a suitable download for your system.${RESET}"
    exit 1
fi

# Download the binary
echo -e "${BLUE}Downloading s version $LATEST_VERSION...${RESET}"
TEMP_DIR=$(mktemp -d)
TEMP_FILE="$TEMP_DIR/s"
curl -L -s "$DOWNLOAD_URL" -o "$TEMP_FILE"

# Make it executable
chmod +x "$TEMP_FILE"

# Move it to the install directory
echo -e "${BLUE}Installing to $INSTALL_DIR/s...${RESET}"
mv "$TEMP_FILE" "$INSTALL_DIR/s"

# Clean up
rm -rf "$TEMP_DIR"

echo -e "${GREEN}Successfully installed s version $LATEST_VERSION!${RESET}"
if [ -n "$CURRENT_VERSION" ]; then
    echo -e "${BLUE}Upgraded from version $CURRENT_VERSION to $LATEST_VERSION${RESET}"
fi

echo -e "${GREEN}You can now use s by running 'source $PROFILE_FILE' (if PATH was updated) or by starting a new terminal session.${RESET}" 