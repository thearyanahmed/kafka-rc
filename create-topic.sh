#!/bin/bash

if [ -z $1 ] ; then
	echo " Topic name not given\n"
	echo " Use it like ./create-topic.sh my-topic"
	exit 1;
fi

docker exec broker \
kafka-topics --bootstrap-server broker:9092 \
             --create \
             --topic $1
