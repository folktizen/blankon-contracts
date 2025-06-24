#!/bin/bash

# CI/CD Pipeline Script for Anchor Project

set -e

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting CI/CD pipeline...${NC}"

# Step 1: Check for required tools
echo -e "${GREEN}Checking for required tools...${NC}"
REQUIRED_TOOLS=("anchor" "solana" "npm" "cargo")
for tool in "${REQUIRED_TOOLS[@]}"; do
    if ! command -v $tool &> /dev/null; then
        echo -e "${RED}Error: $tool is not installed.${NC}"
        exit 1
    fi
done

# Step 2: Check for formatting
echo -e "${GREEN}Checking code formatting...${NC}"
if ! cargo fmt --all -- --check; then
    echo -e "${RED}Code formatting issues found.${NC}"
    exit 1
fi

# Step 3: Run Clippy for linting
echo -e "${GREEN}Running Clippy for linting...${NC}"
if ! cargo clippy --all-targets -- -D warnings; then
    echo -e "${RED}Clippy linting issues found.${NC}"
    exit 1
fi

# # Step 4: Check for security vulnerabilities
# echo -e "${GREEN}Checking for security vulnerabilities...${NC}"
# if ! cargo audit; then
#     echo -e "${RED}Security vulnerabilities found.${NC}"
#     exit 1
# fi

# Step 5: Build the project
echo -e "${GREEN}Building the project...${NC}"
if ! anchor build; then
    echo -e "${RED}Build failed.${NC}"
    exit 1
fi

# Step 6: Run unit tests
echo -e "${GREEN}Running unit tests...${NC}"
if ! anchor test --provider.cluster localnet; then
    echo -e "${RED}Unit tests failed.${NC}"
    exit 1
fi

echo -e "${GREEN}CI/CD pipeline completed successfully!${NC}"
