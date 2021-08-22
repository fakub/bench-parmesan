#!/bin/bash

# copy to frontend
echo
echo -e ">>> Copy files to frontend\n"

# copy scripts
scp \
    02_compile-at-frontend.sh \
    03_bench-parmesan_samson_submit-via-qsub.sh \
    03_bench-parmesan_kirke_submit-via-qsub.sh \
    xx_processor-info.sh \
    skirit:~/parallel-arithmetics-benchmark/

# copy .profile
scp \
    _profile-at-frontend \
    skirit:~/.profile

echo "(check that ALL keys are already copied: SK, BK, KSK)"
echo "(check branches in 02_...sh -- in particular parmesan)"
# possibly archive repos with   $ archive-repo <repo>
