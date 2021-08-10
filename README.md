# Rutodo

This is an example project. It is a basic todolist web api ( graphql and rest ) using rust.

## Start the project 

### With docker 
If you have docker installed, you can do : 
```sh
docker-compose up
```
Then the api should be accessible at [http://localhost:3030](http://localhost:3030)

### Without docker
To run the project you need to have a postgres database and rust installed.\
You can then configure the environment variable in the `docker-compose.yml` file to use your postgres database. \ 
Then you can start the project using : 
```sh 
cargo run 
```
The api should be accessible at [http://localhost:3030](http://localhost:3030)

## Endpoints 
### Graphql
- `/graphql` : endpoint for graphql requests
- `/graphiql` : Graphql playground
