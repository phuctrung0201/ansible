# Overview
This repository is to store personal setup for these tools:
 - [oh-my-zsh](https://ohmyz.sh/)
 - [kitty](https://sw.kovidgoyal.net/kitty/)
 - [aerospace](https://github.com/nikitabobko/aerospace)
 - [LazyVim](https://www.lazyvim.org/)
 - [mise](https://mise.jdx.dev/)

# Runbook

To run the setup, ansible needs installing:

> brew install ansible

The full setup can be run with:

> ansible-playbook main.yml

Each role has a tag to run a specific one:

> ansible-playbook main.yml --tags "ohmyzsh"
