# asyncapi-clients TypeScript / Node.js smoke harness

TypeScript / Node.js 20 build that compiles the generated TypeScript
snippets at
[`../../output/typescript/snippets/`](../../output/typescript/snippets/)
in place to `dist/snippets/*.js`, and runs them under Node against a
live Apache ActiveMQ Artemis broker. The orchestrator at
`../run_all.py` invokes each scenario as
`node dist/snippets/sN.js`.

This file documents how to run **one** TS snippet manually for
debugging — the `make smoketest` target drives all 7 in sequence
across all available languages. For Python, Java, and C# equivalents,
see [`../README.md`](../README.md), [`../java/README.md`](../java/README.md),
and [`../csharp/README.md`](../csharp/README.md).

## Build

```bash
cd asyncapi_clients/smoketest/typescript
npm ci          # one-off; fast, deterministic from package-lock.json
npm run build   # tsc → dist/snippets/*.js
```

Expected output: `dist/snippets/{s1.js,s2.js,...,s7.js}`. ESM
modules; Node 20+ runs them directly with `node`.

## Run a single TS scenario manually

Bring up the broker first (see [`../README.md`](../README.md) for the
docker compose incantation). Then, from the repo root:

```bash
# S1 — subscribe-and-print 5 detector.measurement frames.
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=5 \
    node asyncapi_clients/smoketest/typescript/dist/snippets/s1.js

# In another terminal, fire the Python publisher to feed it.
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
    uv run python asyncapi_clients/smoketest/publisher.py
```

Filenames follow the snippet IDs lowercase: `s1.js` … `s7.js`. Each
respects the `TOPAS_AMQP_URL` and (where applicable)
`TOPAS_MAX_MESSAGES`, `TOPAS_ORIGINATOR` env vars, matching the
Python / Java / C# equivalents.

## Why no reflection dispatcher (unlike C#)?

Each TS snippet is a self-contained top-level script that opens its
own connection, registers its own event handlers, and calls
`process.exit(0)` after `MAX_MESSAGES` / `connection_close`. Node has
no "one entry point per assembly" rule, so `node dist/snippets/sN.js`
is the natural per-scenario invocation — simpler than C#'s reflection
dispatcher and matches Python's straight `python sN.py`.

The `tsconfig.json` in this directory's
`include: ["../../output/typescript/snippets/*.ts"]` and `rootDir:
../../output/typescript` settings point back at the generated tree so
there is no copy. The generated tree stays purely generated; this
directory owns its own build config.

## Pinning rationale

`package.json` pins the runtime dep `rhea` to **exact** version
`3.0.5` (no caret) so CI is bit-deterministic. `typescript` is
similarly pinned to `5.4.5`; `@types/node` uses `^20.0.0` because
type-stubs-within-a-major are stable. Bump pins via `npm install
<pkg>@<version>` (which updates `package-lock.json` too); commit
both files together.

## Default-import idiom for `rhea`

`rhea` 3.x is shipped as CommonJS, while this package is ESM
(`"type": "module"` in `package.json`). Templates therefore use:

```typescript
import rhea from "rhea";
import type { EventContext } from "rhea";
```

…rather than `import * as rhea from "rhea"`. The namespace-import
form compiles cleanly under `esModuleInterop: true` but yields
`{ default: <module> }` at runtime under Node's CJS interop, which
silently leaves `rhea.create_container` undefined. Default import is
the working idiom — and the same pattern integrators following
[`../../output/typescript/README.md`](../../output/typescript/README.md)
should use.
