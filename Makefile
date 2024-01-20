CURRENT_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
GL_CUSTOM_NOBODY_CERT := $(CURRENT_DIR)/gl-certs/client.crt
GL_CUSTOM_NOBODY_KEY := $(CURRENT_DIR)/gl-certs/client-key.pem

all: build

build:
	GL_CUSTOM_NOBODY_CERT=$(GL_CUSTOM_NOBODY_CERT) GL_CUSTOM_NOBODY_KEY=$(GL_CUSTOM_NOBODY_KEY) cargo build

run:
	GL_CUSTOM_NOBODY_CERT=$(GL_CUSTOM_NOBODY_CERT) GL_CUSTOM_NOBODY_KEY=$(GL_CUSTOM_NOBODY_KEY) cargo run

clean:
	cargo clean

test:
	GL_CUSTOM_NOBODY_CERT=$(GL_CUSTOM_NOBODY_CERT) GL_CUSTOM_NOBODY_KEY=$(GL_CUSTOM_NOBODY_KEY) cargo test

fmt:
	cargo fmt

check:
	cargo check

.PHONY: all build run clean test fmt check
