#!/bin/bash

set -o errexit -o nounset

if [ "$TRAVIS_BRANCH" != "master" ]
then
    echo "Not deploying. Commit made against branch: $TRAVIS_BRANCH."
    exit 0
fi

rev=$(git rev-parse --short HEAD)

cargo doc --no-deps
cd target/doc
ln -s tetrs/index.html index.html

git init
git config user.name = "Marc Tiehuis"
git config user.email "marctiehuis@gmail.com"

git remote add upstream "https://$GH_TOKEN@github.com/tiehuis/tetrs.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "[rebuild-pages] ${rev}"
git push -q upstream HEAD:gh-pages
