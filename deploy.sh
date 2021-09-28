#!/bin/sh

./compile.sh

if false # its broken and i don't care at this point
	# git stuff
	git add --all
	git commit -F commit.msg
	gh repo sync
fi
