#!/bin/bash

# copy to frontend
echo
echo ">>> Copy files to frontend"
scp \
    02_compile-bench-parmesan_submit-via-qsub.sh \
    03_copy-bench-parmesan_submit-via-qsub.sh \
    xx_processor-info.sh \
    skirit:~/parallel-arithmetics-benchmark/

echo "(check that keys are already copied)"
echo "(maybe also repos: archive them with   archive-repo <repo>   )"
