#!/bin/bash

input_file="result.csv"

while read -r line; do
    path=$(echo "$line" | cut -f2)
    if ! ls "$path"; then
        echo "Path not found: $path"
    fi
done < "$input_file"