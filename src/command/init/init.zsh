croque::preexec() {
  __croque_start="$EPOCHREALTIME"
}

croque::precmd() {
  __croque_exit_status="$?"
  __croque_jobs="$#jobstates"
  local end="$EPOCHREALTIME"
  __croque_duration="$(($end - ${__croque_start:-$end}))"
  unset __croque_start
}

croque::prompt() {
  croque prompt --exit-status="$__croque_exit_status" --jobs="$__croque_jobs" --duration="$__croque_duration" --width="$COLUMNS" zsh
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd croque::precmd
add-zsh-hook preexec croque::preexec

setopt prompt_subst
PROMPT='$(croque::prompt)'
