all: server

clean:
	rm -rf target

crawler:
	cargo r --bin crawler

server:
	cargo r --bin server

build:
	cargo build --release
	cd www && yarn build

copy:
	rm -rf dist
	mkdir dist
	cp target/release/crawler dist/
	cp target/release/server dist/
	cp favicon.ico dist/
	cp Rocket.toml dist/
	mkdir -p dist/www
	cp www/build.js dist/www/
	cp www/index.html dist/www/

dist: build copy

db:
	psql -d DB_URL
