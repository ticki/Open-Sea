#!/bin/bash

if [ -z "$GH_TOKEN" ]; then
  echo "No token available for pushing the docs."
elif [ "$TRAVIS_REPO_SLUG" == "$RD_PUSH_REPO" ] && [ "$TRAVIS_PULL_REQUEST" == "false" ] && [ "$TRAVIS_BRANCH" == "$RD_PUSH_BRANCH" ]; then

  echo "Rust doc upload..."

  cargo doc --no-deps
  echo "<meta http-equiv=refresh content=0;url=$RD_INDEX_PAGE>" > target/doc/index.html
  pip install --user ghp-import
  ~/.local/bin/ghp-import -n target/doc
  git push -qf https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages

  echo "Docs uploaded."
else
  echo "Error: Couldn't upload docs"

fi

