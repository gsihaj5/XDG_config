# If you come from bash you might have to change your $PATH.
# export PATH=$HOME/bin:$HOME/.local/bin:/usr/local/bin:$PATH

export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="agnoster" # set by `omz`

plugins=( 
    git
    zsh-autosuggestions
    zsh-syntax-highlighting
)

source $ZSH/oh-my-zsh.sh

# Set-up icons for files/directories in terminal using lsd
alias ls='lsd'
alias l='ls -l'
alias la='ls -a'
alias lla='ls -la'
alias lt='ls --tree'

[[ -f "$HOME/XDG_config/zsh/lazy-nvm-conda.zsh" ]] && source "$HOME/XDG_config/zsh/lazy-nvm-conda.zsh"

export PATH="$PATH:/home/gerry/fzf/bin"

