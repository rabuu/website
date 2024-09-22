#!/usr/bin/env sh

set -e

if [ -z "$DEPLOY_USER" ]; then
    echo 'Variable $DEPLOY_USER not set'
    exit 1
fi


echo "Deploy to $DEPLOY_USER@rbuurman.de\n"

scp -r index.html style.css favicon.ico assets common dev misc \
    "$DEPLOY_USER@rbuurman.de:rbuurman.de/httpdocs"
