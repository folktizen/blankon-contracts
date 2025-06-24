#!/bin/bash

# Command to upgrade the IDL
anchor idl upgrade AA9xjMbf543L5vHqTceDGsFFKRW1ZXdTC6T8f33ux6yf -f target/idl/blankon_contracts.json --provider.cluster devnet

# Check if the command was successful
if [ $? -eq 0 ]; then
    echo "IDL upgrade successful."
else
    echo "IDL upgrade failed."
    exit 1
fi
