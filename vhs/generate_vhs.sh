#!/bin/sh
# All tape files are processed with VHS (https://github.com/charmbracelet/vhs) 
# into gifs
set -e
./fixtures_server.py & server=$!
vhs intro.tape
kill "$server"
