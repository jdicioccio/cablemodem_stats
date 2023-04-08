#!/bin/sh

while true; do
  echo "$(date +'%Y-%m-%d %H:%M') Running ..."
  timeout -k 360 300 bash -c 'cablemodem_stats -o influxdbv2 -u ${INFLUXDB2_USERNAME} -p ${INFLUXDB2_PASSWORD} --influxdb-bucket ${INFLUXDB2_BUCKET} --influxdb-org ${INFLUXDB2_ORG} --influxdb-url ${INFLUXDB2_URL} --influxdb-token ${INFLUXDB2_TOKEN} ${MODEM_TYPE}'
  sleep 300
done
