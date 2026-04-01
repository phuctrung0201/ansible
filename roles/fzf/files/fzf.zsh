# fzf shell integration
source <(fzf --zsh)

# Fuzzy git
fgit() {
  local cmd
  cmd=$(printf "branch\nlog\nstash\ndiff\nfetch" | \
    fzf --prompt=" git > " --layout=reverse --height=40%) || return
  case "$cmd" in
    branch)
      local branch
      branch=$(git branch | fzf --prompt="Branch > " --layout=reverse --height=40%) || return
      git checkout "${branch//\*/}"
      ;;
    log)
      git log --oneline | \
        fzf --prompt="Commit > " \
            --layout=reverse \
            --height=60% \
            --preview='git show --stat --color=always {1}'
      ;;
    stash)
      local stash
      stash=$(git stash list | fzf --prompt="Stash > " --layout=reverse --height=40%) || return
      git stash apply "${stash%%:*}"
      ;;
    diff)
      git diff --name-only | \
        fzf --prompt="Diff > " \
            --layout=reverse \
            --height=60% \
            --preview='git diff --color=always {}'
      ;;
    fetch)
      local remote branch
      remote=$(git remote | fzf --prompt="Remote > " --layout=reverse --height=40%) || return
      branch=$(git ls-remote --heads "$remote" | awk '{print $2}' | sed 's|refs/heads/||' | \
        fzf --prompt="Branch > " --layout=reverse --height=40%) || return
      git fetch "$remote" "$branch"
      ;;
  esac
}

# Fuzzy kube
fkube() {
  local cmd
  cmd=$(printf "context\nnamespace\npods\nlogs\nconfigmap\ndeployment\nsecret" | \
    fzf --prompt="󱃾 kube > " --layout=reverse --height=40%) || return
  case "$cmd" in
    context)
      local current selected
      current=$(kubectl config current-context 2>/dev/null)
      selected=$(kubectl config get-contexts -o name | \
        fzf --prompt="Context > " \
            --layout=reverse \
            --height=40% \
            --header="current: ${current}") || return
      kubectl config use-context "$selected"
      ;;
    namespace)
      local current selected
      current=$(kubectl config view --minify --output 'jsonpath={..namespace}' 2>/dev/null)
      selected=$(kubectl get namespaces -o jsonpath='{.items[*].metadata.name}' | tr ' ' '\n' | \
        fzf --prompt="Namespace > " \
            --layout=reverse \
            --height=40% \
            --header="current: ${current:-default}") || return
      kubectl config set-context --current --namespace="$selected"
      ;;
    pods)
      local pod
      pod=$(kubectl get pods --all-namespaces | \
        fzf --prompt="Pod > " \
            --layout=reverse \
            --height=60% \
            --header-lines=1 \
            --preview='kubectl describe pod {2} -n {1}' \
            --preview-window=right:60%) || return
      ;;
    logs)
      local pod ns
      selected=$(kubectl get pods --all-namespaces | \
        fzf --prompt="Pod > " \
            --layout=reverse \
            --height=60% \
            --header-lines=1 \
            --preview='kubectl logs {2} -n {1} --tail=20' \
            --preview-window=right:60%) || return
      ns=$(echo "$selected" | awk '{print $1}')
      pod=$(echo "$selected" | awk '{print $2}')
      kubectl logs -f "$pod" -n "$ns"
      ;;
    configmap)
      local selected ns cm
      selected=$(kubectl get configmaps --all-namespaces | \
        fzf --prompt="ConfigMap > " \
            --layout=reverse \
            --height=60% \
            --header-lines=1 \
            --preview='kubectl get configmap {2} -n {1} -o yaml | bat --color=always --style=plain --language=yaml' \
            --preview-window=right:60%) || return
      ns=$(echo "$selected" | awk '{print $1}')
      cm=$(echo "$selected" | awk '{print $2}')
      kubectl get configmap "$cm" -n "$ns" -o yaml
      ;;
    deployment)
      local selected ns deploy
      selected=$(kubectl get deployments --all-namespaces | \
        fzf --prompt="Deployment > " \
            --layout=reverse \
            --height=60% \
            --header-lines=1 \
            --preview='kubectl describe deployment {2} -n {1}' \
            --preview-window=right:60%) || return
      ns=$(echo "$selected" | awk '{print $1}')
      deploy=$(echo "$selected" | awk '{print $2}')
      kubectl describe deployment "$deploy" -n "$ns"
      ;;
    secret)
      local selected ns secret
      selected=$(kubectl get secrets --all-namespaces | \
        fzf --prompt="Secret > " \
            --layout=reverse \
            --height=60% \
            --header-lines=1 \
            --preview='kubectl get secret {2} -n {1} -o yaml | bat --color=always --style=plain --language=yaml' \
            --preview-window=right:60%) || return
      ns=$(echo "$selected" | awk '{print $1}')
      secret=$(echo "$selected" | awk '{print $2}')
      kubectl get secret "$secret" -n "$ns" -o yaml
      ;;
  esac
}

# Highlight text in bold green
highlight() { printf '\033[1;32m%s\033[0m' "$*"; }

# Fuzzy nb note open
fnb() {
  local selected id
  selected=$(nb list --no-color --limit 0 | \
    fzf --prompt="  Note > " \
        --layout=reverse \
        --height=60% \
        --preview='nb show --print $(echo {} | awk "{print \$1}" | tr -d "[]") | bat --color=always --style=plain' \
        --preview-window=right:60%) || return
  id=$(echo "$selected" | awk '{print $1}' | tr -d '[]')
  nb edit "$id"
}

# Fuzzy batch open with entr
fbat() {
  local file
  file=$(find . -type f | \
    fzf --prompt="󰐄 open > " \
        --layout=reverse \
        --height=60% \
        --preview='file {} | grep -q text && bat --color=always --style=plain {} || echo "[binary file]"' \
        --preview-window=right:60%) || return
  echo "$file" | entr -c bat --style=plain "$file"
}

# Hint
fhelp() {
  echo "$(highlight fgit)   — fuzzy git (branch/log/stash/diff/fetch)"
  echo "$(highlight fkube)  — fuzzy kube (context/namespace/pods/logs/configmap/deployment/secret)"
  echo "$(highlight fnb)    — fuzzy nb note open"
  echo "$(highlight fbat)   — fuzzy select file and watch with entr"
  echo "$(highlight fhelp)  — show this help"
}

# Dracula theme
export FZF_DEFAULT_OPTS='
  --color=fg:#f8f8f2,bg:#282a36,hl:#bd93f9
  --color=fg+:#f8f8f2,bg+:#44475a,hl+:#bd93f9
  --color=info:#ffb86c,prompt:#50fa7b,pointer:#ff79c6
  --color=marker:#ff79c6,spinner:#ffb86c,header:#6272a4
'
