#!/bin/sh
set -e

cargo run -p llm-worker -- --once
