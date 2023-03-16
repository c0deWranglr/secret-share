FROM ubuntu:22.04

RUN apt-get update
RUN apt-get install -y openssl ca-certificates

COPY docker-run.sh docker-run.sh
COPY server/target/release/secret-share /server/secret-share
COPY client/build /client/build

RUN chown root docker-run.sh && chmod +x docker-run.sh
RUN chown -hR root server && chmod +x server/secret-share
RUN chown -hR root client

EXPOSE 8080

ENTRYPOINT [ "sh", "./docker-run.sh" ]