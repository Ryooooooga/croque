croque::precmd() {
  local exit_status="$?"
  local jobs="$(jobs | wc -l)"
  local duration="0"
  PS1="$(croque prompt --exit-status="$exit_status" --jobs="$jobs" --duration="$duration" --width="$COLUMNS" --data.gh="" --data.glab="" bash)"
}

PROMPT_COMMAND=croque::precmd
