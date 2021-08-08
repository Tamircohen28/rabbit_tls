CONTAINER ?= some-rabbit
IMAGE ?= rabbitmq_tls

docker-clean:
	docker stop $(CONTAINER)
	docker rm $(CONTAINER)

docker-build:
	docker build . -t $(IMAGE)

docker-run: docker-build
	docker run -d --hostname my-rabbit --name $(CONTAINER) -p 8080:15672 $(IMAGE)