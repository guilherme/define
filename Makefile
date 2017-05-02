.PHONY: define docker-env docker-machine-name

all: define

define: src/*.rs
	docker run -t -i define cargo run
