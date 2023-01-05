# rust-api
A rust API REST using actix, diesel, tokio and swagger

### Installation
1. Make sure you have Rust and the following modules installed: Actix, Tokio, Diesel, and Swagger. If you don't have them installed yet, follow the instructions on the Rust website.
2. Clone this repository on your local machine:

    ```git clone git@github.com:mitomono/rust-api.git```
    
    or 

    ```git clone https://github.com/mitomono/rust-api.git```


3. Enter the project folder:
   
   ```cd project-name```

## Usage

To use this API, a MAKEFILE has been included with the main options for the project:

```
build:
	docker-compose -f docker-compose.yml build api

build-dev:
	docker-compose -f docker-compose.yml build api_dev

build-test:
	docker-compose -f docker-compose.yml build test

run-dev:
	docker-compose -f docker-compose.yml run api_dev

run:
	docker-compose -f docker-compose.yml run api

run-test:
	docker-compose -f docker-compose.yml run test

up: build
	docker-compose -f docker-compose.yml up api -d

up-dev: build-dev
	docker-compose -f docker-compose.yml up api_dev

down:
	docker-compose -f docker-compose.yml down -v
```

To run them, simply type ```make <option>```, with the options shown in the file.
This way, the services are launched using different Dockerfiles and Dockercompose depending on the selected option.

If you don't have Docker installed or don't want to use it, you can run it locally using Cargo commands: 
```cargo run```, ```cargo test```, ```cargo build```, etc. 

To do this, you need to have Rust previously installed on your operating system.

To use the repository, you have to create 2 files: 

- .env
- .env_test 

That contain the database configuration and environment variables for the project. 
These files have the following structure:

.env
```
RUST_LOG=rest_api=info,actix=info,diesel_migrations=info
DATABASE_URL=postgres://postgres:postgres@db:5432/tests
HOST=0.0.0.0
PORT=8000
```

.env_test
```
RUST_LOG=rest_api=info,actix=info,diesel_migrations=info
DATABASE_URL=postgres://postgres:postgres@db_test:5432/tests
HOST=0.0.0.0
PORT=8000
```

In case you use docker-compose, these examples are valid as configuration files. Otherwise, you have to change line 2:

```DATABASE_URL=postgres://postgres:postgres@db_test:5432/tests```

to

```DATABASE_URL=postgres://<user>:<password>@<host>:5432/<database>```

You can leave the port and host as they are or use the ones that suit you best.


If you are not going to use the version in Docker, you have to have installed diesel-cli ```cargo install diesel_cli --no-default-features --features postgres```, once installed run ```diesel migration run``` to create the tables in the database.
## API documentation

Consult the [ API documentation](http://localhost:8000/swagger-ui) generated by Swagger in http://localhost:8000/swagger-ui for more information about available routes and parameters.

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.


## Contributions
If you want to contribute to the project, follow these steps:

1. Fork the repository.
2. Create a branch to make your changes.
3. Commit your changes and push the branch to your fork.
4. Create a pull request for us to review your changes.

## Credits

This project would not have been possible without the help and collaboration of the following people:

- [Adrian Lara Roldan](https://github.com/mitomono/rust-api)
  Thank you all!
