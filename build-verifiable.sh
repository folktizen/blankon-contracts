#!/bin/bash

# Navigate to the script's directory
cd "$(dirname "$0")"

#!/bin/bash
# filepath: build-verifiable.sh
set -e  # Exit on any error

# Define error handler
trap 'echo "Error: Command failed at line $LINENO"' ERR

# Then your build commands
anchor build && \
anchor keys sync && \
anchor build && \
anchor build --verifiable
