FROM debian:buster-slim

COPY server/target/release/secret-share /server/secret-share
COPY client/build /client/build

RUN chown -hR root server && chmod +x server/secret-share
RUN chown -hR root client

EXPOSE 8080

CMD ["./server/secret-share"]