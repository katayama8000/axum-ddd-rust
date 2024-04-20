.PHONY: server-up db-up down ps

server-up:
	docker-compose up -d
	docker-compose exec -it server /bin/bash

db-up:
	docker-compose up -d
	docker-compose exec -it db /bin/bash

down:
	docker-compose down

ps:
	docker-compose ps