#!/usr/bin/env bash


set -e

HOST= # @TODO(you): add remote host for deploy target.
WEB_HOME=/home/web
NAME=\${PWD##*/}
TARGET_PATH=\$WEB_HOME/$app_name;format="normalize"$/

function run_on_remate {
    ssh root@\$HOST \$@
}
