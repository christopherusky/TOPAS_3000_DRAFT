/*
 * File:        sdk_stubs/java/tests/TaggedUnionTest.java
 * Origin:      Hand-written regression test for the Java SDK
 * Changes:     Hand-edit here, not in output/ — copied into
 *              output/java/sdk/tests/ by step_sdk_stubs
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-28
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * Two things this guards, both of which were broken:
 *
 *  - every discriminator enum was generated as `Kind` from the
 *    property name and written to one Kind.java per package, so in a
 *    domain with two discriminated types the last one won. A detector
 *    command could not be deserialised at all —
 *    "Cannot deserialize Kind" — because the surviving enum held only the
 *    measurement values.
 *  - the union itself was flattened by quicktype into a single
 *    type carrying every variant's fields, so nothing distinguished a
 *    reset command from a calibrate one.
 */
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.topas.utmc.detector.DetectorCommand;
import org.junit.jupiter.api.Test;

public class TaggedUnionTest {

    private static final ObjectMapper MAPPER = new ObjectMapper().findAndRegisterModules();

    private static String command(String kind) {
        return "{\"kind\":\"" + kind + "\",\"commandCorrelationId\":7,"
                + "\"detectorId\":\"DET-1\",\"requestTime\":\"2026-08-28T12:00:00Z\"}";
    }

    @Test
    void reset_and_calibrate_are_distinguished() throws Exception {
        // These two variants have identical property sets; `kind` is the
        // only thing separating them.
        DetectorCommand reset = MAPPER.readValue(command("reset"), DetectorCommand.class);
        assertEquals("reset", reset.getKind());
        assertNotNull(reset.asResetDetectorCommand());
        assertNull(reset.asCalibrateDetectorCommand());

        DetectorCommand calibrate =
                MAPPER.readValue(command("calibrate"), DetectorCommand.class);
        assertEquals("calibrate", calibrate.getKind());
        assertNotNull(calibrate.asCalibrateDetectorCommand());
        assertNull(calibrate.asResetDetectorCommand());
    }

    @Test
    void the_discriminator_survives_a_round_trip() throws Exception {
        DetectorCommand calibrate =
                MAPPER.readValue(command("calibrate"), DetectorCommand.class);
        String json = MAPPER.writeValueAsString(calibrate);
        assertTrue(json.contains("\"kind\":\"calibrate\""),
                "round-tripped JSON lost its discriminator: " + json);
    }

    @Test
    void an_unknown_discriminator_is_rejected() {
        assertThrows(Exception.class,
                () -> MAPPER.readValue(command("explode"), DetectorCommand.class));
    }

    @Test
    void a_missing_discriminator_is_rejected() {
        assertThrows(Exception.class,
                () -> MAPPER.readValue("{\"detectorId\":\"DET-1\"}", DetectorCommand.class));
    }
}
