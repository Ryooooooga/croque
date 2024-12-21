croq::precmd() {
  local exit_status="$?"
  local jobs="$(jobs | wc -l)"
  local duration="0"
  PS1="$(croq prompt --exit-status="$exit_status" --jobs="$jobs" --duration="$duration" --width="$COLUMNS" --data.gh="" --data.glab="" bash)"
}

PROMPT_COMMAND=croq::precmd
