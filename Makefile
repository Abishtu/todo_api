
MAKEFILE_ENV=.env

include ${MAKEFILE_ENV}

define copy_env
	cp .env ./docker/.env
endef

define build
	docker compose -p ${COMPOSE_PROJECT_NAME} -f ./docker/docker-compose.yaml build
endef

define up
	docker compose -p ${COMPOSE_PROJECT_NAME} -f ./docker/docker-compose.yaml up
endef

define stop
	docker compose -p ${COMPOSE_PROJECT_NAME} -f ./docker/docker-compose.yaml down --remove-orphans
endef

define session
	docker compose -p ${COMPOSE_PROJECT_NAME} -f ./docker/docker-compose.yaml run -it todo-api bash
endef

define run_local
	POSTGRES_USER=${POSTGRES_USER}\
	POSTGRES_PASSWORD=${POSTGRES_PASSWORD}\
	POSTGRES_PORT=${POSTGRES_PORT}\
	POSTGRES_DB=${POSTGRES_DB}\
	cargo run
endef

build:
	${call copy_env}
	${call build}

up:
	${call up}

stop:
	${call stop}

deploy:
	${call copy_env}
	${call build}
	${call up}

redeploy:
	${call stop}
	${call copy_env}
	${call build}
	${call up}

session:
	${call session}

local_run:
	${call run_local}