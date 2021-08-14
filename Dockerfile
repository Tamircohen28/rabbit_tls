# pull rabbit managment image
FROM rabbitmq:management

# copy certificate direcroty
COPY ./certificate/ /etc/rabbitmq/cert

# set ownership to rabbit user
RUN chown -R rabbitmq:rabbitmq /etc/rabbitmq/cert

# copy rabbit server configuration
COPY ./rabbitmq.conf /etc/rabbitmq