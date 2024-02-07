use crate::cmd::{FileStructure, FolderStructure};

// Internal Folder
// Folders: adapters, core
const ADAPTERS_FOLDER: &str = "adapters";
const CORE_FOLDER: &str = "core";

// Adapters Folder
// Folders: cache, handler, repository, token
const CACHE_FOLDER: &str = "cache";
const HANDLER_FOLDER: &str = "handler";
const REPOSITORY_FOLDER: &str = "repository";
const TOKEN_FOLDER: &str = "token";

// Cache Files
const CACHE_FILE: &str = "cache.go";

// Handler Files
const HANDLER_FILE: &str = "handler.go";

// Repository Files
const REPOSITORY_FILE: &str = "repository.go";

// Token Files
const TOKEN_FILE: &str = "token.go";

// Core Folder
// Folders: domain, port, service, util
const DOMAIN_FOLDER: &str = "domain";
const PORT_FOLDER: &str = "port";
const SERVICE_FOLDER: &str = "service";
const UTIL_FOLDER: &str = "util";

// Domain Files
const ENTITY_FILE: &str = "entity.go";
const VALUE_OBJECT_FILE: &str = "value_object.go";

// Port Files
const INPUT_PORT_FILE: &str = "input_port.go";
const OUTPUT_PORT_FILE: &str = "output_port.go";

// Service Files
const SERVICE_FILE: &str = "service.go";

// Util Files
const ERROR_FILE: &str = "error.go";
const LOGGER_FILE: &str = "logger.go";

pub fn internal_folders() -> Vec<FolderStructure> {
    vec![
        FolderStructure {
            folder_title: ADAPTERS_FOLDER.to_string(),
            sub_folders: Some(vec![
                FolderStructure {
                    folder_title: CACHE_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![FileStructure {
                        file_name: CACHE_FILE.to_string(),
                        content: "package cache".to_string(),
                    }]),
                },
                FolderStructure {
                    folder_title: HANDLER_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![FileStructure {
                        file_name: HANDLER_FILE.to_string(),
                        content: "package handler".to_string(),
                    }]),
                },
                FolderStructure {
                    folder_title: REPOSITORY_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![FileStructure {
                        file_name: REPOSITORY_FILE.to_string(),
                        content: "package repository".to_string(),
                    }]),
                },
                FolderStructure {
                    folder_title: TOKEN_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![FileStructure {
                        file_name: TOKEN_FILE.to_string(),
                        content: "package token".to_string(),
                    }]),
                },
            ]),
            files: None,
        },
        FolderStructure {
            folder_title: CORE_FOLDER.to_string(),
            sub_folders: Some(vec![
                FolderStructure {
                    folder_title: DOMAIN_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![
                        FileStructure {
                            file_name: ENTITY_FILE.to_string(),
                            content: "package domain".to_string(),
                        },
                        FileStructure {
                            file_name: VALUE_OBJECT_FILE.to_string(),
                            content: "package domain".to_string(),
                        },
                    ]),
                },
                FolderStructure {
                    folder_title: PORT_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![
                        FileStructure {
                            file_name: INPUT_PORT_FILE.to_string(),
                            content: "package port".to_string(),
                        },
                        FileStructure {
                            file_name: OUTPUT_PORT_FILE.to_string(),
                            content: "package port".to_string(),
                        },
                    ]),
                },
                FolderStructure {
                    folder_title: SERVICE_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![FileStructure {
                        file_name: SERVICE_FILE.to_string(),
                        content: "package service".to_string(),
                    }]),
                },
                FolderStructure {
                    folder_title: UTIL_FOLDER.to_string(),
                    sub_folders: None,
                    files: Some(vec![
                        FileStructure {
                            file_name: ERROR_FILE.to_string(),
                            content: "package util".to_string(),
                        },
                        FileStructure {
                            file_name: LOGGER_FILE.to_string(),
                            content: "package util".to_string(),
                        },
                    ]),
                },
            ]),
            files: None,
        },
    ]
}