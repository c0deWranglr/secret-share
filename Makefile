# ---- CONFIG ----
config-client:
	@vi client/public/config.js

config-server:
	@vi server/.cargo/config.toml

# ---- RELEASE ----
release-client:
	@cd client && npm run build

release-server:
	@cd server && cargo build --release

release-docker:
	@docker build -t secret-share .

release: release-client release-server release-docker

# ---- LOCAL START/RUN ----
start-client:
	@cd client && npm start

start-server:
	@echo "You can configure envs at server/.cargo/config.toml\n\n"
	@cd server && cargo run

PORT ?= 8080
start-docker:
	@echo "Running on http://localhost:${PORT}\n"
	@docker run --rm --name secret-share \
	-e `cat server/.cargo/config.toml | sed 's/\[env\]//g' | sed 's/#.*//g' | xargs | sed 's/ / -e /g'` \
	-e PORT=${PORT} \
	-p ${PORT}:${PORT} \
	-it secret-share http://localhost:${PORT}
