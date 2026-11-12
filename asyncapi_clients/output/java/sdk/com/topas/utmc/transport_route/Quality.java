/*
 * File:        output/java/sdk/com/topas/utmc/transport_route/Quality.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.transport_route;

import com.fasterxml.jackson.annotation.*;
import java.time.OffsetDateTime;
import java.util.Map;

/**
 * Optional quality statement.
 *
 * Quality statement attached to a UTMC data object or measurement.
 */
public class Quality {
    private Map<String, Object> extensions;
    private OffsetDateTime lastUpdated;
    private String participantID;
    private long qualityID;
    private String sourceName;
    private String sourceType;

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * When this quality statement was last updated.
     */
    @JsonProperty("lastUpdated")
    public OffsetDateTime getLastUpdated() { return lastUpdated; }
    @JsonProperty("lastUpdated")
    public void setLastUpdated(OffsetDateTime value) { this.lastUpdated = value; }

    /**
     * Identifier of the UTMC participant system that owns this quality statement.
     */
    @JsonProperty("participantId")
    public String getParticipantID() { return participantID; }
    @JsonProperty("participantId")
    public void setParticipantID(String value) { this.participantID = value; }

    /**
     * Identifier of this Quality statement.
     */
    @JsonProperty("qualityId")
    public long getQualityID() { return qualityID; }
    @JsonProperty("qualityId")
    public void setQualityID(long value) { this.qualityID = value; }

    /**
     * Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no
     * source cannot be interpreted.
     */
    @JsonProperty("sourceName")
    public String getSourceName() { return sourceName; }
    @JsonProperty("sourceName")
    public void setSourceName(String value) { this.sourceName = value; }

    /**
     * Free-text description of the source kind.
     */
    @JsonProperty("sourceType")
    public String getSourceType() { return sourceType; }
    @JsonProperty("sourceType")
    public void setSourceType(String value) { this.sourceType = value; }
}
