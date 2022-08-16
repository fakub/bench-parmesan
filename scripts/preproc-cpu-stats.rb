#!/usr/bin/env ruby

NCPUS               = 4

LOG_DSTAT           = "raw-cpu-stats-dstat.log"
LOG_TOP             = "raw-cpu-stats-top.log"

CPU_LOAD_ORD_DSTAT  = "cpu-load-ord-dstat.log"
CPU_LOAD_MAP_DSTAT  = "cpu-load-map-dstat.log"
CPU_FREQ_MAP_DSTAT  = "cpu-freq-map-dstat.log"
CPU_SYST_DSTAT      = "cpu-systime-dstat.log"

CPU_LOAD_TOP        = "cpu-load-top.log"


# ==============================================================================
#   dstat: extract load/freq & transpose
#
if File.exists? LOG_DSTAT
    ary = []

    # skip first two lines
    File.readlines(LOG_DSTAT).drop(2).each do |ls|
        # replace /missed [0-9]+ ticks/
        ls.gsub!(/missed [0-9]+ ticks/, "")
        # load as int/float-from-mm:ss.sss
        ary << ls.gsub("|", " ").split.map{|n|n.include?(".") ? \
            n.gsub(":", " ").split.map{|t|t.to_f}.zip([60, 1]).map{|p|p.reduce(:*)}.reduce(:+) : \
            n.to_i}
    end

    # write out: load, time, freq
    File.open(CPU_LOAD_ORD_DSTAT, 'w') do |f_load_ord|
    File.open(CPU_LOAD_MAP_DSTAT, 'w') do |f_load_map|
    File.open(CPU_FREQ_MAP_DSTAT, 'w') do |f_freq_map|
    File.open(CPU_SYST_DSTAT,     'w') do |f_syst|
        # write load & freq maps, systime
        is_load = true
        ts_index = 0
        begin
            ary.transpose.each.with_index do |l,i|
                is_load = false if l.first.is_a? Float
                ts_index = i    if l.first.is_a? Float
                la = l.map do |n|
                    n.is_a?(Integer) ? sprintf(" %3d", n) : sprintf(" %7.3f", n)
                end
                if l.first.is_a? Float
                    f_syst.write la.join + "\n"
                elsif is_load
                    f_load_map.write la.join + "\n"
                else
                    f_freq_map.write la.join + "\n"
                end
            end
        rescue
            raise "(!) File '#{LOG_DSTAT}' contains extra columns, exiting."
        end

        # write ordered load (for bar graph)
        ary.each do |l|
            la = [sprintf(" %8.3f", l[ts_index])]
            la.append *l[0..ts_index-1].sort.reverse.map{|n| sprintf(" %3d", n) }
            #~ puts la.join + "\n"
            f_load_ord.write la.join + "\n"
        end
    end
    end
    end
    end

    puts "(i) Pre-processed dstat log file: #{LOG_DSTAT}"
else
    puts "(i) Log file #{LOG_DSTAT} not found, not processing it."
end


# ==============================================================================
#   top: find groups of threads
#
if File.exists? LOG_TOP
    ary = []

    # read job PID from 2nd line: "# Job PID: 46365"
    jobPID = File.readlines(LOG_TOP)[1].split[3].to_i

    # prepare tmp array for individual threads & global ary
    th_loads = []
    ary = []
    max_thlen = 0

    # skip first two lines
    File.readlines(LOG_TOP).drop(2).each do |ls|
        # read to array
        la = ls.gsub(":", " ").gsub(",", ".").split
        if la[0] == "TS"
            # new block for non-triv array
            if th_loads.size >= 2
                # sort loads
                #~ th_loads[1..-1] = th_loads[1..-1].sort.reverse
                max_thlen = th_loads.size if th_loads.size > max_thlen
                ary << th_loads
            end
            th_loads = [la[2].to_f]
        else
            # only log runnable processes
            th_loads << la[8].to_f if la[7] == 'R'
        end
    end
    ary << th_loads if th_loads.size >= 2

    # write out
    File.open(CPU_LOAD_TOP, 'w') do |f_load|
        f_load.write "# max_threads = #{max_thlen-1}\n"   # there is extra timestamp

        #~ ary.transpose.each do |l|
        ary.each do |l|
            # fill with zeros
            l.fill(0.0, (l.size..max_thlen-1))
            f_load.write l.join(" ") + "\n"
        end
    end

    puts "(i) Pre-processed top log file: #{LOG_TOP}"
else
    puts "(i) Log file #{LOG_TOP} not found, not processing it."
end