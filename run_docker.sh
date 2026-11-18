#!/usr/bin/env bash
set -e

# Remove existing container if it exists
docker rm -f pg-rusty > /dev/null 2>&1 || true

# Run PostgreSQL container (native ARM64)
# Host networking makes port 5432 available on localhost directly
# Bind mount persists data at /Users/hp/p/web1/data on local Mac
docker run -d \
  --name pg-rusty \
  --network host \
  -e POSTGRES_USER=sa \
  -e POSTGRES_PASSWORD=Password1! \
  -e POSTGRES_DB=rusty \
  -e PGPORT=5432 \
  -v /Users/hp/p/web1/data:/var/lib/postgresql/data \
  postgres:17

echo "PostgreSQL is ready. Database: rusty, User: sa, Port: 5432"
