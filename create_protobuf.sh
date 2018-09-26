#!/usr/bin/env bash

# Install google protoc https://github.com/protocolbuffers/protobuf/releases/tag/v3.5.1
# Install rust plugin cargo install protobuf-codegen --vers 2.0.4

protoc blockchain.proto --proto_path ./proto --rust_out ./src/
