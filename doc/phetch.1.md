PHETCH(1)

# NAME

phetch - quick lil gopher client

# SYNOPSIS

*phetch* [_OPTIONS_] [_URL_]

# OPTIONS

*-l*, *--local*
	Connect to the local gopher server at URL _127.0.0.1:7070_.

*-p* _URL_, *--print* _URL_
	Print a rendered gopher server response of _URL_ and exit.

*-r* _URL_, *--raw* _URL_
	Print the raw gopher server response of _URL_ and exit.

*-t*, *--tls*
	Attempt to fetch all pages over TLS.

*-h*, *--help*
	Print a help summary and exit.

*-v*, *--version*
	Print version information and exit.

# NOTES

When given a _URL_, *phetch* will show the requested gopher page and
enter interactive mode.

Without a _URL_, *phetch* will show a builtin dashboard with easy
access to online help, bookmarks and history, and enter interactive
mode.

# NAVIGATION

## KEYBOARD SHORTCUTS

All single letter commands also work with the *Ctrl* key: e.g., *h*
and *Ctrl-h* are synonyms.

*h*
	Go to builtin help page.
*q*
	Quit *phetch*.

*←*, *left arrow*
	Go back in history.
*→*, *right arrow*
	Go forward in history.
*↑*, *up arrow*, *p*, *k*
	Select previous link.
*↓*, *down arrow*, *n*, *j*
	Select next link.
*PgUp*, *PgDn*
	Scroll up or down by many lines.
*-*, *SPACE*
	Same as *PgUp* and *PgDn*.

*Number key*
	Open/select link.
*Enter*
	Open current link.
*Esc*, *Ctrl-c*
	Cancel

*f*, */*
	Find link in page.

*g*
	Go to gopher URL.
*u*
	Show gopher URL.
*y*
	Copy URL.

*b*
	Show bookmarks.
*s*
	Save bookmark.
*a*
	Show history.

*r*
	View raw source.
*w*
	Toggle wide mode.

## MENU NAVIGATION

Up and down arrows
	Use the up and down arrows, *j* and *k* keys, or *n* and *p*
	keys to select links. Phetch will scroll for you, or you can
	use page up and page down (or *-* and spacebar) to scroll by
	many lines at once.

Number keys
	If there are few enough menu items, pressing a number key will
	open a link. Otherwise, the first matching number will be
	selected. Use *Enter* to open the selected link.

Incremental search
	Press *f* or */* to activate search mode, then just start
	typing. Phetch will look for the first case-insensitive match
	and try to select it. Use arrow keys or *p*/*n* to cycle
	matches.

# BOOKMARKS

There are two ways to save the URL of the current page:

*y*
	Copy URL.
*s*
	Save bookmark.

Bookmarks will be saved to the file _~/.config/phetch/bookmarks.gph_ if
the directory _~/.config/phetch/_ exists.

*b*
	View saved bookmarks.

The clipboard function uses *pbcopy* on MacOS, and *xsel* _-sel clip_
on Linux.

# HISTORY

If you create a _history.gph_ file in _~/.config/phetch/_, each gopher
URL you open will be stored there.

New URLs are appended to the bottom, but loaded in reverse order, so
you'll see the most recently visited pages first when you press the
*a* key.

Feel free to edit your history file directly, or share it with your
friends!

# ABOUT

Phetch is maintained by dvkt, and released under the MIT license.

Phetch's gopher hole:
	_gopher://phkt.io/1/phetch_
Phetch's webpage:
	_https://github.com/dvkt/phetch_