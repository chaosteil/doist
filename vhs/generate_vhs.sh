#!/bin/sh
# All tape files are processed with VHS (https://github.com/charmbracelet/vhs) 
# into gifs
./fixtures_server.py & server=$!
vhs intro.tape
kill "$server"
