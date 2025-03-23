#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting PKGBUILD test in Docker container...${NC}"

# Build and run the Docker container
docker run -it --rm \
    -v "$PWD/..:/pkg" \
    archlinux bash -c '
        # Update system and install dependencies
        pacman -Syu --noconfirm base-devel rust cargo screen
        
        # Create builder user
        useradd -m builder
        chown -R builder:builder /pkg
        
        # Switch to builder user and build package
        su builder -c "cd /pkg && makepkg -f"
    '

echo -e "${GREEN}PKGBUILD test completed successfully!${NC}" 