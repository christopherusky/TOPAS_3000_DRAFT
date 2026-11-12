/*
 * File:        sdk_stubs/csharp/tests/TaggedUnionTests.cs
 * Origin:      Hand-written regression test for the C# SDK
 * Changes:     Hand-edit here, not in output/ — copied into
 *              output/csharp/sdk/tests/ by step_sdk_stubs
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-29
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * Two things this guards, both of which were broken:
 *
 *  - `just-types-and-namespace` stripped [JsonProperty] and the
 *    per-enum converters, so the SDK serialised PascalCase names and enum
 *    members as their C# names — {"Kind":0} and later "Calibrate" rather
 *    than "calibrate". It could not produce valid wire JSON at all.
 *  - the union was flattened by quicktype into a single type
 *    carrying every variant's fields, so nothing distinguished a reset
 *    command from a calibrate one.
 */
using Newtonsoft.Json;
using Topas.Utmc.Detector;
using Xunit;

public class TaggedUnionTests
{
    private static string Command(string kind) =>
        $@"{{""kind"":""{kind}"",""commandCorrelationId"":8,
             ""detectorId"":""DET-2"",""requestTime"":""2026-08-28T12:00:00Z""}}";

    [Theory]
    [InlineData("reset", typeof(ResetDetectorCommand))]
    [InlineData("calibrate", typeof(CalibrateDetectorCommand))]
    public void The_discriminator_selects_the_variant(string kind, Type expected)
    {
        // These two variants have identical property sets; `kind` is the
        // only thing separating them.
        var command = UtmcJson.Deserialize<IDetectorCommand>(Command(kind));
        Assert.IsType(expected, command);
    }

    [Fact]
    public void Serialisation_uses_wire_names_and_wire_enum_values()
    {
        var command = UtmcJson.Deserialize<IDetectorCommand>(Command("calibrate"));
        string json = UtmcJson.Serialize(command);

        // camelCase property names, from [JsonProperty].
        Assert.Contains("\"commandCorrelationId\"", json);
        Assert.DoesNotContain("\"CommandCorrelationId\"", json);

        // The wire value, not the C# enum member name and not its ordinal.
        Assert.Contains("\"kind\":\"calibrate\"", json);
        Assert.DoesNotContain("\"Calibrate\"", json);
        Assert.DoesNotContain("\"kind\":0", json);
    }

    [Fact]
    public void An_unknown_discriminator_is_rejected()
    {
        Assert.Throws<JsonSerializationException>(
            () => UtmcJson.Deserialize<IDetectorCommand>(Command("explode")));
    }
}
