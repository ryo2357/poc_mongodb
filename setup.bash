#!/bin/bash

current_dir=$(pwd)
project_name=$(basename $current_dir)

sed -i "s/template-rust-devcontainer/$project_name/g" Cargo.toml
sed -i "s/template-rust-devcontainer/$project_name/g" Dockerfile
sed -i "s/template-rust-devcontainer/$project_name/g" .devcontainer/devcontainer.json
touch .devcontainer/devcontainer.env

echo "Setup complete"
rm -- "$0"