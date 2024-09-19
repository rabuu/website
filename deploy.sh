#!/usr/bin/env sh

set -e

if [ -z "$DEPLOY_USER" ]; then
    echo 'Variable $DEPLOY_USER not set'
    exit 1
fi


echo "Deploy to $DEPLOY_USER@rbuurman.de\n"

echo 'common...'
scp common/* $DEPLOY_USER@rbuurman.de:rbuurman.de/httpdocs

echo 'home...'
scp home/* $DEPLOY_USER@rbuurman.de:rbuurman.de/httpdocs

echo 'dev...'
scp dev/* $DEPLOY_USER@rbuurman.de:dev.rbuurman.de/httpdocs
