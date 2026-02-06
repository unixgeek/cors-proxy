#!/bin/sh -x

curl -vvv -H 'accept-encoding: deflate' -H 'accept: application/json' -H 'origin: http://localhost' http://localhost:8787?url=https%3A%2F%2Fipinfo.io%2F
echo
