#!/usr/bin/env python3
"""
ScapyCOBOLBankLab Client

Sends simulated bank data transfer commands to the server.
Commands are sent every 30 seconds, chosen at random.
"""

import socket
import random
import time
import sys

SERVER_HOST = "127.0.0.1"
SERVER_PORT = 9999

# Bank account names for simulation
ACCOUNTS = ["ALICE", "BOB", "CHARLIE", "DAVE", "EVE", "JACKY", "BECKY"]

# Demo file paths in /tmp
DEMO_FILES = [
    "/tmp/banklab/hello.sh",
    "/tmp/banklab/echo_args.sh",
    "/tmp/banklab/list_tmp.sh",
]

# Commands to send (some vulnerable to injection!)
COMMANDS = [
    # Normal bank data transfers
    "BANKDATA|XFER|{from_acct}|{to_acct}|0001000X|X",
    "BANKDATA|XFER|{from_acct}|{to_acct}|0000500X|X",
    "BANKDATA|XFER|{from_acct}|{to_acct}|0000200X|X",

    # CPY commands (vulnerable to injection!)
    "BANKCMD|CPY|X|X|{src_file}|/tmp/banklab/copy_{dst}.txt",

    # RUN commands (vulnerable to injection!)
    "BANKCMD|RUN|X|X|{script}|",
]


def format_command(template):
    """Fill in template with random values."""
    from_acct = random.choice(ACCOUNTS)
    to_acct = random.choice([a for a in ACCOUNTS if a != from_acct])

    result = template.format(
        from_acct=from_acct,
        to_acct=to_acct,
        src_file=random.choice(DEMO_FILES),
        dst=random.randint(1000, 9999),
        script=random.choice(DEMO_FILES),
    )
    return result


def send_command(sock, command):
    """Send a single command to the server."""
    try:
        sock.sendall((command + "\n").encode())
        print(f"[SENT] {command}")

        # Try to receive response
        response = sock.recv(4096).decode().strip()
        if response:
            print(f"[RECV] {response}")
        return True
    except Exception as e:
        print(f"[ERROR] Failed to send: {e}")
        return False


def main():
    print("[[ SCAPY COBOL BANK CLIENT ]]")
    print(f"Connecting to {SERVER_HOST}:{SERVER_PORT}")
    print("Press Ctrl+C to exit")
    print("=" * 40)

    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((SERVER_HOST, SERVER_PORT))
        print(f"[+] Connected to server!")

        while True:
            # Pick a random command template
            template = random.choice(COMMANDS)
            command = format_command(template)

            send_command(sock, command)
            print()

            # Wait 30 seconds before next command
            time.sleep(30)

    except KeyboardInterrupt:
        print("\n[*] Client shutting down...")
    except ConnectionRefusedError:
        print("[ERROR] Could not connect to server. Is it running?")
        print("Run: just start-server")
        sys.exit(1)
    finally:
        sock.close()


if __name__ == "__main__":
    main()
