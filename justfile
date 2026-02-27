# ScapyCOBOLBankLab - Just Commands

# Start the vulnerable bank server
start-server:
    cd server && cargo run

# Start the client (copies scripts to /tmp first)
start-client:
    just setup
    uv run python client/client.py

# Setup - copy demo scripts to /tmp
setup:
    mkdir -p /tmp/banklab
    cp scripts/* /tmp/banklab/
    chmod +x /tmp/banklab/*.sh

# Cleanup demo files from /tmp
cleanup:
    rm -rf /tmp/banklab

# Run the exploit demo
exploit:
    uv run python exploit_demo.py

# Install dependencies with uv
install:
    uv sync

# List default recipe
default:
    @just --list
