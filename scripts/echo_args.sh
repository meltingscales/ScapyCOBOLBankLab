#!/bin/bash
# Harmless demo script that echoes arguments
echo "Arguments received:"
for arg in "$@"; do
    echo "  - $arg"
done
