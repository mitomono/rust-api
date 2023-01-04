run:
	docker-compose -f docker-compose.yml up api -d

run-dev:
	docker-compose -f docker-compose.yml up api_dev -d

run-test:
	docker-compose -f docker-compose.yml run test

build:
	docker-compose -f docker-compose.yml build api

up: build
	docker-compose -f docker-compose.yml up api -d

down:
	docker-compose -f docker-compose.yml down -v
