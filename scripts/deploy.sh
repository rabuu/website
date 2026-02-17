#!/usr/bin/env sh

set -xe

user=$(pass netcup.de/hosting/8000/username)
password=$(pass netcup.de/hosting/8000/password)

echo "Deploy to $user@rbuurman.de\n"

sshpass -p "$password" scp -r \
    common.js common.css \
    index.html style.css favicon.ico \
    assets modules \
    projekte krimskrams \
    "$user@rbuurman.de:rbuurman.de/httpdocs"
