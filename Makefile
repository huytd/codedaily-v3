all: server

clean:
	rm -rf target

crawler:
	cargo r --bin crawler

server:
	cargo r --bin server

db:
	psql -d DB_URL
