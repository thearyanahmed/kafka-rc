#!/bin/bash

cp .env.example .env
set -a; source .env; set +a;
echo "done \n"
echo "now run docker compose up"
