CONTAINER ?= some-rabbit
IMAGE ?= rabbitmq_tls
MANAGER_PORT ?= 15672
TLS_PORT ?= 5671

run:
	cargo run

build:
	cargo build

docker-clean:
	docker stop $(CONTAINER)
	docker rm $(CONTAINER)

docker-build:
	docker build . -t $(IMAGE)

docker-run: docker-build
	docker run -d --hostname my-rabbit --name $(CONTAINER) -p $(TLS_PORT):$(TLS_PORT) -p 8080:$(MANAGER_PORT) $(IMAGE)