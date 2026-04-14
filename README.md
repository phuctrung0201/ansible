# Overview
This repository is to store personal setup for these tools:
 - [oh-my-zsh](https://ohmyz.sh/)
 - [Ghostty](https://ghostty.org/)
 - [aerospace](https://github.com/nikitabobko/aerospace)
 - [Neovim](https://neovim.io/)
 - [mise](https://mise.jdx.dev/)

# Runbook

To run the setup, ansible needs installing:

> brew install ansible

The full setup can be run with:

> ansible-playbook main.yml

Each role has a tag to run a specific one:

> ansible-playbook main.yml --tags "ohmyzsh"



