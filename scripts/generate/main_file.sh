#!/bin/sh

# Create the main.go file
cat > main.go <<EOF
package main

import (
    "fmt"
    "log"
    "net/http"
    "os"
    "github.com/gorilla/mux"
    "github.com/gorilla/handlers"
    "github.com/joho/godotenv"
    "github.com/jinzhu/gorm"
    _ "github.com/jinzhu/gorm/dialects/sqlite"
    "github.com/dgrijalva/jwt-go"
    "golang.org/x/crypto/bcrypt"
    "github.com/dgrijalva/jwt-go/request"
    "github.com/jinzhu/gorm/dialects/postgres"
    "github.com/jinzhu/gorm/dialects/mysql"
    _ "github.com/go-sql-driver/mysql"
    _ "github.com/lib/pq"
    _ "github.com/mattn/go-sqlite3"
    "github.com/jinzhu/gorm/dialects/mssql"
    _ "github.com/denisenkom/go-mssqldb"
    "github.com/go-sql-driver/mysql"
)

func main() {
    // Load the .env file
    err := godotenv.Load()
    if err != nil {
        log.Fatal("Error loading .env file")
    }

    // Create a new router
    r := mux.NewRouter()

    // Create a new subrouter for the API
    api := r.PathPrefix("/api").Subrouter()

    // Create a new subrouter for the auth
    auth := r.PathPrefix("/auth").Subrouter()

    // Create a new subrouter for the users
    users := api.PathPrefix("/users").Subrouter()

    // Create a new subrouter for the posts
    posts := api.PathPrefix("/posts").Subrouter()

    // Create a new subrouter for the comments
    comments := api.PathPrefix("/comments").Subrouter()

    // Create a new subrouter for the likes
    likes := api.PathPrefix("/likes").Subrouter()

    // Create a new subrouter for the follows
    follows := api.PathPrefix("/follows").Subrouter()

    // Create a new subrouter for the messages
    messages := api.PathPrefix("/messages").Subrouter()

    // Create a new subrouter for the notifications
    notifications := api.PathPrefix("/notifications").Subrouter()

    // Create a new subrouter for the settings
    settings := api.PathPrefix("/settings").Subrouter()

    // Create a new subrouter for the search
    search := api.PathPrefix("/search").Subrouter()

    // Create a new subrouter for the reports
    reports := api.PathPrefix("/reports").Subrouter()

    // Create a new subrouter for the categories
    categories := api.PathPrefix("/categories").Subrouter()

    // Create a new subrouter for the tags
    tags := api.PathPrefix("/tags").Subrouter()

    // Create a new subrouter for the roles
    roles := api.PathPrefix("/roles").Subrouter()

    // Create a new subrouter for the permissions
    permissions := api.PathPrefix("/permissions").Subrouter()

    // Create a new subrouter for the files
    files := api.PathPrefix("/files").Subrouter()

    // Create a new subrouter for the events
    events := api.PathPrefix("/events").Subrouter()

    // Create a new subrouter for the groups
    groups := api.PathPrefix("/groups").Subrouter()


    // start the server
    log.Fatal(http.ListenAndServe(":8080", r))
}

EOF