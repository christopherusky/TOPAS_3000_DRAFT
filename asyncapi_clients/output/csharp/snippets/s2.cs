/*
 * File:        output/csharp/snippets/s2.cs
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
/// Scenario S2 — Wildcard subscription across a domain.
///
/// Demonstrates: subscribing to *.fault (every
/// fault from every device domain) using AMQP 1.0 source addressing.
/// Teaches A8 (channel naming convention).
///
/// Wildcard syntax is broker-specific. AMQPNetLite passes the source
/// address through verbatim, so the exact pattern depends on the
/// broker:
///   - Apache ActiveMQ Artemis treats <c>*.fault</c> as an
///     address-pattern.
///   - Apache Qpid Dispatch Router uses the multicast prefix
///     (e.g. <c>mc/*.fault</c>).
/// Adjust to your deployment.
/// </summary>
public class S2
{
    private const string Pattern = "*.fault";

    public static async Task Main(string[] args)
    {
        string url = Environment.GetEnvironmentVariable("TOPAS_AMQP_URL")
            ?? "amqps://{host}:{port}".Replace("{host}", "localhost").Replace("{port}", "5671");
        int maxMessages = int.Parse(
            Environment.GetEnvironmentVariable("TOPAS_MAX_MESSAGES") ?? "5");

        var address = new Address(url);
        var connection = await Connection.Factory.CreateAsync(address);
        var session = new Session(connection);
        // Wildcard subscriptions also need the `topic` source capability
        // so Artemis routes via MULTICAST. See S1 for the long-form
        // explanation.
        var source = new Source
        {
            Address = Pattern,
            Capabilities = new Symbol[] { new Symbol("topic") },
        };
        var receiver = new ReceiverLink(session, "s2-receiver", source, null);
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
                    Console.Error.WriteLine("[S2] Receive() timed out — broker idle? aborting.");
                    return;
                }

                string body = BodyAsString(msg);
                using JsonDocument doc = JsonDocument.Parse(body);

                // Concrete address arrives in the message Properties.To
                // (set by the broker on delivery for wildcard sources).
                string concreteAddress = msg.Properties?.To ?? Pattern;
                int dotIdx = concreteAddress.IndexOf('.');
                string domain = dotIdx > 0
                    ? concreteAddress.Substring(0, dotIdx)
                    : concreteAddress;

                Console.WriteLine($"[fault from {domain}] {doc.RootElement}");
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
