#!/bin/bash -e
DAY=1
# check if the first argument passed in exists
if [ -z "$1" ]; then
  DAY=$(date +%d)
else
  DAY=$1
fi

FOLDER_NAME=$(printf "day%d" $DAY)
echo "Creating folder $FOLDER_NAME"
cp -r template $FOLDER_NAME
#replace "REPLACE_PACKAGE_NAME_HERE" with the folder name in Cargo.toml file
sed -i '' "s/REPLACE_PACKAGE_NAME_HERE/$FOLDER_NAME/g" $FOLDER_NAME/Cargo.toml
sed -i '' "s/REPLACE_PACKAGE_NAME_HERE/$FOLDER_NAME/g" $FOLDER_NAME/Cargo.lock
echo "Done"
