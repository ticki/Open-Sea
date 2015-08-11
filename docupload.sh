#!/bin/sh

[ $TRAVIS_BRANCH = master ] &&
[ $TRAVIS_PULL_REQUEST = false ] &&
cargo doc &&
echo "<meta http-equiv=refresh content=0;url=`echo $TRAVIS_REPO_SLUG | cut -d '/' -f 2`/index.html>" > target/doc/index.html &&
mv target/doc ./
git push -f https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
