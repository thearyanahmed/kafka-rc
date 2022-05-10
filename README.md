# kafka-rc


This repository contains 
- A docker compose for zookeeper and kafka. 
- A rust client (WIP).
- Some bash scripts


## Bash scripts

First run 

```bash
chmod +x ./*.sh
```

Then,

```bash
./prep.sh
```

Then, 

```bash
docker compose up -d
```

**Note** It takes some time for kafka to start, it is recommeded you wait for a while.

Then, to create a topic

```bash
./create-topic.sh $topicName
```

Example:

```bash
// ./create-topic.sh topic_1
```

Then open 2 terminals, one for reading another for writing.

## Reading
```bash
./read.sh $topicName
```

So, 
```bash
./read.sh topic_1
```

It will start an interactive terminal, wait for the terminal to display '>' character.

## Writing 
```bash
./write.sh $topicName
```

Eg:

```bash
./write.sh topic_1
```

It will start an interactive terminal, wait for the terminal to display '>' character.
