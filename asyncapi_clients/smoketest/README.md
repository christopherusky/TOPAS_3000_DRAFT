# asyncapi-clients smoke test

Local end-to-end check that the generated snippets actually move bytes
over a real AMQP 1.0 broker.

This is a smoke harness, not a production mirror — plain TCP, no
mTLS, no auth, in-memory broker that drops on `down`.

## Prerequisites

- Docker (Compose v2)
- `uv sync` has installed `python-qpid-proton` (dev dependency)
- Run all snippet/publisher commands with `PYTHONPATH=.` from the
  repo root. The generated snippets import the SDK by absolute path
  (`asyncapi_clients.output.python.sdk.*`), and Python's default
  `sys.path` puts the script's directory first, not the repo root.

## Run everything (recommended)

```bash
make smoketest
```

Brings the broker up, runs S1–S7 end-to-end via
`asyncapi_clients/smoketest/run_all.py`, tears the broker down even
on failure. The same script runs in CI
(`.github/workflows/asyncapi-clients-smoketest.yml`), so a green
local run means a green CI run.

The sections below describe the manual run for each scenario —
useful for debugging a single failure.

## Run S1 (subscribe-and-print)

```bash
# Terminal 1 — start the broker (first run pulls ~250 MB)
docker compose -f asyncapi_clients/smoketest/docker-compose.yml up -d

# Terminal 2 — start S1 first (will block waiting for messages)
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/output/python/snippets/s1.py

# Terminal 3 — inject 5 synthetic detector.measurement frames
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/publisher.py

# Tear down
docker compose -f asyncapi_clients/smoketest/docker-compose.yml down
```

Expected: S1 prints 5 messages then exits cleanly.

## Run S2 (wildcard subscription)

```bash
# Terminal 1 — start the broker (skip if already up)
docker compose -f asyncapi_clients/smoketest/docker-compose.yml up -d

# Terminal 2 — subscribe to *.fault (waits for messages, exits after 3)
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=3 \
  uv run python asyncapi_clients/output/python/snippets/s2.py

# Terminal 3 — spray one fault to each live domain in scope.toml
TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/fault_publisher.py
```

Expected: S2 prints `[fault from detector]`, `[fault from traffic-signal]`,
`[fault from vms]` (in any order), then exits.

## Run S4 (sync batch + delta stream)

```bash
# Terminal 1 — start the broker (skip if already up)
docker compose -f asyncapi_clients/smoketest/docker-compose.yml up -d

# Terminal 2 — start S4 (it does not self-exit; Ctrl-C when done)
TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/output/python/snippets/s4.py

# Terminal 3 — emit a 3-message sync batch then 2 deltas
TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/sync_delta_publisher.py
```

Expected from S4:
- three `[sync <uuid> N/3] {...}` lines, in any order
- one `[sync <uuid>] complete` line once the batch is done
- two `[delta] {...}` lines

## Run S3 (command + response round-trip)

```bash
# Terminal 1 — start the broker (skip if already up)
docker compose -f asyncapi_clients/smoketest/docker-compose.yml up -d

# Terminal 2 — mock VMS that echoes acks on vms.command-response
TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/vms_responder.py
# (or set TOPAS_RESPONDER_MAX_REPLIES=1 to make it self-exit after one)

# Terminal 3 — the S3 snippet sends one command, awaits its response
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_ORIGINATOR=s3-smoke-test \
  uv run python asyncapi_clients/output/python/snippets/s3.py
```

Expected: S3 prints one `[response] {...}` line then exits cleanly.
The responder echoes back the originator and the correlation-id, so
the S3 client matches on correlation-id and prints the ack.

## Run S5 / S6 / S7

```bash
# S5 — traffic-signal telemetry (typed SDK round-trip)
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=5 \
  uv run python asyncapi_clients/output/python/snippets/s5.py
# (in another terminal)
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/traffic_signal_publisher.py

# S6 — discriminated-union dispatch (SCOOT/GPS/ANPR)
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=3 \
  uv run python asyncapi_clients/output/python/snippets/s6.py
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/transport_link_publisher.py

# S7 — network topology snapshot (5 channels, anonymous-relay publisher)
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/output/python/snippets/s7.py
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
  uv run python asyncapi_clients/smoketest/network_publisher.py
```

Expected:
- **S5** prints 5 `signal=TS-NNNN mode=... sub=... plan=N` lines.
- **S6** prints one each of `SCOOT flow=... occupancy%=...`,
  `GPS meanTravelTime=...s`, `ANPR in=... out=... matches=...`.
- **S7** prints `[topology] node=1, link=1, zone=1, path=1, turn=1`
  and exits.

## What the broker exposes

| Port | Protocol           | Notes                                 |
|------|--------------------|---------------------------------------|
| 5672 | AMQP 1.0 (plain)   | Snippets connect here                 |
| 8161 | HTTP (web console) | http://localhost:8161  admin/admin    |

Anonymous login is enabled — snippets do not need to pass credentials.

## Why no mTLS here?

Production TOPAS uses mTLS (A7). This harness deliberately
skips it: certificate provisioning is broker-specific, and the goal
of the smoke test is to validate that **envelope conventions**
(`x-originator`, `x-message-mode`, channel naming, JSON body shape)
flow correctly, not to re-validate the TLS posture. A second harness
with mTLS-on can come later once a CA and per-client certs are in
play.

## Conformance suites

Beside the S1–S7 scenarios, three suites test requirements the specification states as MUSTs. Each runs a deliberately defective counterpart as well and fails if any defect goes unreported.

```bash
# Requested sync (A12), against the plain broker on 5672
PYTHONPATH=. uv run python asyncapi_clients/smoketest/a12_sync.py

# The data-message envelope on every smoke publisher, against the plain broker
PYTHONPATH=. uv run python asyncapi_clients/smoketest/envelope_conformance.py

# TLS 1.3 only with mutual authentication: its own broker on 5675
bash asyncapi_clients/smoketest/tls/make_certs.sh asyncapi_clients/smoketest/tls/certs
docker compose -f asyncapi_clients/smoketest/docker-compose.tls.yml up -d
PYTHONPATH=. uv run python asyncapi_clients/smoketest/tls_conformance.py
docker compose -f asyncapi_clients/smoketest/docker-compose.tls.yml down
```

`make a12-sync`, `make envelope` and `make tls` do the same with the broker started and stopped for you. The publishers build their headers with `envelope.py`'s `data_headers`, which refuses a set that breaks the envelope rule.
