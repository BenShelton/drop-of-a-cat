#!/bin/bash
PATH=$PATH:/vercel/.cargo/bin

trunk build --config app/Trunk.toml --release
