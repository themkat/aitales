#!/bin/bash

# parses the text from the story post file (markdown)
# TODO: what is the best way to cut? now we just assume that all metadata is done at line 10
cat $1 | sed -n '10,$p'
