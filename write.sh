#!/bin/bash

if [ -z $1 ] ; then
	echo " Topic name not given\n"
	echo " Use it like ./write.sh my-topic"
	exit 1;
fi

echo " NOTE: Starting interactive terminal. To exit, press ctrl + c"
echo " Please wait until you see '>' character in your terminal"

docker exec --interactive --tty broker \
kafka-console-producer --bootstrap-server broker:9092 \
                       --topic $1
