#!/bin/bash

echo -e "\nTesting probe output\n"
../target/debug/examples/hello &
tide=$!
trace=-1
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    bpftrace -p $tide ../tools/probe.bt > probe.out &
    trace=$!
else
    echo "${$OSTYPE} : unsupported OS" 
    exit
fi

sleep 2
curl -s http://localhost:8080/ > /dev/null
sleep 1
kill -2 $trace > /dev/null
kill -9 $tide > /dev/null

if [ $(cat probe.out | grep tag-text | wc -l) == 1 ]
then 
    echo -e "\e[1;32m PASS \e[0m\n"
else
    echo -e "\e[1;31m FAIL \e[0m\n"
    echo "Trace content:"
    cat probe.out
fi

rm -fr probe.out 