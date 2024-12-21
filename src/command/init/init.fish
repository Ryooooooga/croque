function fish_prompt
  croq prompt --exit-status=$status --jobs=(count (jobs -p)) --duration=(math $CMD_DURATION / 1000) --width=$COLUMNS --data.gh="" --data.glab="" fish
end

function fish_right_prompt
  croq prompt --right --exit-status=$status --jobs=(count (jobs -p)) --duration=(math $CMD_DURATION / 1000) --width=$COLUMNS --data.gh="" --data.glab="" fish
end
