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