#!/bin/bash

# Command to upgrade the IDL
anchor idl upgrade 2ZV48S4LYwusvaahmKSSkkqcYFDPTPJHJhyHHMVLHuY4 -f target/idl/blankon_contracts.json --provider.cluster devnet

# Check if the command was successful
if [ $? -eq 0 ]; then
    echo "IDL upgrade successful."
else
    echo "IDL upgrade failed."
    exit 1
fi
