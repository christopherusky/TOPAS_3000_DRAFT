/*
 * File:        smoketest/csharp/Program.cs
 * Origin:      Hand-written dispatcher for the asyncapi-clients C# smoke harness
 * Changes:     Hand-edit allowed — this is a test harness, not generated output
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-05-04
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * Invoked by run_all.py as `dotnet TopasSnippets.dll SN` — picks the
 * S1..S7 class by reflection and awaits its async Main(string[]).
 */
using System;
using System.Reflection;
using System.Threading.Tasks;

internal static class Program
{
    public static async Task<int> Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine(
                "[dispatcher] usage: dotnet TopasSnippets.dll <S1|S2|S3|S4|S5|S6|S7>");
            return 2;
        }

        string scenarioName = args[0];
        Type scenarioType = Type.GetType(scenarioName);
        if (scenarioType == null)
        {
            Console.Error.WriteLine(
                $"[dispatcher] no class named '{scenarioName}' in this assembly");
            return 2;
        }

        MethodInfo mainMethod = scenarioType.GetMethod(
            "Main",
            BindingFlags.Public | BindingFlags.Static);
        if (mainMethod == null)
        {
            Console.Error.WriteLine(
                $"[dispatcher] {scenarioName} has no public static Main(...)");
            return 2;
        }

        // The generated snippets declare `public static async Task Main(string[] args)`.
        // Invoke and await; let any inner exception surface to stderr with the original
        // stack trace (don't wrap in AggregateException).
        try
        {
            object invoked = mainMethod.Invoke(null, new object[] { Array.Empty<string>() });
            if (invoked is Task task)
            {
                await task.ConfigureAwait(false);
            }
            return 0;
        }
        catch (TargetInvocationException tie) when (tie.InnerException != null)
        {
            Console.Error.WriteLine(
                $"[dispatcher] {scenarioName}.Main threw: {tie.InnerException}");
            return 1;
        }
    }
}
