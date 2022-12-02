#!/bin/sh -e

cd /home/nonroot
exec npx wrangler dev --local --persist
