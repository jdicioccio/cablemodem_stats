#!/bin/bash
echo "rrdtool create inet.rrd \\"
echo "--step 300 \\"
DSLIST=""
VALLIST=""
for val in $(cat -); do
  DS=$(echo $val | sed 's/\./_/' | awk -F: '{print $1}')
  TYPE="GAUGE"
  LOWER_LIMIT="0"

  echo $DS | grep -E '_e$' >/dev/null 2>&1
  if [ $? -eq 0 ]; then
    TYPE="COUNTER"
  fi

  echo $DS | grep -E '_power$' >/dev/null 2>&1
  if [ $? -eq 0 ]; then
    LOWER_LIMIT="U"
  fi

  echo "DS:$DS:$TYPE:600:$LOWER_LIMIT:U \\"
done
echo "RRA:AVERAGE:0.5:1:315360"
