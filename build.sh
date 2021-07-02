#!/bin/bash
# Build and bundle the necessary files for deploying to another machine.

set -eu
cd $(git rev-parse --show-toplevel)

OUTPUT_DIR=resource-aggregator
OUTPUT_PATH=/tmp/.resource-aggregator/$OUTPUT_DIR

echo 'Making temporary directory'
mkdir -p $OUTPUT_PATH

echo 'Compiling server'
cargo build --release
echo 'Copying server binary'
cp target/release/aggregator $OUTPUT_PATH/server

echo 'Copying database migrations'
cp -r migrations/ $OUTPUT_PATH/

echo 'Building front-end files'
cd public
ng build
echo 'Copying built front-end files'
cp -r dist/resources/ $OUTPUT_PATH/public/

echo 'Copying docker-compose.yml'
cd ..
cp docker-compose.yml $OUTPUT_PATH/

echo 'Bundling files'
cd $OUTPUT_PATH/..
tar czf $OUTPUT_DIR.tar.gz $OUTPUT_DIR
cd -

echo 'Moving bundle'
cp $OUTPUT_PATH.tar.gz .

echo 'Removing temporary directory'
rm -r $OUTPUT_PATH

echo
echo 'Done'
echo
