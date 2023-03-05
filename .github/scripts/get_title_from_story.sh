#!/bin/bash

# parses the title from the story post file (markdown)
cat $1 | grep 'title: ' | sed -E 's/title\:[ ]*\"(.*)\"/\1/'
