# asyncapi-clients Java smoke harness

Maven build that compiles the generated Java snippets in
[`../../output/java/snippets/`](../../output/java/snippets/) in place
and produces a shaded uber-jar containing `qpid-jms-client` +
`jackson-databind`. The orchestrator at `../run_all.py` invokes each
snippet as `java -cp target/topas-snippets-shaded.jar SN`.

This file documents how to run **one** Java snippet manually for
debugging — the `make smoketest` target drives all 7 in sequence.
For Python equivalents, see [`../README.md`](../README.md).

## Build

```bash
mvn -f asyncapi_clients/smoketest/java/pom.xml -B package
```

Expected output: `BUILD SUCCESS`. The shaded jar lands at
`target/topas-snippets-shaded.jar`.

## Run a single Java scenario manually

Bring up the broker first (see `../README.md` for the docker compose
incantation). Then, from the repo root:

```bash
# S1 — subscribe-and-print 5 detector.measurement frames.
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=5 \
    java -cp asyncapi_clients/smoketest/java/target/topas-snippets-shaded.jar S1

# In another terminal, fire the Python publisher to feed it.
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
    uv run python asyncapi_clients/smoketest/publisher.py
```

Class names follow the snippet IDs: `S1`, `S2`, `S3`, `S4`, `S5`,
`S6`, `S7`. Each respects the `TOPAS_AMQP_URL` and (where
applicable) `TOPAS_MAX_MESSAGES`, `TOPAS_ORIGINATOR` env vars,
matching the Python equivalents.

## Why a hand-written pom?

The pom lives outside `output/` (which is generated and policed by
the drift workflow) so the smoke harness is free to evolve its build
config without polluting the customer-facing generated tree. The
`<sourceDirectory>` element points back at the generated snippets so
there is no copy.
