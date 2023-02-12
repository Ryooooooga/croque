croque::preexec() {
  __croque_start="$EPOCHREALTIME"
}

croque::precmd() {
  local exit_status="$?"
  local jobs="$#jobstates"
  local end="$EPOCHREALTIME"
  local duration="$(($end - ${__croque_start:-$end}))"
  PROMPT="$(croque prompt --exit-status="$exit_status" --jobs="$jobs" --duration="$duration" zsh)"
  unset __croque_start
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd croque::precmd
add-zsh-hook preexec croque::preexec
