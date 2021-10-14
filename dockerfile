FROM debian:buster-slim
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
# COPY /server/target/release/secret-share /server/secret-share
# COPY /client/build /client/build
CMD ["./server/secret-share"]