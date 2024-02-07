#!/bin/sh

# Start the go project
go mod init github.com/username/project
go mod tidy

# Install the dependencies
go get -u github.com/gorilla/mux
go get -u github.com/gorilla/handlers
go get -u github.com/joho/godotenv
go get -u github.com/jinzhu/gorm
go get -u github.com/jinzhu/gorm/dialects/sqlite
go get -u github.com/dgrijalva/jwt-go
go get -u golang.org/x/crypto/bcrypt
go get -u github.com/dgrijalva/jwt-go/request
go get -u github.com/jinzhu/gorm/dialects/postgres
go get -u github.com/jinzhu/gorm/dialects/mysql
go get -u github.com/go-sql-driver/mysql
go get -u github.com/lib/pq
go get -u github.com/mattn/go-sqlite3
go get -u github.com/jinzhu/gorm/dialects/mssql
go get -u github.com/denisenkom/go-mssqldb
go get -u github.com/go-sql-driver/mysql

# Start the go project
sh scripts/generate/project.sh

# Create the main.go file
sh scripts/generate/main_file.sh

# Create the .env file
sh scripts/generate/env_file.sh

# Create the Dockerfile
sh scripts/generate/docker_file.sh

# Create the .gitignore file
cat >.gitignore <<EOF
# .gitignore
# This file tells Git which files to ignore when committing to the repository

# Ignore the .env file
.env

# Ignore the main binary
main

# Ignore the vendor directory
vendor
EOF

# Create the Makefile
cat >Makefile <<EOF
# Makefile
# This file contains the commands to build and run the application

# Build the application
build:
    go build -o main .

# Run the application
run:
    go run .

# Build the Docker image
docker-build:
    docker build -t myapp .

# Run the Docker container
docker-run:
    docker run -p 8080:8080 myapp

# Remove the Docker image
docker-rm:
    docker rmi myapp
EOF

# Call the generate_folder.sh script
sh scripts/generate/folders.sh
