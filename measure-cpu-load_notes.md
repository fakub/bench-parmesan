
# Observations

This is what I like the most:

    $ dstat --cpu-use -t --noheaders --cpufreq

But the problems are:
 
 1. interval shorter than 1s cannot be set (same holds for timestamp),
 2. there is a little peak in the beginning of the measurement, which makes it impossible to call dstat repeatedly each 100ms.
