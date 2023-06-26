#!/bin/bash

function __update_version_in_file() {
    VERSION=$1
    FILE_TO_UPDATE=$2

    # pattern we look for:
    #       image: ghcr.io/themkat/aitales_story_generator:
    cat $FILE_TO_UPDATE | sed -E "s!(      image: ghcr.io/themkat/aitales_story_generator:).+\$!\1$VERSION!" > tmpfile.txt
    mv tmpfile.txt $FILE_TO_UPDATE
}

echo 'Wee! Updating versions!'
NEW_VERSION=$1
__update_version_in_file $NEW_VERSION .github/workflows/generate_story.yml
__update_version_in_file $NEW_VERSION .github/workflows/sequelize_story.yml
