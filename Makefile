# Project directories
PYTHON_DIR := python_service
RUST_DIR := rust_service

# Python settings
PYTHON_VENV := $(PYTHON_DIR)/venv
PYTHON := $(PYTHON_VENV)/bin/python
PIP := $(PYTHON_VENV)/bin/pip

# Check operating system
ifeq ($(OS),Windows_NT)
    PYTHON := $(PYTHON_VENV)/Scripts/python
    PIP := $(PYTHON_VENV)/Scripts/pip
    RM := del /Q
    RMDIR := rmdir /S /Q
else
    RM := rm -f
    RMDIR := rm -rf
endif

.PHONY: all setup clean python-setup rust-setup generate-proto run-python-server run-python-client run-rust-server run-rust-client

# Default target
all: setup generate-proto

# Setup everything
setup: python-setup rust-setup

# Clean generated files
clean:
	$(RMDIR) $(PYTHON_VENV)
	cd $(RUST_DIR) && cargo clean
	$(RM) $(PYTHON_DIR)/message_pb2*.py

# Python setup
python-setup:
	python -m venv $(PYTHON_VENV)
	$(PIP) install --upgrade pip
	$(PIP) install grpcio grpcio-tools protobuf

# Rust setup
rust-setup:
	cd $(RUST_DIR) && cargo build

# Generate Protocol Buffer code
generate-proto:
	$(PYTHON) -m grpc_tools.protoc \
		-I./protos \
		--python_out=$(PYTHON_DIR) \
		--grpc_python_out=$(PYTHON_DIR) \
		./protos/message.proto

# Run servers and clients
run-python-server:
	cd $(PYTHON_DIR) && $(PYTHON) server.py

run-python-client:
	cd $(PYTHON_DIR) && $(PYTHON) client.py

run-rust-server:
	cd $(RUST_DIR) && cargo run --bin server

run-rust-client:
	cd $(RUST_DIR) && cargo run --bin client

# Help target
help:
	@echo "Available commands:"
	@echo "  make setup            : Install all dependencies"
	@echo "  make generate-proto   : Generate Protocol Buffer code"
	@echo "  make run-python-server: Run Python server"
	@echo "  make run-python-client: Run Python client"
	@echo "  make run-rust-server  : Run Rust server"
	@echo "  make run-rust-client  : Run Rust client"
	@echo "  make clean           : Remove generated files and virtual environments"