#!/bin/sh

# Create the Dockerfile
cat > Dockerfile <<EOF
# Use the official golang image to create a build artifact.
# This is based on Debian and sets the GOPATH to /go.
# https://hub.docker.com/_/golang
FROM golang:1.16 as builder

# Create and change to the app directory.
WORKDIR /app

# Retrieve application dependencies.
# This allows the container build to reuse cached dependencies.
# Expecting to copy go.mod and if present go.sum.
COPY go.mod ./
COPY go.sum ./
RUN go mod download

# Copy local code to the container image.
COPY . ./

# Build the binary.
RUN go build -v -o main

# Use the official Debian slim image for a lean production container.
# https://hub.docker.com/_/debian

FROM debian:bullseye-slim

# Copy the binary to the production image from the builder stage.
COPY --from=builder /app/main /app/main

# Run the web service on container startup.
CMD ["/app/main"]
EOF

# Create the .dockerignore file
cat > .dockerignore <<EOF
# .dockerignore
# This file tells Docker which files to ignore when building the image

# Ignore the .git directory
.git

# Ignore the .env file
.env

# Ignore the Dockerfile
Dockerfile

# Ignore the .dockerignore file
.dockerignore
EOF