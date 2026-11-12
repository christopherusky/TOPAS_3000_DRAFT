# asyncapi-clients — code generator + per-language client snippets

Generates AMQP 1.0 client snippets for **six languages** (Python, Java,
C#, C++, TypeScript, Rust) from a single hand-authored `asyncapi.yaml`
at the repo root. Five of those (Python, Java, C#, TypeScript, Rust)
have an end-to-end broker smoke harness running on every CI push, and
all five pass the A11 request/reply conformance gate. Only C++ ships
generated snippets that have not been validated against a live broker.

Go was dropped entirely; C++ remains generated but is off the roadmap.

This README is the entry point. It explains:

1. [Pipeline overview](#pipeline-overview)
2. [Quick start — regenerate everything](#quick-start--regenerate-everything)
3. [Toolchain prerequisites](#toolchain-prerequisites)
4. [What `make asyncapi-clients` actually does](#what-make-asyncapi-clients-actually-does)
5. [Per-language status + how to use each output](#per-language-status--how-to-use-each-output)
6. [Validating the output (smoke harness + drift CI)](#validating-the-output-smoke-harness--drift-ci)
7. [CI overview](#ci-overview)
8. [Adding or extending a language](#adding-or-extending-a-language)
9. [Where things live](#where-things-live)

---

## Pipeline overview

```
              topas_3000_model/                ◄── Pydantic models — TRUE source of truth
                   │
                   ├──► bin/emit.py
                   │         │
                   │         ▼
                   │       Schemas/             (JSON Schemas, one file per domain)
                   │
                   └──► bin/emit_asyncapi.py
                             │
                             │   asyncapi_template.yaml  ◄── hand-authored shell
                             │   (server config, security, message traits)
                             ▼
                          asyncapi.yaml        (generated, do not hand-edit)
                             │
                             ▼
                          Schemas_asyncapi/    (per-channel intermediates)
                             │
                             │   asyncapi_clients/templates/<lang>/*.j2
                             │   asyncapi_clients/generator/generate.py
                             ▼
                          asyncapi_clients/output/<lang>/
                            ├── sdk/        (Python only — typed model classes)
                            ├── snippets/   (S1..S7 runnable demos)
                            └── README.md   (per-language integrator guide)
                             │
                             ▼
                          asyncapi_clients/smoketest/
                            ├── publisher.py / fault_publisher.py / ...
                            ├── docker-compose.yml  (Apache Artemis 2.31.2)
                            ├── run_all.py          (orchestrator)
                            ├── java/      (mvn build for Java snippets)
                            └── csharp/    (dotnet build for C# snippets)
```

**The Pydantic models in `topas_3000_model/` are the only place a wire-format
change is hand-authored.** `Schemas/` and `asyncapi.yaml` are both
emitted from them by `bin/emit.py` and `bin/emit_asyncapi.py`
respectively. The only other hand-authored input is
`asyncapi_template.yaml` — a YAML shell carrying the server URL,
security blocks, and reusable message traits that don't belong in the
domain models.

Two CI workflows police the contract:

- **AsyncAPI Clients Drift** — re-runs `make asyncapi-clients` and
  fails if the regenerated output differs from what's committed.
- **AsyncAPI Clients Smoke Test** — three-leg matrix
  (`python | java | csharp`), each leg drives its seven snippets
  end-to-end against a throwaway Artemis container.

---

## Quick start — regenerate everything

From the repo root:

```bash
make asyncapi-clients
```

This is the single command for "regenerate every language's snippets
from the current `asyncapi.yaml` + templates". It writes everything
under `asyncapi_clients/output/<lang>/`. After running, commit any
changes — the drift workflow will fail otherwise.

To rebuild from scratch (clears all generated artefacts first):

```bash
make clean
make all
```

`make all` runs the full pipeline: `clean → schemas → asyncapi →
asyncapi-clients → diagrams → validate → test`.

---

## Toolchain prerequisites

### Always required (for the generator itself)

| Tool | Version | Used for |
|---|---|---|
| **Python** | 3.11 | the generator + `datamodel-code-generator` (Python SDK) + Jinja templates |
| **uv** | latest | dependency management (replaces `pip install -r`) |
| **Make** | any GNU make | one-command pipeline targets |
| **Node.js** | 22 | `npx quicktype` for the Java / C# / TypeScript SDKs |
| **cargo-typify** | 0.7.0 | the Rust SDK (`cargo install cargo-typify --version 0.7.0 --locked`) |

SDK generators by language:

| SDK | Generator | Notes |
|---|---|---|
| Python | `datamodel-code-generator==0.57.0` | Pydantic v2 models |
| Java / C# / TypeScript | pinned `quicktype` | discriminated unions emitted separately as real tagged unions |
| Rust | pinned `cargo-typify` | real `serde` enums; `const` is presented to typify as a single-member `enum` so the discriminator works |
| C++ | *none* | off the roadmap |

Bootstrap once:

```bash
uv sync
```

This installs every dependency the generator needs (Jinja2, Pydantic,
`datamodel-code-generator`, `python-qpid-proton` for the smoke harness)
into `.venv/`.

### Required if running the **smoke harness** for a specific language

| Tool | Used for | Install hint |
|---|---|---|
| **Docker** (with Compose v2) | spins up the Apache Artemis broker | `apt install docker-compose-plugin` |
| **JDK 11+ + Maven** | building Java snippet uber-jar | `apt install default-jdk maven` |
| **.NET 8 SDK** | building the C# `TopasSnippets.dll` | Microsoft package feed: `apt install dotnet-sdk-8.0` |

`make smoketest` does pre-flight checks for `mvn`, `javac`, and
`dotnet`, with actionable error messages if any is missing. It also
auto-skips a language's leg under `--lang all` (the default) when its
toolchain is absent — useful on a partial dev box.

### Not required to regenerate; required to **build** generated output

C++ is the one remaining "generated-only" language: it emits runnable
snippets but the smoke harness does not build or run them. If you want
to compile those snippets you'll need qpid-proton-cpp. See
`output/cpp/README.md` for specifics.

---

## What `make asyncapi-clients` actually does

The Make target is `uv run python -m asyncapi_clients.generator.generate`.
Internally that's four ordered steps:

1. **`step_python_sdk`** — runs `datamodel-code-generator` against
   `Schemas/` (Pydantic source-of-truth JSON schemas) to produce
   typed Pydantic v2 models in `output/python/sdk/`. Python only.
2. **`step_snippets`** — for each `(language, scenario)` pair, renders
   the matching Jinja template under
   `templates/<lang>/s{1..7}.<ext>.j2` to
   `output/<lang>/snippets/s{1..7}.<ext>`. Templates that don't exist
   for a language yet are skipped.
3. **`step_headers`** — prepends the header block to every
   emitted source file (snippets + SDK files). The `Date:` field
   comes from `_gen_date()` in `generate.py`, which reads
   `gen_stamp.json` (a SHA-256 of `asyncapi.yaml` paired with a date) so
   the stamp is a function of the spec's content. Header injection is idempotent;
   re-running over an already-headered file is a no-op.
4. **`step_readme`** — renders `templates/<lang>/readme.md.j2` to
   `output/<lang>/README.md` for every language that ships a
   readme template.

You can run the generator with flags for partial regen during
debugging:

```bash
# Just Python, all four steps:
uv run python -m asyncapi_clients.generator.generate --only-lang python

# All languages, but only the snippet-render step:
uv run python -m asyncapi_clients.generator.generate --only-step snippets

# Combined:
uv run python -m asyncapi_clients.generator.generate \
    --only-lang csharp --only-step snippets
```

Allowed `--only-step` values: `python_sdk`, `snippets`, `headers`,
`readme`.

---

## Per-language status + how to use each output

Each language gets a directory at `output/<lang>/` containing
generated `snippets/` and an integrator-facing `README.md`. Three
also have a smoke-harness build harness under
`smoketest/<lang>/` that compiles and runs the snippets against a
real broker.

### Smoke-validated languages (Python, Java, C#)

These three are exercised on every CI push. Adding a new envelope
convention or fixing a wire-format bug needs to land in *all three*
templates (or be flagged-but-not-failing via `XFAIL` in
`smoketest/run_all.py`).

| Language | Output | Build/Run | Smoke harness |
|---|---|---|---|
| **Python** | [`output/python/`](output/python/README.md) | `uv run python output/python/snippets/s1.py` | runs by default; no compile step |
| **Java** | [`output/java/`](output/java/README.md) | `mvn -f smoketest/java/pom.xml package` → `java -cp target/topas-snippets-shaded.jar S1` | qpid-jms-client 1.13.0 (NOT 2.x) — see [`smoketest/java/README.md`](smoketest/java/README.md) |
| **C#** | [`output/csharp/`](output/csharp/README.md) | `dotnet publish -c Release smoketest/csharp/TopasSnippets.csproj` → `dotnet bin/Release/net8.0/publish/TopasSnippets.dll S1` | AMQPNetLite.Core 2.4.11 + reflection dispatcher — see [`smoketest/csharp/README.md`](smoketest/csharp/README.md) |

For end-to-end manual testing against a local broker, see
[`smoketest/README.md`](smoketest/README.md). The same script the
smoketest README documents is what `make smoketest` runs.

### Generated-only language (C++)

Snippet templates exist and emit working-looking source, but no smoke
harness runs them — so they compile-and-run in CI for nobody. Treat
C++ as scaffolding; it is off the roadmap and no smoke slice
is planned.

| Language | Output | Hint |
|---|---|---|
| **C++** | [`output/cpp/`](output/cpp/README.md) | qpid-proton-cpp; CMake-based build expected if a smoke slice ever lands |

The languages with a live harness — Python, Java, C#, TypeScript and
Rust — are documented under
[`smoketest/`](smoketest/) and gated by the A11 conformance run.

Each per-language README lists the dependencies the integrator needs
to add, the env vars the snippet honours, and a `dotnet run` /
`mvn exec:java` / equivalent command to run a single scenario.

---

## Validating the output (smoke harness + drift CI)

Two complementary checks:

### Drift check — does the committed output match what the generator produces?

```bash
make asyncapi-clients
git diff --exit-code asyncapi_clients/output/
```

If the diff is non-empty, either someone hand-edited generated output
(forbidden) or someone forgot to commit a regen. Either way: regenerate
and commit. The CI workflow `AsyncAPI Clients Drift` enforces this on
every push.

### Smoke harness — do the snippets actually move bytes?

```bash
make smoketest
```

This:
1. Runs pre-flight checks for `mvn`, `javac`, `dotnet`.
2. Builds the Java uber-jar (`mvn package`).
3. Builds the C# DLL (`dotnet publish`).
4. Spins up Apache ActiveMQ Artemis 2.31.2 via Docker Compose.
5. Waits for broker health.
6. Runs `asyncapi_clients/smoketest/run_all.py --lang all` —
   21 scenarios total (7 Python + 7 Java + 7 C#).
7. Tears down the broker, even on failure.

The `run_all.py` orchestrator uses a `LANG_SCENARIOS: dict[str, list[Scenario]]`
registry. Adding a fourth language is one new entry per registry dict
plus a `smoketest/<lang>/` build directory.

To run only one language's leg locally (faster iteration):

```bash
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
    uv run python asyncapi_clients/smoketest/run_all.py --lang csharp
```

Allowed `--lang` values: `all` (default), `python`, `java`, `csharp`.
Missing toolchains under `--lang all` skip silently with a notice;
under a specific `--lang` they error out.

To run a single scenario manually for debugging, see
[`smoketest/README.md`](smoketest/README.md) (Python, plus the
shared docker-compose) and the per-language equivalents at
[`smoketest/java/README.md`](smoketest/java/README.md) and
[`smoketest/csharp/README.md`](smoketest/csharp/README.md).

---

## CI overview

Four workflows run on every push to `trunk`:

| Workflow | File | Purpose |
|---|---|---|
| **Build Diagrams** | `.github/workflows/build-and-publish.yml` | Runs the full `make schemas/asyncapi/validate/diagrams` pipeline — guards the diagram pipeline (Pages publish parked while the repo is private) |
| **AsyncAPI Clients Smoke Test** | `.github/workflows/asyncapi-clients-smoketest.yml` | `strategy.matrix.language: [python, java, csharp]` with `fail-fast: false`; per-leg Artemis service container; runs `run_all.py --lang ${{ matrix.language }}` |
| **AsyncAPI Clients Drift** | `.github/workflows/asyncapi-clients-drift.yml` | Regenerates and diffs against committed output; `fetch-depth: 0` so `_gen_date`'s git lookup works |
| **Lint JSON Schemas** | `.github/workflows/lint.yml` | Metaschema lint on PRs only |

A green push means: every snippet template produced output that
compiled, every smoke scenario received the expected bytes from a
real broker, and the committed output is byte-identical to a
fresh regeneration.

---

## Adding or extending a language

**To bring a generated-only language into the smoke harness** (the
"phase B → phase C" promotion):

1. Create `asyncapi_clients/smoketest/<lang>/` with whatever build
   manifest the language needs (e.g. `pom.xml` for Java,
   `TopasSnippets.csproj` for C#, `Cargo.toml` for Rust). Point its
   sources at `../../output/<lang>/snippets/` so the canonical
   generated snippets compile in place — no copy.
2. Add the build step to `Makefile`'s `smoketest:` target with a
   pre-flight `command -v` check for the toolchain.
3. Add three new entries to `asyncapi_clients/smoketest/run_all.py`:
   `LANG_SCENARIOS["<lang>"] = ...` (7 `Scenario` entries),
   `LANG_PREFIX["<lang>"] = "..."` (two-letter summary code),
   `LANG_TOOLCHAIN_PROBE["<lang>"] = ["<binary>", "--version"]`.
4. Add the language to the matrix in
   `.github/workflows/asyncapi-clients-smoketest.yml` and add a
   conditional `setup-<lang>` step gated on `matrix.language ==
   '<lang>'`.
5. Run `make smoketest` locally; expect to surface latent template
   bugs (the patterns to look for are
   filename casing, default address-routing, hyphenated AMQP
   property names, async/sync API quirks, dependency package-name
   drift). Fix the templates inside the slice if straightforward;
   `XFAIL` them and note why if not.
6. Commit template fixes + regenerated `output/<lang>/snippets/*` in
   the same commit (drift workflow contract).
7. Note any findings in the pull request description.

**To add a brand-new language** (an eighth):

1. Add the language code to `LANGUAGES` in `generate.py`.
2. Add an extension entry to `LANG_EXT`.
3. Author the seven `templates/<lang>/s{1..7}.<ext>.j2` templates
   plus `templates/<lang>/readme.md.j2` — the existing Python or
   Java templates are good shape references.
4. Add the comment-syntax entry to `_SYNTAX` in
   `asyncapi_clients/generator/headers.py` so the header
   block renders with the right comment delimiters.
5. Run `make asyncapi-clients`; `output/<lang>/` should now exist.
6. Then promote into the smoke harness as above.

---

## Where things live

| Path | What |
|---|---|
| `topas_3000_model/` (repo root) | **the source of truth** — Pydantic models for every domain. All wire-format changes start here |
| `bin/emit.py` | walks the Pydantic models and writes `Schemas/*.json` |
| `bin/emit_asyncapi.py` | combines `FILE_SPECS` (from `bin/emit.py`) + `asyncapi_template.yaml` to produce `asyncapi.yaml` |
| `asyncapi_template.yaml` (repo root) | hand-authored shell — server URL, security blocks, reusable message traits — that `bin/emit_asyncapi.py` expands into the full `asyncapi.yaml` |
| `asyncapi.yaml` (repo root) | **generated**, do not hand-edit — the input the asyncapi-clients generator reads |
| `asyncapi_clients/generator/generate.py` | the generator — pipeline orchestration, `_gen_date` |
| `asyncapi_clients/generator/headers.py` | header builder (per-language comment syntax) |
| `asyncapi_clients/generator/snippets.py` | Jinja rendering for snippet templates |
| `asyncapi_clients/templates/<lang>/` | the templates — edit here, never the output |
| `asyncapi_clients/output/<lang>/` | regenerated by `make asyncapi-clients`. Do not hand-edit |
| `asyncapi_clients/smoketest/run_all.py` | orchestrator — `--lang {all,python,java,csharp}`, `LANG_SCENARIOS` registry |
| `asyncapi_clients/smoketest/<lang>/` | per-language build harness (Java, C# only so far) |
| `asyncapi_clients/smoketest/README.md` | per-scenario manual run instructions |

