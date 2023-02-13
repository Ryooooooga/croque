function fish_prompt
  croque prompt --exit-status=$status --jobs=(count (jobs -p)) --duration=(math $CMD_DURATION / 1000) --width=$COLUMNS fish
end
