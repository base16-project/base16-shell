#!/usr/bin/env fish

# ----------------------------------------------------------------------
# Setup config variables and env
# ----------------------------------------------------------------------

# Allow users to optionally configure their tmux plugin path and set the
# value if one doesn't exist. This runs each time a script is switched
# so it's important to check for previously set values.

if test -z "$BASE16_SHELL_TMUXCONF_PATH"
  set -g BASE16_SHELL_TMUXCONF_PATH "$BASE16_CONFIG_PATH/tmux.base16.conf"
end

if test -z "$BASE16_TMUX_PLUGIN_PATH"
  set -g BASE16_TMUX_PLUGIN_PATH "$HOME/.tmux/plugins/base16-tmux"
end

# If base16-tmux path directory doesn't exist, stop hook
if not test -d $BASE16_TMUX_PLUGIN_PATH
  return 2
end

# ----------------------------------------------------------------------
# Execution
# ----------------------------------------------------------------------

# If base16-tmux is used, provide a file for base16-tmux to source
if test -d "$BASE16_TMUX_PLUGIN_PATH"
  echo "set -g @colors-base16 '$BASE16_THEME'" > \
    "$BASE16_SHELL_TMUXCONF_PATH"

  # Source tmux config
  tmux source-file $(tmux display-message -p "#{config_files}")
end
