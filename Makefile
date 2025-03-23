dev:
	cargo watch -x run

run: 
	cargo run

health:
	curl http://localhost:3001/health

predictions:
	curl http://localhost:3001/predictions