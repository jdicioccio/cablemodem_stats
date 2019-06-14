#!/bin/bash
DSLIST=""
VALLIST=""
for val in $(cat -); do
  DS=$(echo $val | sed 's/\./_/' | awk -F: '{print $1}')
  VAL=$(echo $val | awk -F: '{print $2}')
  if [ "x${DSLIST}" = "x" ]; then
    DSLIST="${DS}"
    VALLIST="${VAL}"
  else
    DSLIST="${DSLIST}:${DS}"
    VALLIST="${VALLIST}:${VAL}"
  fi
done
echo "rrdupdate inet.rrd -t ${DSLIST} N:${VALLIST}"
rrdupdate inet.rrd -t ${DSLIST} N:${VALLIST}
