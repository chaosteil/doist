# Remote Installation Guide for Rust Applications

This guide documents the process for installing Rust applications (like `doist`) on remote Linux servers.

## Prerequisites Check

Before starting, check the remote system's architecture and OS:
```bash
ssh <server> "uname -m && cat /etc/os-release | head -2"
```

## Installation Methods

### Method 1: Cross-Platform Binary Copy (Fast but Limited)

**When to use:** When you have a compatible pre-built binary (same architecture/OS).

**Limitations:** macOS ARM binaries won't work on Linux x86_64 servers.

```bash
# Copy binary to remote server
scp target/release/<binary> <server>:~/

# Install on remote
ssh <server> "mkdir -p ~/.local/bin && mv ~/<binary> ~/.local/bin/ && chmod +x ~/.local/bin/<binary>"
```

### Method 2: Build from Source on Remote (Recommended)

**When to use:** Different architectures or when you want the latest version.

#### Step 1: Install Rust Toolchain
```bash
ssh <server> "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
```

#### Step 2: Install Build Dependencies
For Ubuntu/Debian:
```bash
ssh <server> "sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev git"
```

For RHEL/CentOS/Fedora:
```bash
ssh <server> "sudo yum install -y pkgconfig openssl-devel git"
```

#### Step 3: Clone and Build
```bash
# Clone repository
ssh <server> "git clone <repo-url> ~/<project-name>"

# Checkout correct branch (if needed)
ssh <server> "cd ~/<project-name> && git checkout <branch>"

# Build release version
ssh <server> "cd ~/<project-name> && source ~/.cargo/env && cargo build --release"
```

#### Step 4: Install Binary
```bash
ssh <server> "mkdir -p ~/.local/bin && cp ~/<project-name>/target/release/<binary> ~/.local/bin/"

# Verify installation
ssh <server> "~/.local/bin/<binary> --version"
```

#### Step 5: Update PATH
```bash
ssh <server> 'grep -q ".local/bin" ~/.bashrc || echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> ~/.bashrc'
```

## Example: Installing doist

Complete installation script for `doist`:
```bash
SERVER="your-server"

# Install Rust
ssh $SERVER "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"

# Install dependencies
ssh $SERVER "sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev git"

# Clone and build
ssh $SERVER "git clone https://github.com/robbarry/doist.git ~/doist-cli"
ssh $SERVER "cd ~/doist-cli && git checkout rob/patches && source ~/.cargo/env && cargo build --release"

# Install
ssh $SERVER "mkdir -p ~/.local/bin && cp ~/doist-cli/target/release/doist ~/.local/bin/"
ssh $SERVER 'grep -q ".local/bin" ~/.bashrc || echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> ~/.bashrc'

# Verify
ssh $SERVER "~/.local/bin/doist --version"
```

## Troubleshooting

### Common Issues

1. **Binary format error**: You're trying to run a binary compiled for a different architecture. Solution: Build from source on the target machine.

2. **Missing pkg-config**: Install with `apt install pkg-config` or `yum install pkgconfig`.

3. **OpenSSL development headers missing**: Install with `apt install libssl-dev` or `yum install openssl-devel`.

4. **Build timeout**: Large projects may take time to compile. Use longer timeout or build in background:
   ```bash
   ssh <server> "cd ~/project && nohup cargo build --release > build.log 2>&1 &"
   ```

### Checking Build Status
```bash
# Check if build completed
ssh <server> "ls -la ~/project/target/release/<binary>"

# Check build logs
ssh <server> "tail -f ~/project/build.log"
```

## Updating Remote Installations

```bash
# Pull latest changes and rebuild
ssh <server> "cd ~/project && git pull && source ~/.cargo/env && cargo build --release"

# Replace binary
ssh <server> "cp ~/project/target/release/<binary> ~/.local/bin/"
```

## Multiple Server Deployment

For deploying to multiple servers, create a script:
```bash
#!/bin/bash
SERVERS=("server1" "server2" "server3")
BINARY="doist"
REPO="https://github.com/robbarry/doist.git"

for SERVER in "${SERVERS[@]}"; do
    echo "Installing on $SERVER..."
    # Add installation commands here
done
```