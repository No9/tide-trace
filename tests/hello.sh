#!/bin/bash

echo "Testing hello for Result in Output\n"
../target/debug/examples/hello &
tide=$!
trace=-1
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    bpftrace -p $tide ../tools/route-time.bt > trace.out &
    trace=$!
elif [[ "$OSTYPE" == "darwin"* ]]; then
    dtrace -p $tide ../tools/route-time.d > trace.out &
    trace=$!
elif [[ "$OSTYPE" == "freebsd"* ]]; then
    dtrace -p $tide ../tools/route-time.d > trace.out &
    trace=$!
else
    echo "${$OSTYPE} : unsupported OS" 
fi

sleep 3
curl -s http://localhost:8080/ > /dev/null

kill -9 $trace > /dev/null
kill -9 $tide > /dev/null

if [ $(cat trace.out | grep Request | wc -l) == 1 ]
then 
    echo -e "\e[1;32m PASS \e[0m"
else
    echo -e "\e[1;31m FAIL \e[0m"
    echo "Trace content:"
    cat trace.out
fi

rm -fr trace.out 