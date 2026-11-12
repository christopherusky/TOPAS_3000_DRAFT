/*
 * File:        output/java/sdk/com/topas/utmc/detector/DetectorType.java
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   quicktype@23.0.171 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
package com.topas.utmc.detector;

import com.fasterxml.jackson.annotation.*;
import java.util.Map;

/**
 * Detector type definitions.
 */
public class DetectorType {
    private String description;
    private Map<String, Object> extensions;
    private String notes;
    private String specificationLink;
    private String typeID;

    /**
     * Human-readable description of the detector kind.
     */
    @JsonProperty("description")
    public String getDescription() { return description; }
    @JsonProperty("description")
    public void setDescription(String value) { this.description = value; }

    /**
     * Vendor or local extensions. Open-content object.
     */
    @JsonProperty("extensions")
    public Map<String, Object> getExtensions() { return extensions; }
    @JsonProperty("extensions")
    public void setExtensions(Map<String, Object> value) { this.extensions = value; }

    /**
     * Additional notes, e.g. deployment details.
     */
    @JsonProperty("notes")
    public String getNotes() { return notes; }
    @JsonProperty("notes")
    public void setNotes(String value) { this.notes = value; }

    /**
     * URL to an external specification document for this detector type.
     */
    @JsonProperty("specificationLink")
    public String getSpecificationLink() { return specificationLink; }
    @JsonProperty("specificationLink")
    public void setSpecificationLink(String value) { this.specificationLink = value; }

    /**
     * Identifier of this detector type.
     */
    @JsonProperty("typeId")
    public String getTypeID() { return typeID; }
    @JsonProperty("typeId")
    public void setTypeID(String value) { this.typeID = value; }
}
