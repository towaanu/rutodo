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
- `POST /graphql` : Endpoint for graphql requests
- `GET /graphiql` : Graphql playground

### Rest
#### Todolists
- `GET /todo-lists` : List all todo lists
- `GET /todo-lists/:id` : Get details about a todo list with the provided id
- `POST /todo-lists` : Create a new todo list ( label must be provided as json : `{"label": "a label"}` )
- `DELETE /todo-lists/:id` : Delete todo lists with provided id

#### Todos
- `GET /todos/:id` : Get details about a todo with the provided id
- `POST /todos` : Create a new todo ( label must be provided as json : `{"label": "todo label", "todoListId": ...}` )
- `DELETE /todos/:id` : Delete todo with provided id
- `GET /todos/:id/toggle` : Toggle the todo id with the provided id
