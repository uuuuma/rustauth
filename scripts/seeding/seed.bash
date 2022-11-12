#!/usr/bin/env bash

WORKING_DIR=$(dirname $0)
SEED_FILE="${WORKING_DIR}/seed.sql"

docker compose exec -T postgres psql -U admin < $SEED_FILE
