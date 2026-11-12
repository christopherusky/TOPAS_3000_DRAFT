/*
 * File:        output/csharp/snippets/s6.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
using System;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

using Amqp;
using Amqp.Framing;
using Amqp.Types;

/// <summary>
/// Scenario S6 — Discriminated-union consumer for transport-link.measurement.
///
/// Demonstrates: client-side dispatch on the <c>kind</c> discriminator
///. The TransportLinkMeasurement payload is one of three
/// variants — SCOOT, GPS, or ANPR — distinguished by the <c>kind</c>
/// field. The snippet reads the JSON, switches on the discriminator,
/// and dispatches to a per-variant handler.
///
/// This is the idiom for handling any of the discriminated unions in
/// TOPAS 3000A (F6 in the cross-cutting design log).
///
/// Tip: a typed SDK is not yet generated for C#. When it ships,
/// replace the <c>switch</c> below with a sealed record/abstract
/// hierarchy + pattern-matched dispatch over the generated variant
/// types.
/// </summary>
public class S6
{
    private const string Channel = "transport-link.measurement";

    public static async Task Main(string[] args)
    {
        string url = Environment.GetEnvironmentVariable("TOPAS_AMQP_URL")
            ?? "amqps://{host}:{port}".Replace("{host}", "localhost").Replace("{port}", "5671");
        int maxMessages = int.Parse(
            Environment.GetEnvironmentVariable("TOPAS_MAX_MESSAGES") ?? "5");

        var address = new Address(url);
        var connection = await Connection.Factory.CreateAsync(address);
        var session = new Session(connection);
        // See S1 for why the source needs the `topic` capability.
        var source = new Source
        {
            Address = Channel,
            Capabilities = new Symbol[] { new Symbol("topic") },
        };
        var receiver = new ReceiverLink(session, "s6-receiver", source, null);
        // Readiness handshake: the harness waits for this before
        // starting the publisher. Every publisher sends a fixed burst and
        // exits, so a message published before this link is attached is
        // discarded by the broker with no consumer to route it to.
        Console.Error.WriteLine("[ready]");

        try
        {
            for (int received = 0; received < maxMessages; received++)
            {
                Message msg = await receiver.ReceiveAsync(TimeSpan.FromSeconds(5));
                if (msg == null)
                {
                    Console.Error.WriteLine("[S6] Receive() timed out — broker idle? aborting.");
                    return;
                }

                string body = BodyAsString(msg);
                using JsonDocument doc = JsonDocument.Parse(body);
                Console.WriteLine($"[{received + 1}] {Handle(doc.RootElement)}");
                receiver.Accept(msg);
            }
        }
        finally
        {
            await receiver.CloseAsync();
            await session.CloseAsync();
            await connection.CloseAsync();
        }
    }

    /// <summary>Dispatch on the <c>kind</c> JSON discriminator.</summary>
    private static string Handle(JsonElement variant)
    {
        string kind = variant.TryGetProperty("kind", out JsonElement k)
            && k.ValueKind == JsonValueKind.String
                ? k.GetString()
                : string.Empty;

        return kind switch
        {
            "scoot" => $"SCOOT flow={Field(variant, "currentFlowVehicles")} occupancy%={Field(variant, "occupancyPercent")}",
            "gps" => $"GPS meanTravelTime={Field(variant, "meanTravelTimeSeconds")}s",
            "anpr" => $"ANPR in={Field(variant, "numberPlatesIn")} out={Field(variant, "numberPlatesOut")} matches={Field(variant, "numberPlatesMatches")}",
            _ => "unknown variant kind=" + (string.IsNullOrEmpty(kind) ? "<missing>" : kind),
        };
    }

    private static string Field(JsonElement node, string name)
    {
        if (!node.TryGetProperty(name, out JsonElement v)
            || v.ValueKind == JsonValueKind.Null
            || v.ValueKind == JsonValueKind.Undefined)
        {
            return "n/a";
        }
        return v.ValueKind == JsonValueKind.String ? v.GetString() : v.ToString();
    }

    private static string BodyAsString(Message msg)
    {
        object body = msg.Body;
        return body switch
        {
            string s => s,
            byte[] bytes => Encoding.UTF8.GetString(bytes),
            null => string.Empty,
            _ => body.ToString() ?? string.Empty,
        };
    }
}
