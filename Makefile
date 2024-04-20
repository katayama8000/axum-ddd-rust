.PHONY: server-up db-up server-down db-down down ps 

server-up:
	docker-compose up -d server
	docker-compose exec -it server /bin/bash

db-up:
	docker-compose up -d db
	docker-compose exec -it db /bin/bash

server-down:
	docker-compose down server

db-down:
	docker-compose down db

down:
	docker-compose down

ps:
	docker-compose ps