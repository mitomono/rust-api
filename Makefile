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

up-dev:
	docker-compose -f docker-compose.yml up api_dev

down:
	docker-compose -f docker-compose.yml down -v
