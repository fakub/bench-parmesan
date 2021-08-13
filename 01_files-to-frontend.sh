#!/bin/bash

# copy to frontend
echo
echo ">>> Copy files to frontend"
scp \
    02_compile-bench-parmesan_submit-via-qsub.sh \
    03_copy-bench-parmesan_submit-via-qsub.sh \
    xx_processor-info.sh \
    skirit:~/parallel-arithmetics-benchmark/

echo "(check that ALL keys are already copied: SK, BK, KSK)"
echo "(check branches in 02_...sh -- in particular parmesan)"
# possibly archive repos with   $ archive-repo <repo>
