#!/bin/bash

set -euo pipefail

base_url='https://github.com/martypenner/css-to-go/raw/HEAD/vscode-extension'
base_urls="--baseContentUrl=$base_url --baseImagesUrl=$base_url"

move_artifacts() {
  mkdir -p dist/autocompletion-engine >/dev/null 2>&1 || true
  cp -rfv package.json LICENSE *.md media dist
  my_find --hidden --no-ignore --type f '(.node|index.js)' --exclude 'node_modules' --exclude 'target' '../node_modules/autocompletion-engine' --exec cp -fv {} dist/autocompletion-engine/
}

# Ubuntu has a different fd bin than others
my_find() {
  if $(which fd 2>/dev/null); then
    fd "$@"
  else
    fdfind "$@"
  fi
}

case "$1" in
package)
  move_artifacts
  cd dist
  sed -i 's/"main": ".*",/"main": ".\/main.js",/' package.json
  npx @vscode/vsce package "$base_urls"
  ;;
publish)
  cd dist
  if [ -z ${IS_PRERELEASE+x} ]; then
    npx @vscode/vsce publish "$base_urls"
  else
    npx @vscode/vsce publish --pre-release "$base_urls"
  fi
  ;;
*)
  echo "Invalid command: $1"
  exit 1
  ;;
esac
