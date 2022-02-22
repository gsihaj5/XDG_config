if status is-interactive
    # Commands to run in interactive sessions can go here
	if test -z "$DISPLAY" -a $XDG_VTNR = 1
		exec startx ~/.config/X11/xinit/xinitrc -- -keeptty
	end
end



