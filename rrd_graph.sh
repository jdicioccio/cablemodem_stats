#!/bin/bash
for channel in $(seq 1 32); do
  for ds in power snr corr_e uncorr_e lock_st; do
    rrdtool graph -s -172800 -e -300 -t "Channel ${channel} - ${ds}" ds_${channel}_${ds}.png \
       DEF:var=inet.rrd:${channel}_ds_${ds}:AVERAGE \
       LINE1:var#FF0000 \
       VDEF:varavg=var,AVERAGE \
       VDEF:varmax=var,MAXIMUM \
       VDEF:varlast=var,LAST \
       GPRINT:varlast:" Current\:%8.2lf %s " \
       GPRINT:varavg:"Average\:%8.2lf %s" \
       GPRINT:varmax:"Maximum\:%8.2lf %s"
  done
done

for channel in $(seq 1 4); do
   for us in power; do
     rrdtool graph -s -172800 -e -300 -t "Channel ${channel} - ${us}" us_${channel}_${us}.png \
       DEF:var=inet.rrd:${channel}_us_${us}:AVERAGE \
       LINE1:var#FF0000 \
       VDEF:varavg=var,AVERAGE \
       VDEF:varmax=var,MAXIMUM \
       VDEF:varlast=var,LAST \
       GPRINT:varlast:" Current\:%8.2lf %s " \
       GPRINT:varavg:"Average\:%8.2lf %s" \
       GPRINT:varmax:"Maximum\:%8.2lf %s"
  done
done

