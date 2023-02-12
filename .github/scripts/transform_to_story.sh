#!/bin/bash

# title without any quotes etc.
PROCESSED_TITLE=$(cat $TITLE_FILE | tr -d '\n' | tr -d '\"')
MAIN_FILE_TITLE=$(date '+%Y-%m-%d')-$(echo "$PROCESSED_TITLE" | tr ' ' '_' | tr -d ':' | tr -d "'")
BLOGPOST_FILENAME=$MAIN_FILE_TITLE.md
IMAGE_FILENAME=$MAIN_FILE_TITLE.png

# download the image
wget -O assets/images/$IMAGE_FILENAME $(cat $IMAGE_URL_FILE | tr -d '\n')

# populate the new file with the template settings
cat .github/scripts/template.md > _posts/$BLOGPOST_FILENAME

# populate the correct title
cat _posts/$BLOGPOST_FILENAME | sed "s/###TITLE_HERE###/$PROCESSED_TITLE/" > TMP_FILE.md
mv TMP_FILE.md _posts/$BLOGPOST_FILENAME
rm TMP_FILE.md

# populate the correct category
CATEGORIES_LIST=$(cat $CATEGORY_FILE | tr -d '\n')
cat _posts/$BLOGPOST_FILENAME | sed "s/###CATEGORIES###/$CATEGORIES_LIST/" > TMP_FILE.md
mv TMP_FILE.md _posts/$BLOGPOST_FILENAME
rm TMP_FILE.md

# populate the correct image filename
cat _posts/$BLOGPOST_FILENAME | sed "s/###IMAGE_FILENAME###/$IMAGE_FILENAME/" > TMP_FILE.md
mv TMP_FILE.md _posts/$BLOGPOST_FILENAME
rm TMP_FILE.md

# add the generated text
cat $TEXT_FILE >> _posts/$BLOGPOST_FILENAME

echo "$MAIN_FILE_TITLE"
