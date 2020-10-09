#!/bin/bash

set -euo pipefail

function main(){
    /usr/src/app/target/release/dark-sky-weather "$@"
}

main "$@"
