/*
 * File:        smoketest/csharp/Responder.cs
 * Origin:      Hand-written for asyncapi-clients conformance testing
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-28
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * A11 command responder — C# half of the cross-language matrix.
 *
 * The mirror image of S3: consume vms.command, read the requester's
 * reply-to, and answer *that node* with correlation-id mirroring the
 * request's message-id and commandCorrelationId carried through
 * unchanged. Every requester asserts exactly those three things.
 *
 * The sender uses an anonymous terminus — a Target with no Address — so
 * one link answers every reply node, with each message's `To` routing it
 * (A11.4).
 *
 * Configuration mirrors vms_responder.py so the harness can drive any
 * language's responder without knowing which it is.
 */
using System;
using System.Globalization;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Threading.Tasks;

using Amqp;
using Amqp.Framing;
using Amqp.Types;

public class Responder
{
    private const string CommandChannel = "vms.command";

    public static async Task Main(string[] args)
    {
        string url = Environment.GetEnvironmentVariable("TOPAS_AMQP_URL")
            ?? "amqp://localhost:5672";
        string[] progression = (Environment.GetEnvironmentVariable("TOPAS_RESPONDER_PROGRESSION")
            ?? "pending,pending,success").Split(',', StringSplitOptions.RemoveEmptyEntries);
        double delay = double.Parse(
            Environment.GetEnvironmentVariable("TOPAS_RESPONDER_DELAY") ?? "0.15",
            CultureInfo.InvariantCulture);
        string maxEnv = Environment.GetEnvironmentVariable("TOPAS_RESPONDER_MAX_REPLIES");
        int? maxReplies = string.IsNullOrEmpty(maxEnv) ? (int?)null : int.Parse(maxEnv);

        var connection = await Connection.Factory.CreateAsync(new Address(url));
        var session = new Session(connection);

        var source = new Source
        {
            Address = CommandChannel,
            Capabilities = new Symbol[] { new Symbol("topic") },
        };
        var receiver = new ReceiverLink(session, "responder-commands", source, null);

        // Anonymous terminus: a Target with no Address. Each message's `To`
        // routes it, so one link serves every reply node (A11.4).
        var sender = new SenderLink(session, "responder-replies", new Target(), null);

        // Readiness handshake, identical across languages.
        Console.WriteLine("[responder-ready] csharp (anonymous terminus)");
        Console.Out.Flush();

        int replied = 0;
        try
        {
            while (maxReplies == null || replied < maxReplies)
            {
                Message command = await receiver.ReceiveAsync(TimeSpan.FromSeconds(30));
                if (command == null)
                {
                    Console.Error.WriteLine("[responder] no command within 30s — exiting");
                    break;
                }

                string replyTo = command.Properties?.ReplyTo;
                if (string.IsNullOrEmpty(replyTo))
                {
                    // Under A11 there is nowhere to address the response.
                    // Loud, and does not answer.
                    Console.Error.WriteLine("[responder] FATAL: command carried no reply-to; "
                        + "under A11 there is nowhere to address the response");
                    receiver.Accept(command);
                    break;
                }
                // A11: correlation-id mirrors the request's message-id. Kept
                // as the AMQP value rather than a string rendering, so a
                // non-string identifier survives the round trip.
                // GetMessageId(), not the MessageId property: AMQPNetLite
                // exposes both, and the property is typed `string` — a
                // lossy convenience over an AMQP identifier that may be a
                // ulong, uuid or binary. The Get/Set pair carries the union
                // as `object`.
                object requestMessageId = command.Properties?.GetMessageId();

                // Decoding does not degrade: an unrecognised body is a
                // visible failure rather than a placeholder.
                JsonNode body = JsonNode.Parse(BodyAsString(command));
                JsonNode appId = body?["commandCorrelationId"];
                receiver.Accept(command);

                for (int i = 0; i < progression.Length; i++)
                {
                    var frame = new JsonObject
                    {
                        ["vmsId"] = body?["vmsId"]?.DeepClone(),
                        // Carried through unchanged, at the top level
                        // rather than buried in an echo.
                        ["commandCorrelationId"] = appId?.DeepClone(),
                        ["status"] = progression[i],
                        ["responseTime"] = DateTime.UtcNow.ToString("o"),
                        ["echo"] = body?.DeepClone(),
                    };

                    var reply = new Message
                    {
                        // A Data section — see the note in s3.cs. The
                        // `new Message(object)` constructor would emit an
                        // AmqpValue containing binary, which fe2o3-amqp's
                        // typed receive cannot decode.
                        BodySection = new Data
                        {
                            Binary = Encoding.UTF8.GetBytes(frame.ToJsonString()),
                        },
                        Properties = new Properties
                        {
                            To = replyTo,
                            ContentType = "application/json",
                        },
                    };
                    // SetCorrelationId(object) rather than the string
                    // property, so the request's identifier type survives
                    // into the response.
                    reply.Properties.SetCorrelationId(requestMessageId);
                    await sender.SendAsync(reply);
                    Console.WriteLine(
                        $"[responder] {progression[i]} -> {replyTo} corr={requestMessageId}");
                    Console.Out.Flush();

                    if (i < progression.Length - 1 && delay > 0)
                    {
                        await Task.Delay(TimeSpan.FromSeconds(delay));
                    }
                }
                replied++;
            }
        }
        finally
        {
            await sender.CloseAsync();
            await receiver.CloseAsync();
            await session.CloseAsync();
            await connection.CloseAsync();
        }
    }

    /// <summary>
    /// An explicit compatibility layer over the body representations
    /// clients produce. Throws rather than degrading — a conformance
    /// responder that turns an unrecognised body into a placeholder loses
    /// the field under test.
    /// </summary>
    private static string BodyAsString(Message msg)
    {
        object body = msg.Body;
        return body switch
        {
            string s => s,
            byte[] bytes => Encoding.UTF8.GetString(bytes),
            _ => throw new InvalidOperationException(
                $"unhandled body representation {body?.GetType().Name ?? "null"}; "
                + "add it here rather than degrading to a placeholder"),
        };
    }
}
