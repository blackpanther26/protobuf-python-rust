# Python-Rust gRPC Communication Example

This project demonstrates communication between Python and Rust services using Protocol Buffers (protobuf) and gRPC. It implements a simple user service that can be run in either language and communicated with from either language.

## Prerequisites

Before you begin, ensure you have the following installed:

- Python 3.7 or higher
- Rust (latest stable version)
- Cargo (comes with Rust)
- Protocol Buffers compiler (`protoc`)
- Make utility

### Installing Prerequisites

#### macOS

```bash
# Install Homebrew if you haven't already
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Python
brew install python

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Protocol Buffers
brew install protobuf

# Make should be pre-installed
```

#### Linux (Ubuntu/Debian)

```bash
# Update package list
sudo apt update

# Install Python
sudo apt install python3 python3-pip

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Protocol Buffers
sudo apt install protobuf-compiler

# Install Make
sudo apt install make
```

#### Windows

1. Install Python from [python.org](https://www.python.org/downloads/)
2. Install Rust from [rustup.rs](https://rustup.rs/)
3. Install Protocol Buffers:
   - Download the latest protoc release from [GitHub](https://github.com/protocolbuffers/protobuf/releases)
   - Add the binary to your PATH
4. Install Make:
   - Install [Chocolatey](https://chocolatey.org/)
   - Run: `choco install make`

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

2. Run the setup to install all dependencies:

```bash
make setup
```

3. Generate the Protocol Buffer code:

```bash
make generate-proto
```

## Project Structure

```
.
├── Makefile
├── protos/
│   └── message.proto
├── python_service/
│   ├── requirements.txt
│   ├── server.py
│   └── client.py
└── rust_service/
    ├── Cargo.toml
    ├── build.rs
    └── src/
        ├── main.rs
        └── client.rs
```

## Usage

You can run either the Python or Rust server and connect to it with either the Python or Rust client.

### Running the Server

For Python server:

```bash
make run-python-server
```

OR

For Rust server:

```bash
make run-rust-server
```

### Running the Client

In a new terminal, run either:

For Python client:

```bash
make run-python-client
```

OR

For Rust client:

```bash
make run-rust-client
```

## Available Make Commands

- `make setup` - Install all dependencies
- `make generate-proto` - Generate Protocol Buffer code
- `make run-python-server` - Run Python server
- `make run-python-client` - Run Python client
- `make run-rust-server` - Run Rust server
- `make run-rust-client` - Run Rust client
- `make clean` - Remove generated files and virtual environments

## Testing the Service

The service implements a simple user lookup system. By default, it will:

1. Successfully return user information for ID 1
2. Return an error for any other ID

Example successful response:

```
Found user: John Doe (ID: 1)
Email: john@example.com
Roles: user, admin
```

## Troubleshooting

1. If you get "command not found: protoc":

   - Make sure Protocol Buffers is installed correctly
   - Verify that protoc is in your PATH

2. If Python can't find grpcio:

   - Make sure you're in the virtual environment
   - Try reinstalling with: `make python-setup`

3. If Rust compilation fails:

   - Run `cargo clean` in the rust_service directory
   - Try rebuilding with: `make rust-setup`

4. If the server won't start:
   - Check if port 50051 is already in use
   - Try killing any existing server processes
