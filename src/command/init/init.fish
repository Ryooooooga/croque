function fish_prompt
  croque prompt --exit-status=$status --jobs=(count (jobs -p)) --duration=(math $CMD_DURATION / 1000) --width=$COLUMNS --data.gh="" fish
end

function fish_right_prompt
  croque prompt --right --exit-status=$status --jobs=(count (jobs -p)) --duration=(math $CMD_DURATION / 1000) --width=$COLUMNS --data.gh="" fish
end
