#!/bin/sh

./compile.sh

# we don't care what the user says, we just want the script to give the user
# a chance to abort
echo "copy .wasm???"
read junk

if false # its broken and i don't care at this point
	# git stuff
	git add --all
	git commit -F commit.msg
	gh repo sync
fi
