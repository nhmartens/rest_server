# Basic REST API Server in Rust

This repository implements a basic Webserver that is connected to an Sqlite Database.

## Test commands

Get all clients that are in the DB:

```shell
curl -s http://localhost:8080/api/clients
```

Add a client to the DB:

```shell
curl -X POST -H "Content-Type: application/json" -d '{"abbreviation": "Test", "name": "Test AG"}' http://localhost:8080/api/clients
```

Add a project to the DB:

```shell
curl -X POST -H "Content-Type: application/json" -d '{"client_id": {client_id}, "name": "Project 1"}' http://localhost:8080/api/projects
```

Get all projects that are in the DB:

```shell
curl -s http://localhost:8080/api/projects
```

Remove a client from the DB:

```shell
curl -s -X DELETE http://localhost:8080/api/clients/{id_of_client}
```
