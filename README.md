# kafka-rc


This repository contains 
- A docker compose for zookeeper and kafka. 
- A rust client (WIP).
- Some bash scripts

After logging in to the server, 
```bash
cd kafka-rc
```

## Running the project

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

It will start an interactive terminal.

## Writing 
```bash
./write.sh $topicName
```

Eg:

```bash
./write.sh topic_1
```

It will start an interactive terminal, wait for the terminal to display '>' character.

## Setup procedure

I've already done the following steps, you can ignore the following part.

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

