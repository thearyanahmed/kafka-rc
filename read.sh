#!/bin/bash

if [ -z $1 ] ; then
	echo " Topic name not given\n"
	echo " Use it like ./read.sh my-topic"
	exit 1;
fi

echo " NOTE: Starting interactive terminal. To exit, press ctrl + c"
echo " If you see a blank screen, make sure to write something using ./write.sh topic" 

docker exec --interactive --tty broker \
kafka-console-consumer --bootstrap-server broker:9092 \
                       --topic $1 \
                       --from-beginning
