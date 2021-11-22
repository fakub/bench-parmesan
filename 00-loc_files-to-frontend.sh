#!/bin/bash

# copy to frontend
echo
echo -e ">>> Copy files to frontend (scripts & .profile)\n"

# copy scripts
scp \
    01-rem_compile-at-frontend.sh \
    02-rem_bench-parmesan_elwe_submit-via-qsub.sh \
    02-rem_bench-parmesan_kirke_submit-via-qsub.sh \
    02-rem_bench-parmesan_samson_submit-via-qsub.sh \
    skirit:~/parallel-arithmetics-benchmark/
#   xx_processor-info.sh

# copy .profile
scp \
    _profile-at-frontend \
    skirit:~/.profile

echo "(check that ALL keys are already copied: SK, BK, KSK)"
echo "(check branches in 01-rem_compile...sh)"
# possibly archive repos with   $ archive-repo <repo>
