# Overview 
This repository is to store personal setup for these tools:
 - [oh-my-zsh](https://ohmyz.sh/)
 - [fzf](https://github.com/junegunn/fzf)
 - [ghostty](https://ghostty.org/)
 - [aerospace](https://github.com/nikitabobko/aerospace)
 - [LazyVim](https://www.lazyvim.org/)

# Runbook

To run the setup, the ansible need installing:

> brew install ansible

The all setup can be run with `ansible-runbook`:

> ansible-playbook main.yml

Each setup there is already a tag to run specific one:

> ansible-playbook main.yml --tags "oh-my-zsh"
