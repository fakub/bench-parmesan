#!/usr/bin/env ruby

LOGFILE = "cpu-stats.log"

CPU_LOAD = "cpu-load.log"
CPU_FREQ = "cpu-freq.log"
CPU_SYST = "cpu-systime.log"

ary = []

# skip first two lines
File.readlines(LOGFILE).drop(2).each do |ls|
    ary << ls.gsub(":", " ").gsub("|", " ").split.map{|n|n.include?(".") ? n.to_f : n.to_i}
end

File.open(CPU_LOAD, 'w') do |f_load|
File.open(CPU_FREQ, 'w') do |f_freq|
File.open(CPU_SYST, 'w') do |f_syst|
    is_load = true
    ary.transpose.each do |l|
        is_load = false if l.first.is_a? Float
        la = l.map do |n|
            n.is_a?(Integer) ? sprintf(" %3d", n) : sprintf(" %7.3f", n)
        end
        if l.first.is_a? Float
            f_syst.write la.join + "\n"
        elsif is_load
            f_load.write la.join + "\n"
        else
            f_freq.write la.join + "\n"
        end
    end
end
end
end
