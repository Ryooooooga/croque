croque::prepare-async::callback-git() {
  __croque_git_info="$3"
  zle reset-prompt
}

croque::prepare-async::callback-gh() {
  __croque_gh_info="$3"
  zle reset-prompt
}

croque::prepare-async::callback-glab() {
  __croque_glab_info="$3"
  zle reset-prompt
}

croque::prepare-async() {
  local source="$1"
  local worker="croque_async_worker_$source"
  async_stop_worker "$worker"
  async_start_worker "$worker" -n
  async_register_callback "$worker" "croque::prepare-async::callback-$source"
  async_job "$worker" croque prepare "$source"
}

croque::prepare() {
  if (( ${+ASYNC_VERSION} )); then
    croque::prepare-async git
    (( ${+commands[gh]} )) && croque::prepare-async gh
    (( ${+commands[glab]} )) && croque::prepare-async glab
  else
    __croque_git_info="$(croque prepare git)"
    (( ${+commands[gh]} )) && __croque_gh_info="$(croque prepare gh)"
    (( ${+commands[glab]} )) && __croque_glab_info="$(croque prepare glab)"
  fi
}

croque::chpwd() {
  unset __croque_git_info
  unset __croque_gh_info
  unset __croque_glab_info
}

croque::preexec() {
  unset __croque_exit_status_overwrite
  __croque_start="$EPOCHREALTIME"
}

croque::precmd() {
  __croque_exit_status="${__croque_exit_status_overwrite:-$?}"
  __croque_jobs="$#jobstates"
  local end="$EPOCHREALTIME"
  __croque_duration="$(($end - ${__croque_start:-$end}))"
  unset __croque_start

  croque::prepare
}

croque::prompt() {
  croque prompt --exit-status="$__croque_exit_status" --jobs="$__croque_jobs" --duration="$__croque_duration" --width="$COLUMNS" --data.git="$__croque_git_info" --data.gh="$__croque_gh_info" --data.glab="$__croque_glab_info" zsh
}

croque::rprompt() {
  croque prompt --right --exit-status="$__croque_exit_status" --jobs="$__croque_jobs" --duration="$__croque_duration" --width="$COLUMNS" --data.git="$__croque_git_info" --data.gh="$__croque_gh_info" --data.glab="$__croque_glab_info" zsh
}

croque::clear-screen() {
  __croque_exit_status_overwrite=0
  croque::precmd
  zle .clear-screen
}

zle -N clear-screen croque::clear-screen

autoload -Uz add-zsh-hook
add-zsh-hook chpwd croque::chpwd
add-zsh-hook precmd croque::precmd
add-zsh-hook preexec croque::preexec

setopt prompt_subst
PROMPT='$(croque::prompt)'
RPROMPT='$(croque::rprompt)'
