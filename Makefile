dev:
	cargo watch -x run

run: 
	cargo run

health:
	curl http://localhost:3001/health

predictions:
	curl http://localhost:3001/predictions

## Postgres
postgres-up:
	docker run -d --name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD=123456 postgres:14

postgres-down:
	docker stop postgres-14
	docker rm postgres-14

psql-session:
	PGPASSWORD=123456 psql -h localhost -U postgres -d postgres -p 5432