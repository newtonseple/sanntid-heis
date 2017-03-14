#!/usr/bin/env bash
# Based on http://stackoverflow.com/questions/696839/how-do-i-write-a-bash-script-to-restart-a-process-if-it-dies
until ./manhattan; do
    echo "Manhattan crashed with exit code $?.  Respawning.." >&2
    sleep 1
done
