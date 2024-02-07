#!/bin/sh

# Check if Go is installed
if ! [ -x "$(command -v go)" ]; then
    echo 'Error: Go is not installed.' >&2
    exit 1
fi
