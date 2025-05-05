include .env
export $(shell sed 's/=.*//' .env)

.PHONY: setup_tools
setup_tools:
	pip install -r requirements.txt

.PHONY: migrate
migrate:
	alembic upgrade head

.PHONY: seed
seed:
	python scripts/seeding.py

.PHONY: reset
reset:
	alembic downgrade base

.PHONY: build_api_doc
build_api_doc:
	mkdir -p dist
	npx swagger-merger -i openapi/stock-analytics.yaml -o dist/swagger.json

.PHONY: local_run
local_run:
	docker-compose -f localexec/docker-compose.yaml up -d --build

.PHONY: local_stop
local_stop:
	docker-compose -f localexec/docker-compose.yaml down