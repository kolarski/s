#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PACKAGE_NAME="s-screen"
AUR_HELPER="yay"  # You can change this to paru or other AUR helpers

echo -e "${GREEN}Starting AUR installation test in Docker container...${NC}"

# Build and run the Docker container
docker run -it --rm \
    -v "$PWD/..:/pkg" \
    -e PACKAGE_NAME="$PACKAGE_NAME" \
    -e AUR_HELPER="$AUR_HELPER" \
    archlinux bash -c '
        # Update system and install dependencies
        pacman -Syu --noconfirm base-devel git sudo

        # Create builder user
        useradd -m builder
        echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
        chown -R builder:builder /pkg

        # Switch to builder user
        su builder -c "
            # Install AUR helper
            cd /tmp
            git clone https://aur.archlinux.org/yay.git
            cd yay
            makepkg -si --noconfirm
            cd

            # Try to install the package from AUR
            echo -e \"\n${YELLOW}Attempting to install $PACKAGE_NAME from AUR...${NC}\"
            yay -S --noconfirm $PACKAGE_NAME

            # Verify installation
            echo -e \"\n${YELLOW}Verifying installation...${NC}\"
            if command -v s >/dev/null 2>&1; then
                echo -e \"${GREEN}Package installed successfully!${NC}\"
                s --version
            else
                echo -e \"${RED}Package installation failed!${NC}\"
                exit 1
            fi
        "
    '

echo -e "${GREEN}AUR installation test completed successfully!${NC}" 