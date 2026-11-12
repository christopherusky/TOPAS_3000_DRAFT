/*
 * File:        output/csharp/snippets/s4.cs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

using Amqp;
using Amqp.Framing;
using Amqp.Types;

/// <summary>
/// Scenario S4 — Distinguish sync vs delta messages and reassemble a sync batch.
///
/// Demonstrates: reading the x-message-mode AMQP application
/// property (A3) to tell snapshot data from change events; using the
/// x-sync-id / x-sync-sequence /
/// x-sync-total triple (A5) to detect when a sync batch is
/// complete.
///
/// Sync progress is tracked in a
/// <c>Dictionary&lt;string, HashSet&lt;int&gt;&gt;</c> keyed by
/// x-sync-id; each set entry corresponds to one
/// x-sync-sequence value seen on the wire.
/// </summary>
public class S4
{
    private const string Channel = "detector.measurement";

    public static async Task Main(string[] args)
    {
        string url = Environment.GetEnvironmentVariable("TOPAS_AMQP_URL")
            ?? "amqps://{host}:{port}".Replace("{host}", "localhost").Replace("{port}", "5671");
        int maxMessages = int.Parse(
            Environment.GetEnvironmentVariable("TOPAS_MAX_MESSAGES") ?? "20");

        var address = new Address(url);
        var connection = await Connection.Factory.CreateAsync(address);
        var session = new Session(connection);
        // See S1 for why the source needs the `topic` capability.
        var source = new Source
        {
            Address = Channel,
            Capabilities = new Symbol[] { new Symbol("topic") },
        };
        var receiver = new ReceiverLink(session, "s4-receiver", source, null);
        // Readiness handshake: the harness waits for this before
        // starting the publisher. Every publisher sends a fixed burst and
        // exits, so a message published before this link is attached is
        // discarded by the broker with no consumer to route it to.
        Console.Error.WriteLine("[ready]");

        // sync-id -> set of sequence numbers received so far.
        var syncProgress = new Dictionary<string, HashSet<int>>();

        try
        {
            for (int received = 0; received < maxMessages; received++)
            {
                Message msg = await receiver.ReceiveAsync(TimeSpan.FromSeconds(5));
                if (msg == null)
                {
                    Console.Error.WriteLine("[S4] Receive() timed out — broker idle? aborting.");
                    return;
                }

                string mode = GetStringProperty(msg, "x-message-mode") ?? "delta";
                string body = BodyAsString(msg);
                using JsonDocument doc = JsonDocument.Parse(body);

                if (mode == "delta")
                {
                    Console.WriteLine($"[delta] {doc.RootElement}");
                    receiver.Accept(msg);
                    continue;
                }

                // mode == "sync" — track batch completion.
                string syncId = GetStringProperty(msg, "x-sync-id");
                int seq = GetIntProperty(msg, "x-sync-sequence") ?? 0;
                int total = GetIntProperty(msg, "x-sync-total") ?? 0;

                if (syncId == null || total == 0)
                {
                    Console.WriteLine($"[sync*] {doc.RootElement}  (incomplete sync metadata)");
                    receiver.Accept(msg);
                    continue;
                }

                if (!syncProgress.TryGetValue(syncId, out HashSet<int> seen))
                {
                    seen = new HashSet<int>();
                    syncProgress[syncId] = seen;
                }
                seen.Add(seq);
                int progress = seen.Count;
                Console.WriteLine($"[sync {syncId} {progress}/{total}] {doc.RootElement}");
                if (progress == total)
                {
                    Console.WriteLine($"[sync {syncId}] complete");
                    syncProgress.Remove(syncId);
                }
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

    private static string GetStringProperty(Message msg, string key)
    {
        object v = msg.ApplicationProperties?[key];
        return v?.ToString();
    }

    private static int? GetIntProperty(Message msg, string key)
    {
        object v = msg.ApplicationProperties?[key];
        if (v == null) return null;
        return v switch
        {
            int i => i,
            long l => (int)l,
            string s when int.TryParse(s, out int n) => n,
            _ => Convert.ToInt32(v),
        };
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
