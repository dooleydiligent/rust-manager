#!/bin/sh
# post_create.sh
echo "--- Running postCreateCommand script ---"
# Example command: create a dummy file in the workspace
touch /workspaces/rust-project/post_create_was_here.txt
echo "--- Created post_create_was_here.txt ---"
# Example command: install a simple utility (replace with actual project needs)
sudo apt-get update && apt-get install -y cowsay
echo "--- Installed cowsay utility ---"
echo "--- postCreateCommand script finished ---"
