#!/bin/bash

# Replace  with ridge in cliff.toml
sed -i '' 's/<<github-repository-placeholder>>/https:\/\/github.com\/vainjoker\/ridge/g' cliff.toml

# Create remote repository and link it to local repository
gh repo create vainjoker/ridge --public

# Execute release-plz init
release-plz init

# Generate Codecov token and open settings page
codecov_url="https://app.codecov.io/github/vainjoker/ridge/settings/"
if command -v xdg-open &> /dev/null; then
    xdg-open "$codecov_url"
elif command -v open &> /dev/null; then
    open "$codecov_url"
else
    echo "Please visit $codecov_url to set up Codecov and copy the token to the github repository settings."
fi

# Link Mergify
mergify_url="https://dashboard.mergify.com/integrations/github?repository=vainjoker/ridge&login=vainjoker"
if command -v xdg-open &> /dev/null; then
    xdg-open "$mergify_url"
elif command -v open &> /dev/null; then
    open "$mergify_url"
else
    echo "Please visit $mergify_url to set up Mergify and active the workflow."
fi

# 删除 generate 文件夹
cd ..
rm -rf generate

echo "Setup process completed and generate folder has been removed."
