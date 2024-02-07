#!/bin/sh

# Create the .env file
cat > .env <<EOF
# .env
# This file contains the environment variables for the application

# Database
DB_DRIVER=sqlite3
DB_USERNAME=
DB_PASSWORD=
DB_NAME=database.db
DB_HOST=localhost
DB_PORT=3306

# JWT
JWT_SECRET=secret
EOF