#!/bin/bash

RUN_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

docker run -it -v $RUN_DIR:/source rust:1.28-stretch /bin/bash