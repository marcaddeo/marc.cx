#!/bin/bash
# This is a temporary script to start up the production server until I can fix releases

export MIX_ENV=prod
brunch build --production && mix phoenix.digest && killall beam && elixir --detached -S mix phoenix.server
unset MIX_ENV
