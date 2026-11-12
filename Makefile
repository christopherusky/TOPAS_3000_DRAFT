UV := uv run

SMOKE_COMPOSE := asyncapi_clients/smoketest/docker-compose.yml
SMOKE_BROKER  := topas-smoketest-artemis

.PHONY: sync schemas scope asyncapi asyncapi-clients diagrams validate test smoketest a11 a11-matrix a11-idempotency a11-auth a12-sync a13-request-status envelope tls clean all

sync:
	uv sync

schemas:
	$(UV) python bin/emit.py $(EMIT_ARGS)

# Which domains are live and which are deferred (scope.toml).
scope:
	$(UV) python bin/scope.py

asyncapi:
	$(UV) python bin/emit_asyncapi.py

asyncapi-clients:
	$(UV) python -m asyncapi_clients.generator.generate

diagrams:
	$(UV) python bin/generate_diagrams.py

validate:
	$(UV) python bin/validate.py

test:
	$(UV) pytest tests -v

# Run S1-S7 end-to-end against a throw-away local Artemis broker.
# Bring the broker up, drive every scenario, tear down — even on failure.
smoketest:
	@command -v mvn >/dev/null 2>&1 || { echo "[smoketest] Maven not found — install via 'apt install maven' or equivalent"; exit 1; }
	@command -v javac >/dev/null 2>&1 || { echo "[smoketest] JDK 11+ not found — install via 'apt install default-jdk' or equivalent"; exit 1; }
	@command -v dotnet >/dev/null 2>&1 || { echo "[smoketest] .NET 8 SDK not found — install via 'apt install dotnet-sdk-8.0' or equivalent"; exit 1; }
	@command -v node >/dev/null 2>&1 || { echo "[smoketest] Node.js 22+ not found — install via 'apt install nodejs npm' or use nvm"; exit 1; }
	@command -v npm >/dev/null 2>&1 || { echo "[smoketest] npm not found — install via 'apt install npm' or use nvm"; exit 1; }
	@command -v cargo >/dev/null 2>&1 || { echo "[smoketest] Rust/cargo not found — install via 'curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh' or your distro's rustup package"; exit 1; }
	@echo "[smoketest] building Java snippets..."
	@mvn -f asyncapi_clients/smoketest/java/pom.xml -B -q package
	@echo "[smoketest] building C# snippets..."
	@dotnet publish -c Release asyncapi_clients/smoketest/csharp/TopasSnippets.csproj --nologo -v quiet
	@echo "[smoketest] installing TypeScript dependencies (npm ci)..."
	@cd asyncapi_clients/smoketest/typescript && npm ci --silent
	@echo "[smoketest] building TypeScript snippets (tsc)..."
	@cd asyncapi_clients/smoketest/typescript && npm run build --silent
	@echo "[smoketest] building Rust snippets (cargo build --release)..."
	@cd asyncapi_clients/smoketest/rust && cargo build --release --quiet
	@echo "[smoketest] starting broker..."
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@echo "[smoketest] waiting for broker to become healthy..."
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/run_all.py; \
		status=$$?; \
		echo "[smoketest] tearing down broker..."; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

A11_LANG ?= python

# Every requester against every responder. Needs all five toolchains
# built first (make smoketest does that), and takes a few minutes.
# A11.2 idempotency: execute once, answer every attempt.
# A11 authenticated identity: what the authentication boundary provides.
# Uses its own broker (port 5674) with security genuinely enabled.
a11-auth:
	@command -v docker >/dev/null 2>&1 || { echo "[a11] Docker not found"; exit 1; }
	@docker compose -f asyncapi_clients/smoketest/docker-compose.auth.yml up -d
	@until docker inspect --format='{{.State.Health.Status}}' topas-smoketest-artemis-auth 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. $(UV) python asyncapi_clients/smoketest/a11_authenticated.py; \
		status=$$?; \
		docker compose -f asyncapi_clients/smoketest/docker-compose.auth.yml down; \
		exit $$status

# A12: a requested sync is addressed, ordered and closed.
a12-sync:
	@command -v docker >/dev/null 2>&1 || { echo "[a12] Docker not found — needed for the Artemis broker"; exit 1; }
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/a12_sync.py; \
		status=$$?; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

# requestStatus is acknowledged, then answered by a delta update.
a13-request-status:
	@command -v docker >/dev/null 2>&1 || { echo "[a13] Docker not found — needed for the Artemis broker"; exit 1; }
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/a13_request_status.py; \
		status=$$?; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

# Every data message carries the envelope the specification requires.
envelope:
	@command -v docker >/dev/null 2>&1 || { echo "[envelope] Docker not found — needed for the Artemis broker"; exit 1; }
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/envelope_conformance.py; \
		status=$$?; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

# The Broker offers TLS 1.3 only, with mutual authentication (AC5-4).
# Own broker on port 5675; throwaway certificates are generated first.
tls:
	@command -v docker >/dev/null 2>&1 || { echo "[tls] Docker not found"; exit 1; }
	@bash asyncapi_clients/smoketest/tls/make_certs.sh asyncapi_clients/smoketest/tls/certs >/dev/null
	@docker compose -f asyncapi_clients/smoketest/docker-compose.tls.yml up -d
	@until docker inspect --format='{{.State.Health.Status}}' topas-smoketest-artemis-tls 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. $(UV) python asyncapi_clients/smoketest/tls_conformance.py; \
		status=$$?; \
		docker compose -f asyncapi_clients/smoketest/docker-compose.tls.yml down; \
		exit $$status

a11-idempotency:
	@command -v docker >/dev/null 2>&1 || { echo "[a11] Docker not found — needed for the Artemis broker"; exit 1; }
	@echo "[a11] starting broker..."
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/a11_idempotency.py; \
		status=$$?; \
		echo "[a11] tearing down broker..."; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

a11-matrix:
	@command -v docker >/dev/null 2>&1 || { echo "[a11] Docker not found — needed for the Artemis broker"; exit 1; }
	@echo "[a11] starting broker..."
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/a11_matrix.py; \
		status=$$?; \
		echo "[a11] tearing down broker..."; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

a11:
	@command -v docker >/dev/null 2>&1 || { echo "[a11] Docker not found — needed for the Artemis broker"; exit 1; }
	@echo "[a11] starting broker..."
	@docker compose -f $(SMOKE_COMPOSE) up -d
	@until docker inspect --format='{{.State.Health.Status}}' $(SMOKE_BROKER) 2>/dev/null | grep -q healthy; do sleep 2; done
	@PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
		$(UV) python asyncapi_clients/smoketest/a11_isolation.py --lang $(A11_LANG); \
		status=$$?; \
		echo "[a11] tearing down broker..."; \
		docker compose -f $(SMOKE_COMPOSE) down; \
		exit $$status

clean:
	rm -rf Schemas/* Schemas_asyncapi/* Diagrams/mermaid/* Diagrams/uml/* Diagrams/png/*

all: clean schemas asyncapi asyncapi-clients diagrams validate test