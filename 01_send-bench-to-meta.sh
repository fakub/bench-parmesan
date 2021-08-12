#!/bin/bash

echo -e "N.b.: must be executed next to git repos in:\n\n    concrete-lib    parmesan    bench-parmesan\n"

archive-repo concrete-lib
archive-repo parmesan
archive-repo bench-parmesan

scp \
    concrete-lib.tar.gz \
    parmesan.tar.gz \
    bench-parmesan.tar.gz \
    bench-parmesan/02_compile-parmesan_submit-via-qsub.sh \
    bench-parmesan/03_bench-parmesan_submit-via-qsub.sh \
    skirit:~/parallel-arithmetics-benchmark/
