#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║      📦 Todo CLI Installer           ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

# Check if we have sudo
if [ "$EUID" -ne 0 ]; then
    echo -e "${YELLOW}🔑 Need sudo privileges to install...${NC}"
    sudo -v
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Sudo failed. Run with sudo or as root.${NC}"
        exit 1
    fi
fi

# Install wget if missing
if ! command -v wget &> /dev/null; then
    echo -e "${YELLOW}📥 Installing wget...${NC}"
    sudo apt update && sudo apt install -y wget
fi

# Download URL - USING YOUR ACTUAL GITHUB
GITHUB_USER="HonseGathGath"
REPO="todors"
VERSION="0.1.0"
URL="https://github.com/${GITHUB_USER}/${REPO}/releases/download/v${VERSION}/todo.deb"

echo -e "${YELLOW}⬇️  Downloading todo v${VERSION}...${NC}"
echo -e "${BLUE}   From: ${URL}${NC}"

if ! wget -q --show-progress "$URL" -O /tmp/todo.deb; then
    echo -e "${RED}❌ Download failed!${NC}"
    echo ""
    echo "Possible reasons:"
    echo "1. Check your internet connection"
    echo "2. Make sure the file exists:"
    echo "   ${URL}"
    echo "3. Visit: https://github.com/HonseGathGath/todors/releases"
    exit 1
fi

echo -e "${YELLOW}⚙️  Installing package...${NC}"
if sudo dpkg -i /tmp/todo.deb 2>/dev/null; then
    echo -e "${GREEN}✅ Installation successful!${NC}"
else
    echo -e "${YELLOW}⚠️  Fixing dependencies...${NC}"
    sudo apt install -f -y
fi

# Clean up
rm -f /tmp/todo.deb

# Verify installation
if command -v todo &> /dev/null; then
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║        🎉 INSTALLATION COMPLETE      ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}📋 Quick Start:${NC}"
    echo "  todo add \"Buy milk\""
    echo "  todo list"
    echo "  todo done 1"
    echo ""
    echo -e "${YELLOW}📖 Run 'todo --help' for all options${NC}"
    echo ""
    echo -e "${BLUE}🌍 Project: https://github.com/HonseGathGath/todors${NC}"
else
    echo -e "${RED}❌ Installation failed!${NC}"
    echo "Try manually:"
    echo "  wget ${URL}"
    echo "  sudo dpkg -i todo.deb"
    echo "  sudo apt install -f"
    exit 1
fi
