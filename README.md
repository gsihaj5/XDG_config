# CONFIGURATION FOR PROGRAM LISTED IN THE MAIN DIRECTORY


## NVIM on Windows

### Prerequisite

1. install scoop (run 1 by 1)

```
`Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Optional: Needed to run a remote script the first time`
`irm get.scoop.sh | iex`
```
2. install gcc -> 
`scoop install gcc`

3. install ripgrep ->
`scoop install ripgrep`

4. install alacritty (optional install whatever u want) ->
`scoop install alacritty`

5. download font from https://www.nerdfonts.com/font-downloads

6. make sure the alacritty config at user/AppData/Roaming/alacritty/alacritty.yml

### installing nvim and utilize this repo

1. install nvim 0.9^ from https://github.com/neovim/neovim/releases/tag/stable or `scoop install neovim`
1. pull this repo
1. create symlink from %AppData%/local/nvim to \<path to XDG_CONFIG>/nvim

    while in `$AppData%/local`

    to create symlink do `mklink /D nvim <path to this clone>\nvim`

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

