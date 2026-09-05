# Lazy-load nvm (and common Node binaries) on first use.
export NVM_DIR="$HOME/.nvm"

__nvm_lazy_load() {
  unset -f nvm node npm npx pnpm pnpx __nvm_lazy_load
  [ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh"
  [ -s "$NVM_DIR/bash_completion" ] && . "$NVM_DIR/bash_completion"
}

nvm()  { __nvm_lazy_load; nvm "$@"; }
node() { __nvm_lazy_load; node "$@"; }
npm()  { __nvm_lazy_load; npm "$@"; }
npx()  { __nvm_lazy_load; npx "$@"; }
pnpm() { __nvm_lazy_load; pnpm "$@"; }
pnpx() { __nvm_lazy_load; pnpx "$@"; }

# Lazy-load conda on first `conda` invocation.
__conda_lazy_load() {
  unset -f conda __conda_lazy_load
  local __conda_setup="$('/home/gerry/miniconda3/bin/conda' 'shell.zsh' 'hook' 2> /dev/null)"
  if [ $? -eq 0 ]; then
    eval "$__conda_setup"
  elif [ -f "/home/gerry/miniconda3/etc/profile.d/conda.sh" ]; then
    . "/home/gerry/miniconda3/etc/profile.d/conda.sh"
  else
    export PATH="/home/gerry/miniconda3/bin:$PATH"
  fi
}

conda() {
  __conda_lazy_load
  conda "$@"
}
