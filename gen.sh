#!/bin/bash

git submodule update --init --recursive

rm -rf ./src/proto/
mkdir -p ./src/proto/

cp -r ./SecretNetwork/proto/* ./src/proto/
cp -r ./SecretNetwork/third_party/proto/* ./src/proto/

find ./src/proto -name '*.proto' |
    parallel -P 1 'dirname "{}"' |
    sort -u |
    parallel -P 1 $'protoc --rust_out "{}" --proto_path ./src/proto $(find "{}" -name \'*.proto\')'

find ./src/proto -name '*.proto' -delete

find ./src/proto -type d |
    parallel -P 1 $'find "{}" -type d -maxdepth 1 -mindepth 1 | xargs -n 1 basename | awk \'{print "pub mod "$1";"; print "pub use self::"$1"::*;\\n"}\' >> "{}"/mod.rs'