FROM rabbitmq:management
COPY ./certificate/ /etc/rabbitmq/cert
RUN chown -R rabbitmq:rabbitmq /etc/rabbitmq/cert
COPY ./rabbitmq.conf /etc/rabbitmq