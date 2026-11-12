# asyncapi-clients C# / .NET smoke harness

.NET 8 build that compiles the generated C# snippets at
[`../../output/csharp/snippets/`](../../output/csharp/snippets/) in
place plus a hand-written `Program.cs` reflection dispatcher, and
produces a single `TopasSnippets.dll` that the orchestrator at
`../run_all.py` invokes per scenario as
`dotnet TopasSnippets.dll SN`.

This file documents how to run **one** C# snippet manually for
debugging — the `make smoketest` target drives all 7 in sequence
across all available languages. For Python and Java equivalents,
see [`../README.md`](../README.md) and
[`../java/README.md`](../java/README.md).

## Build

```bash
dotnet publish -c Release \
    asyncapi_clients/smoketest/csharp/TopasSnippets.csproj
```

Expected output: `Build succeeded.`. The DLL lands at
`bin/Release/net8.0/publish/TopasSnippets.dll`.

## Run a single C# scenario manually

Bring up the broker first (see `../README.md` for the docker compose
incantation). Then, from the repo root:

```bash
# S1 — subscribe-and-print 5 detector.measurement frames.
TOPAS_AMQP_URL=amqp://localhost:5672 TOPAS_MAX_MESSAGES=5 \
    dotnet asyncapi_clients/smoketest/csharp/bin/Release/net8.0/publish/TopasSnippets.dll S1

# In another terminal, fire the Python publisher to feed it.
PYTHONPATH=. TOPAS_AMQP_URL=amqp://localhost:5672 \
    uv run python asyncapi_clients/smoketest/publisher.py
```

Class names follow the snippet IDs: `S1`, `S2`, `S3`, `S4`, `S5`,
`S6`, `S7`. Each respects the `TOPAS_AMQP_URL` and (where
applicable) `TOPAS_MAX_MESSAGES`, `TOPAS_ORIGINATOR` env vars,
matching the Python and Java equivalents.

## Why a single .csproj + reflection dispatcher?

.NET projects allow only one entry point per assembly. Rather than
seven csproj files (one per scenario) or fiddling with
`-p:StartupObject=` on every invocation, `Program.cs` reads
`argv[0]` and reflects to the matching `S<N>.Main`. The build is one
`dotnet publish`; the runtime cost is one `MethodInfo.Invoke`.

The csproj's `<Compile Include="..\..\output\csharp\snippets\*.cs" />`
points back at the generated output so there is no copy. The
generated tree stays purely generated; this directory owns its own
build config.
