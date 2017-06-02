all: clean crawler

clean:
	rm -rf target

crawler:
	cargo r --bin crawler

db:
	psql -d postgres://huy@localhost/codedaily
