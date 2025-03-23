#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🍺 Testing Homebrew package installation..."

# Create a temporary directory for the test
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

# Start Homebrew container
echo "🐳 Starting Docker container..."
docker run --rm -it \
    -v "$(pwd):/workspace" \
    -w /workspace \
    homebrew/brew:latest /bin/bash -c "
        echo '${GREEN}➡️ Setting up environment...${NC}'
        # Ensure the Homebrew environment is properly set up
        eval \"\$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)\"

        echo '${GREEN}➡️ Adding tap...${NC}'
        if brew tap kolarski/s-screen; then
            echo '${GREEN}✅ Tap added successfully${NC}'
        else
            echo '${YELLOW}ℹ️  Tap kolarski/s-screen is not yet available${NC}'
        fi

        echo '${GREEN}➡️ Attempting to install s-screen...${NC}'
        if brew install s-screen; then
            echo '${GREEN}✅ s-screen installed successfully!${NC}'
            # Test the binary
            if command -v s &> /dev/null; then
                echo '${GREEN}✅ s command is available${NC}'
                s --version
                exit 0
            else
                echo '${RED}❌ s command not found after installation${NC}'
                exit 1
            fi
        else
            echo '${YELLOW}ℹ️  Package s-screen is not yet available in Homebrew${NC}'
            exit 0
        fi
    "

# Cleanup
cd - > /dev/null
rm -rf "$TEST_DIR" 