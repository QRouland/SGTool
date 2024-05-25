#!/bin/sh

if ! command -v pb-rs &> /dev/null
then
    echo "Failed to find pb-rs command"
    echo "To install it run : cargo install pb-rs"
    exit 1
fi

pb-rs src/protos/stormgate.proto