# CONFIGURATION FOR PROGRAM LISTED IN THE MAIN DIRECTORY


## NVIM
to utilize this config 


### WINDOWS

if gcc needed install scoop
`Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Optional: Needed to run a remote script the first time`

`irm get.scoop.sh | iex`

`scoop install gcc`

for faster search 

`scoop install ripgrep`

for extra font install from

https://www.nerdfonts.com/font-downloads

make sure the alacritty config at 

user/AppData/Roaming/alacritty/alacritty.yml


1. install nvim 0.9^ from https://github.com/neovim/neovim/releases/tag/stable
1. pull this repo
1. create symlink from %AppData%/local/nvim <-> XDG_CONFIG/nvim
    to create symlink do 
    
    `mklink /D nvim <path to this clone>\nvim`

    while in $AppData%/local
1. install packer
    `git clone https://github.com/wbthomason/packer.nvim "$env:LOCALAPPDATA\nvim-data\site\pack\packer\start\packer.nvim"`

1. open nvim 
1. type `:PackerSync`

### using latex on windows
before using latex 
1. install perl https://strawberryperl.com/
1. install miktex https://miktex.org/download
1. after that run the command `latexmk` on your tex directory (it will prompt latexmk installation and install it !)

for pdf viewer
1. download muPDF
1. extract muPDF
1. add mupdf executable to path

