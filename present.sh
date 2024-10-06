#!/usr/bin/env bash

if ! command -v wezterm &>/dev/null; then
  echo "wezterm is not installed. Please install it first."
  exit 1
fi

if ! command -v presenterm &>/dev/null; then
  echo "installing presenterm..."
  cargo install --git https://github.com/mfontanini/presenterm
  exit 1
fi

git submodule update --init --recursive
wezterm --config-file .wezterm.lua start presenterm "$PWD"/presentation.md --config-file "$PWD"/config.yml -X -p &
