#!/bin/bash

# Navigate to the directory containing the Anchor project
cd "$(dirname "$0")" || exit

# Run the anchor deploy command with the --verifiable flag
anchor deploy --verifiable --provider.cluster devnet

# Check if the command was successful
if [ $? -eq 0 ]; then
    echo "Deployment completed successfully with verifiable build."
else
    echo "Deployment failed. Please check the logs for details."
    solana program close --buffers -u d
    if [ $? -eq 0 ]; then
        echo "Program closed successfully."
    else
        echo "Failed to close the program."
    fi
    exit 1
fi
