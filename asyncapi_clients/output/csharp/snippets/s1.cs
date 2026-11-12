/*
 * File:        output/csharp/snippets/s1.cs
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
/// Scenario S1 — Subscribe to detector.measurement and print each message.
///
/// Demonstrates: connect over AMQP 1.0 + mTLS, subscribe to a single
/// channel, parse the JSON payload, print, exit after the first N
/// messages.
///
/// Tip: a typed SDK is not yet generated for C# (deferred
/// to a follow-up sub-project); deserialize the JSON to a record/POCO
/// of your choosing — System.Text.Json's <c>JsonSerializer</c> is
/// the usual bridge once the SDK ships.
/// </summary>
public class S1
{
    private const string Channel = "detector.measurement";

    public static async Task Main(string[] args)
    {
        string url = Environment.GetEnvironmentVariable("TOPAS_AMQP_URL")
            ?? "amqps://{host}:{port}".Replace("{host}", "localhost").Replace("{port}", "5671");
        int maxMessages = int.Parse(
            Environment.GetEnvironmentVariable("TOPAS_MAX_MESSAGES") ?? "5");

        var address = new Address(url);
        var connection = await Connection.Factory.CreateAsync(address);
        var session = new Session(connection);
        // Mark the AMQP source with the `topic` capability so Artemis
        // binds the receiver to a MULTICAST queue under the
        // auto-created address. Without this the default Source has no
        // capability set and Artemis falls back to ANYCAST, which
        // doesn't match the publisher's multicast routing — messages
        // would never reach this subscriber.
        //
        // The receive loop below also uses ReceiveAsync (not the
        // synchronous Receive overload) on purpose: the sync API
        // blocks the AMQPNetLite read pump on .NET 8, which means the
        // attach response and inbound transfers don't get processed
        // until the consumer detaches. await ReceiveAsync(...) yields
        // back to the I/O loop so frames keep flowing.
        var source = new Source
        {
            Address = Channel,
            Capabilities = new Symbol[] { new Symbol("topic") },
        };
        var receiver = new ReceiverLink(session, "s1-receiver", source, null);
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
                    Console.Error.WriteLine("[S1] Receive() timed out — broker idle? aborting.");
                    return;
                }

                string body = BodyAsString(msg);
                using JsonDocument doc = JsonDocument.Parse(body);
                Console.WriteLine($"[{received + 1}] {doc.RootElement}");
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

    /// <summary>
    /// Read the AMQP message body as a UTF-8 string. TOPAS 3000A uses
    /// JSON in the AMQP value/data section by default (A1); tolerant
    /// readers across <see cref="byte"/>[], <see cref="string"/> and
    /// generic object payloads are cheap insurance.
    /// </summary>
    internal static string BodyAsString(Message msg)
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
