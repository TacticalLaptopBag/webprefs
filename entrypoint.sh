#!/bin/sh

if [ ! -f /app/data/webprefs.db ]; then
    echo "Database is missing, copying default database..."
    mkdir -p /app/data/
    cp /app/default.db /app/data/webprefs.db
fi
exec ./webprefs
