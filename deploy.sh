#!/bin/sh

./compile.sh

# git stuff
git add --all
git commit -F commit.msg
gh repo sync
