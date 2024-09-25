#!/usr/bin/env sh

set -e

if [ -z "$DEPLOY_USER" ]; then
    echo 'Variable $DEPLOY_USER not set'
    exit 1
fi


echo "Deploy to $DEPLOY_USER@rbuurman.de\n"

scp -r \
    common.js common.css \
    index.html style.css favicon.ico \
    assets modules \
    projekte krimskrams \
    "$DEPLOY_USER@rbuurman.de:rbuurman.de/httpdocs"
