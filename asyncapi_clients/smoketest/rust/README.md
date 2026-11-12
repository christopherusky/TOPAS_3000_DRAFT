# asyncapi-clients Rust smoke harness

Cargo package that compiles the seven generated Rust snippets at
[`../../output/rust/snippets/`](../../output/rust/snippets/) into
`target/release/sN` binaries via seven `[[bin]]` entries pointing at
those files in place. The orchestrator at `../run_all.py` invokes
each scenario as `target/release/sN`.

This file documents how to run **one** Rust binary manually for
debugging — the `make smoketest` target drives all 7 in sequence
across all available languages. For the other languages' equivalents,
see [`../README.md`](../README.md), [`../java/README.md`](../java/README.md),
[`../csharp/README.md`](../csharp/README.md),
[`../typescript/README.md`](../typescript/README.md), and
[`../typescript/README.md`](../typescript/README.md).

## Build

```bash
cd asyncapi_clients/smoketest/rust
cargo build --release            # cold ~2-3 min; warm ~5s
ls target/release/s?
```

Expected: `target/release/{s1,s2,...,s7}` (seven binaries).

## Why one package with seven `[[bin]]` (instead of a workspace)?

Each Rust snippet declares its own `async fn main()` under
`#[tokio::main]`. Cargo's `[[bin]]` syntax handles multiple-binary
single-package layouts cleanly — and crucially, it shares one
compile of the dep graph (fe2o3-amqp + tokio + serde_json + uuid)
across all seven binaries. A workspace with seven member crates
would re-resolve and re-compile the dep graph per member, ~20× more
wall clock for no benefit.

The `[[bin]]` `path` entries point at `../../output/rust/snippets/sN.rs`
so the generated tree stays purely generated; this directory owns
its own build config (Cargo.toml + Cargo.lock).

## Run a single Rust scenario manually

Bring up the broker (see [`../README.md`](../README.md)). Then, from
the repo root:

```bash
# S1 — subscribe-and-print 5 detector.measurement frames.
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=5 \
    asyncapi_clients/smoketest/rust/target/release/s1

# In another terminal, fire the Python publisher.
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
    uv run python asyncapi_clients/smoketest/publisher.py
```

Each binary respects `TOPAS_AMQP_URL` and (where applicable)
`TOPAS_MAX_MESSAGES`, `TOPAS_ORIGINATOR` env vars — same env-var
contract as Python / Java / C# / TypeScript.

## Pinning rationale

`Cargo.toml` pins `fe2o3-amqp = "=0.14.0"` (note the `=`) so CI is
deterministic. Pre-1.0 crates can have breaking minor bumps; the
exact-version pin is the safe default. tokio, serde_json, serde, and
uuid use semver carets — those crates are post-1.0 and follow strict
semver.

`Cargo.lock` is committed; combined with `=` pins, this gives
fully-reproducible CI builds.

To bump fe2o3-amqp:

```bash
cargo update -p fe2o3-amqp --precise X.Y.Z
cargo build --release    # verifies new version still builds
```

Commit both `Cargo.toml` and `Cargo.lock` together. Be aware that
fe2o3-amqp 0.x minor bumps can be heavily breaking — the 0.13→0.14
migration touched body-handling, the AMQP `Value` variant naming,
and `TransparentVec` field privacy. Plan template-side fixes ahead
of any minor bump.

## fe2o3-amqp 0.14.0 idioms used here

The smoke-tested templates use these patterns; integrators should
mirror them:

- `receiver.recv::<Body<Value>>().await?` rather than
  `receiver.recv::<Value>()` — only `Body<T>` impls `FromBody<'de>`
  in 0.14.x.
- `Body::Data(batch)` arm iterates via `Deref<Target = Vec<Data>>`
  (`batch.iter()`) — `TransparentVec`'s inner field is private.
- AMQP `ApplicationProperties` carry `SimpleValue` not `Value`
  (the "simple" subset excludes Lists/Maps/Arrays/Described).
- Link capabilities only via the builder API: `Receiver::builder()
  .name(...).source(Source::builder().address(addr).capabilities(
  vec![Symbol::from("topic")]).build()).attach(&mut session)`. The
  3-arg `Receiver::attach(&mut session, name, addr)` shortcut
  silently produces a no-capability Source — works against
  non-Artemis brokers but fails on Artemis MULTICAST.
