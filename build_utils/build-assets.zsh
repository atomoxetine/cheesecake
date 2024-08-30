#!/bin/zsh
mkdir -p ../dist/assets
cp -r ../assets/** ../dist/assets/
find ../dist/assets/**/*.js | while read fname; do
  echo "minifying $fname"
  pnpm uglifyjs $fname --compress --mangle -o $fname
done
