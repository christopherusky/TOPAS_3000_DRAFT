/*
 * File:        output/rust/sdk/src/traffic_event.rs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   typify@0.7.0 + asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Accident traffic event payload."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Event Accident\","]
#[doc = "  \"description\": \"Accident traffic event payload.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"accidentType\","]
#[doc = "    \"eventId\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"accidentTime\": {"]
#[doc = "      \"description\": \"When the accident was reported. Optional: unknown when first reported.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"accidentType\": {"]
#[doc = "      \"description\": \"Classification of the accident.\","]
#[doc = "      \"$ref\": \"#/$defs/AccidentType\""]
#[doc = "    },"]
#[doc = "    \"confirmedBy\": {"]
#[doc = "      \"description\": \"Who confirmed.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"diversionInForce\": {"]
#[doc = "      \"description\": \"Whether diversion is active.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"diversionRoute\": {"]
#[doc = "      \"description\": \"Diversion route description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"endTime\": {"]
#[doc = "      \"description\": \"When cleared.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this event.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"externalCountryCode\": {"]
#[doc = "      \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"externalSituationId\": {"]
#[doc = "      \"description\": \"Cross-ref situation identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"externalSituationVersion\": {"]
#[doc = "      \"description\": \"Cross-ref situation version. Requires external_situation_id.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"externalSystemCode\": {"]
#[doc = "      \"description\": \"Cross-ref system code.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lanesAffected\": {"]
#[doc = "      \"description\": \"Lane closure pattern (I=open, T=closed).\","]
#[doc = "      \"$ref\": \"#/$defs/LanesAffectedType\""]
#[doc = "    },"]
#[doc = "    \"lanesOperational\": {"]
#[doc = "      \"description\": \"Count of operational lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lanesRestricted\": {"]
#[doc = "      \"description\": \"Count of closed/restricted lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lightConditions\": {"]
#[doc = "      \"description\": \"Light conditions at time.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"WGS84 point location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"locationDescription\": {"]
#[doc = "      \"description\": \"Free-text location description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modifiedBy\": {"]
#[doc = "      \"description\": \"Who last modified.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"Event phase (unconstrained).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"publicDescription\": {"]
#[doc = "      \"description\": \"Public-facing summary for journey planners.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reportedBy\": {"]
#[doc = "      \"description\": \"Who reported the event.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"roadConditions\": {"]
#[doc = "      \"description\": \"Road surface conditions.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Event severity.\","]
#[doc = "      \"$ref\": \"#/$defs/SeverityType\""]
#[doc = "    },"]
#[doc = "    \"statisticDate\": {"]
#[doc = "      \"description\": \"When statistics were collected.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventStatus\""]
#[doc = "    },"]
#[doc = "    \"vehicleCount\": {"]
#[doc = "      \"description\": \"Number of vehicles involved.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"weatherConditions\": {"]
#[doc = "      \"description\": \"Weather conditions.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentRequired\": {"]
#[doc = "    \"externalSituationVersion\": ["]
#[doc = "      \"externalSituationId\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AccidentEvent {
    #[doc = "When the accident was reported. Optional: unknown when first reported."]
    #[serde(
        rename = "accidentTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accident_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Classification of the accident."]
    #[serde(rename = "accidentType")]
    pub accident_type: AccidentType,
    #[doc = "Who confirmed."]
    #[serde(
        rename = "confirmedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confirmed_by: ::std::option::Option<::std::string::String>,
    #[doc = "Whether diversion is active."]
    #[serde(
        rename = "diversionInForce",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_in_force: ::std::option::Option<bool>,
    #[doc = "Diversion route description."]
    #[serde(
        rename = "diversionRoute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_route: ::std::option::Option<::std::string::String>,
    #[doc = "When cleared."]
    #[serde(
        rename = "endTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Unique identifier of this event."]
    #[serde(rename = "eventId")]
    pub event_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
    #[serde(
        rename = "externalCountryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_country_code: ::std::option::Option<AccidentEventExternalCountryCode>,
    #[doc = "Cross-ref situation identifier."]
    #[serde(
        rename = "externalSituationId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Cross-ref situation version. Requires external_situation_id."]
    #[serde(
        rename = "externalSituationVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_version: ::std::option::Option<u64>,
    #[doc = "Cross-ref system code."]
    #[serde(
        rename = "externalSystemCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_system_code: ::std::option::Option<::std::string::String>,
    #[doc = "Lane closure pattern (I=open, T=closed)."]
    #[serde(
        rename = "lanesAffected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_affected: ::std::option::Option<LanesAffectedType>,
    #[doc = "Count of operational lanes."]
    #[serde(
        rename = "lanesOperational",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_operational: ::std::option::Option<u64>,
    #[doc = "Count of closed/restricted lanes."]
    #[serde(
        rename = "lanesRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_restricted: ::std::option::Option<u64>,
    #[doc = "Light conditions at time."]
    #[serde(
        rename = "lightConditions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub light_conditions: ::std::option::Option<::std::string::String>,
    #[doc = "WGS84 point location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Free-text location description."]
    #[serde(
        rename = "locationDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who last modified."]
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event phase (unconstrained)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<::std::string::String>,
    #[doc = "Public-facing summary for journey planners."]
    #[serde(
        rename = "publicDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who reported the event."]
    #[serde(
        rename = "reportedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reported_by: ::std::option::Option<::std::string::String>,
    #[doc = "Road surface conditions."]
    #[serde(
        rename = "roadConditions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub road_conditions: ::std::option::Option<::std::string::String>,
    #[doc = "Event severity."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<SeverityType>,
    #[doc = "When statistics were collected."]
    #[serde(
        rename = "statisticDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub statistic_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub status: EventStatus,
    #[doc = "Number of vehicles involved."]
    #[serde(
        rename = "vehicleCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vehicle_count: ::std::option::Option<u64>,
    #[doc = "Weather conditions."]
    #[serde(
        rename = "weatherConditions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weather_conditions: ::std::option::Option<::std::string::String>,
}
impl AccidentEvent {
    pub fn builder() -> builder::AccidentEvent {
        Default::default()
    }
}
#[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AccidentEventExternalCountryCode(::std::string::String);
impl ::std::ops::Deref for AccidentEventExternalCountryCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AccidentEventExternalCountryCode> for ::std::string::String {
    fn from(value: AccidentEventExternalCountryCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AccidentEventExternalCountryCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AccidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AccidentEventExternalCountryCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Classification of accident (20 values).\n\n18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,\nwhich has no XSD code. XSD codes 5 (\"Accident History\") and 20 (\"Vehicle\nspun around\") are not carried."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Classification of accident (20 values).\\n\\n18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,\\nwhich has no XSD code. XSD codes 5 (\\\"Accident History\\\") and 20 (\\\"Vehicle\\nspun around\\\") are not carried.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"accident\","]
#[doc = "    \"busAccident\","]
#[doc = "    \"collision\","]
#[doc = "    \"investigationWork\","]
#[doc = "    \"chemicalSpillage\","]
#[doc = "    \"fuelSpillage\","]
#[doc = "    \"hazardousMaterials\","]
#[doc = "    \"heavyLorry\","]
#[doc = "    \"jackKnifedArticulatedLorry\","]
#[doc = "    \"jackKnifedCaravan\","]
#[doc = "    \"jackKnifedTrailer\","]
#[doc = "    \"multiVehicle\","]
#[doc = "    \"oilSpillage\","]
#[doc = "    \"overturnedHeavyLorry\","]
#[doc = "    \"overturnedTrailer\","]
#[doc = "    \"overturnedVehicle\","]
#[doc = "    \"secondaryAccident\","]
#[doc = "    \"seriousAccident\","]
#[doc = "    \"shedLoad\","]
#[doc = "    \"other\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccidentType {
    #[serde(rename = "accident")]
    Accident,
    #[serde(rename = "busAccident")]
    BusAccident,
    #[serde(rename = "collision")]
    Collision,
    #[serde(rename = "investigationWork")]
    InvestigationWork,
    #[serde(rename = "chemicalSpillage")]
    ChemicalSpillage,
    #[serde(rename = "fuelSpillage")]
    FuelSpillage,
    #[serde(rename = "hazardousMaterials")]
    HazardousMaterials,
    #[serde(rename = "heavyLorry")]
    HeavyLorry,
    #[serde(rename = "jackKnifedArticulatedLorry")]
    JackKnifedArticulatedLorry,
    #[serde(rename = "jackKnifedCaravan")]
    JackKnifedCaravan,
    #[serde(rename = "jackKnifedTrailer")]
    JackKnifedTrailer,
    #[serde(rename = "multiVehicle")]
    MultiVehicle,
    #[serde(rename = "oilSpillage")]
    OilSpillage,
    #[serde(rename = "overturnedHeavyLorry")]
    OverturnedHeavyLorry,
    #[serde(rename = "overturnedTrailer")]
    OverturnedTrailer,
    #[serde(rename = "overturnedVehicle")]
    OverturnedVehicle,
    #[serde(rename = "secondaryAccident")]
    SecondaryAccident,
    #[serde(rename = "seriousAccident")]
    SeriousAccident,
    #[serde(rename = "shedLoad")]
    ShedLoad,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for AccidentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accident => f.write_str("accident"),
            Self::BusAccident => f.write_str("busAccident"),
            Self::Collision => f.write_str("collision"),
            Self::InvestigationWork => f.write_str("investigationWork"),
            Self::ChemicalSpillage => f.write_str("chemicalSpillage"),
            Self::FuelSpillage => f.write_str("fuelSpillage"),
            Self::HazardousMaterials => f.write_str("hazardousMaterials"),
            Self::HeavyLorry => f.write_str("heavyLorry"),
            Self::JackKnifedArticulatedLorry => f.write_str("jackKnifedArticulatedLorry"),
            Self::JackKnifedCaravan => f.write_str("jackKnifedCaravan"),
            Self::JackKnifedTrailer => f.write_str("jackKnifedTrailer"),
            Self::MultiVehicle => f.write_str("multiVehicle"),
            Self::OilSpillage => f.write_str("oilSpillage"),
            Self::OverturnedHeavyLorry => f.write_str("overturnedHeavyLorry"),
            Self::OverturnedTrailer => f.write_str("overturnedTrailer"),
            Self::OverturnedVehicle => f.write_str("overturnedVehicle"),
            Self::SecondaryAccident => f.write_str("secondaryAccident"),
            Self::SeriousAccident => f.write_str("seriousAccident"),
            Self::ShedLoad => f.write_str("shedLoad"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for AccidentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accident" => Ok(Self::Accident),
            "busAccident" => Ok(Self::BusAccident),
            "collision" => Ok(Self::Collision),
            "investigationWork" => Ok(Self::InvestigationWork),
            "chemicalSpillage" => Ok(Self::ChemicalSpillage),
            "fuelSpillage" => Ok(Self::FuelSpillage),
            "hazardousMaterials" => Ok(Self::HazardousMaterials),
            "heavyLorry" => Ok(Self::HeavyLorry),
            "jackKnifedArticulatedLorry" => Ok(Self::JackKnifedArticulatedLorry),
            "jackKnifedCaravan" => Ok(Self::JackKnifedCaravan),
            "jackKnifedTrailer" => Ok(Self::JackKnifedTrailer),
            "multiVehicle" => Ok(Self::MultiVehicle),
            "oilSpillage" => Ok(Self::OilSpillage),
            "overturnedHeavyLorry" => Ok(Self::OverturnedHeavyLorry),
            "overturnedTrailer" => Ok(Self::OverturnedTrailer),
            "overturnedVehicle" => Ok(Self::OverturnedVehicle),
            "secondaryAccident" => Ok(Self::SecondaryAccident),
            "seriousAccident" => Ok(Self::SeriousAccident),
            "shedLoad" => Ok(Self::ShedLoad),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccidentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccidentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccidentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`AllTypes`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"_AllTypes\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"AccidentEvent\": {"]
#[doc = "      \"$ref\": \"#/$defs/AccidentEvent\""]
#[doc = "    },"]
#[doc = "    \"AccidentType\": {"]
#[doc = "      \"$ref\": \"#/$defs/AccidentType\""]
#[doc = "    },"]
#[doc = "    \"Coordinates\": {"]
#[doc = "      \"$ref\": \"#/$defs/Coordinates\""]
#[doc = "    },"]
#[doc = "    \"EventStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventStatus\""]
#[doc = "    },"]
#[doc = "    \"IncidentEvent\": {"]
#[doc = "      \"$ref\": \"#/$defs/IncidentEvent\""]
#[doc = "    },"]
#[doc = "    \"IncidentType\": {"]
#[doc = "      \"$ref\": \"#/$defs/IncidentType\""]
#[doc = "    },"]
#[doc = "    \"LanesAffectedType\": {"]
#[doc = "      \"$ref\": \"#/$defs/LanesAffectedType\""]
#[doc = "    },"]
#[doc = "    \"PlannedEvent\": {"]
#[doc = "      \"$ref\": \"#/$defs/PlannedEvent\""]
#[doc = "    },"]
#[doc = "    \"PlannedEventType\": {"]
#[doc = "      \"$ref\": \"#/$defs/PlannedEventType\""]
#[doc = "    },"]
#[doc = "    \"Point\": {"]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"RoadworksEvent\": {"]
#[doc = "      \"$ref\": \"#/$defs/RoadworksEvent\""]
#[doc = "    },"]
#[doc = "    \"RoadworksType\": {"]
#[doc = "      \"$ref\": \"#/$defs/RoadworksType\""]
#[doc = "    },"]
#[doc = "    \"SeverityType\": {"]
#[doc = "      \"$ref\": \"#/$defs/SeverityType\""]
#[doc = "    },"]
#[doc = "    \"SyncCategoryEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncCategoryEnum\""]
#[doc = "    },"]
#[doc = "    \"SyncComplete\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncComplete\""]
#[doc = "    },"]
#[doc = "    \"SyncRequest\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncRequest\""]
#[doc = "    },"]
#[doc = "    \"SyncStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncStatus\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AllTypes {
    #[serde(
        rename = "AccidentEvent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accident_event: ::std::option::Option<AccidentEvent>,
    #[serde(
        rename = "AccidentType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accident_type: ::std::option::Option<AccidentType>,
    #[serde(
        rename = "Coordinates",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub coordinates: ::std::option::Option<Coordinates>,
    #[serde(
        rename = "EventStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event_status: ::std::option::Option<EventStatus>,
    #[serde(
        rename = "IncidentEvent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub incident_event: ::std::option::Option<IncidentEvent>,
    #[serde(
        rename = "IncidentType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub incident_type: ::std::option::Option<IncidentType>,
    #[serde(
        rename = "LanesAffectedType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_affected_type: ::std::option::Option<LanesAffectedType>,
    #[serde(
        rename = "PlannedEvent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_event: ::std::option::Option<PlannedEvent>,
    #[serde(
        rename = "PlannedEventType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_event_type: ::std::option::Option<PlannedEventType>,
    #[serde(
        rename = "Point",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub point: ::std::option::Option<Point>,
    #[serde(
        rename = "RoadworksEvent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub roadworks_event: ::std::option::Option<RoadworksEvent>,
    #[serde(
        rename = "RoadworksType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub roadworks_type: ::std::option::Option<RoadworksType>,
    #[serde(
        rename = "SeverityType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_type: ::std::option::Option<SeverityType>,
    #[serde(
        rename = "SyncCategoryEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_category_enum: ::std::option::Option<SyncCategoryEnum>,
    #[serde(
        rename = "SyncComplete",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_complete: ::std::option::Option<SyncComplete>,
    #[serde(
        rename = "SyncRequest",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_request: ::std::option::Option<SyncRequest>,
    #[serde(
        rename = "SyncStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_status: ::std::option::Option<SyncStatus>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            accident_event: Default::default(),
            accident_type: Default::default(),
            coordinates: Default::default(),
            event_status: Default::default(),
            incident_event: Default::default(),
            incident_type: Default::default(),
            lanes_affected_type: Default::default(),
            planned_event: Default::default(),
            planned_event_type: Default::default(),
            point: Default::default(),
            roadworks_event: Default::default(),
            roadworks_type: Default::default(),
            severity_type: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
        }
    }
}
impl AllTypes {
    pub fn builder() -> builder::AllTypes {
        Default::default()
    }
}
#[doc = "A WGS84 geographic coordinate pair (longitude, latitude)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A WGS84 geographic coordinate pair (longitude, latitude).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"latitude\","]
#[doc = "    \"longitude\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"latitude\": {"]
#[doc = "      \"description\": \"Latitude in decimal degrees (WGS84).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 90.0,"]
#[doc = "      \"minimum\": -90.0"]
#[doc = "    },"]
#[doc = "    \"longitude\": {"]
#[doc = "      \"description\": \"Longitude in decimal degrees (WGS84).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 180.0,"]
#[doc = "      \"minimum\": -180.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Coordinates {
    #[doc = "Latitude in decimal degrees (WGS84)."]
    pub latitude: f64,
    #[doc = "Longitude in decimal degrees (WGS84)."]
    pub longitude: f64,
}
impl Coordinates {
    pub fn builder() -> builder::Coordinates {
        Default::default()
    }
}
#[doc = "Event lifecycle state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Event lifecycle state.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"active\","]
#[doc = "    \"planned\","]
#[doc = "    \"suspended\","]
#[doc = "    \"cleared\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "cleared")]
    Cleared,
}
impl ::std::fmt::Display for EventStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Planned => f.write_str("planned"),
            Self::Suspended => f.write_str("suspended"),
            Self::Cleared => f.write_str("cleared"),
        }
    }
}
impl ::std::str::FromStr for EventStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "planned" => Ok(Self::Planned),
            "suspended" => Ok(Self::Suspended),
            "cleared" => Ok(Self::Cleared),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Incident traffic event payload."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Event Incident\","]
#[doc = "  \"description\": \"Incident traffic event payload.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"eventId\","]
#[doc = "    \"incidentType\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"confirmedBy\": {"]
#[doc = "      \"description\": \"Who confirmed.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"diversionInForce\": {"]
#[doc = "      \"description\": \"Whether diversion is active.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"diversionRoute\": {"]
#[doc = "      \"description\": \"Diversion route description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"endTime\": {"]
#[doc = "      \"description\": \"When cleared.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this event.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"externalCountryCode\": {"]
#[doc = "      \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"externalSituationId\": {"]
#[doc = "      \"description\": \"Cross-ref situation identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"externalSituationVersion\": {"]
#[doc = "      \"description\": \"Cross-ref situation version. Requires external_situation_id.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"externalSystemCode\": {"]
#[doc = "      \"description\": \"Cross-ref system code.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"incidentTime\": {"]
#[doc = "      \"description\": \"When the incident was reported. Optional: unknown when first reported.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"incidentType\": {"]
#[doc = "      \"description\": \"Classification of the incident.\","]
#[doc = "      \"$ref\": \"#/$defs/IncidentType\""]
#[doc = "    },"]
#[doc = "    \"lanesAffected\": {"]
#[doc = "      \"description\": \"Lane closure pattern.\","]
#[doc = "      \"$ref\": \"#/$defs/LanesAffectedType\""]
#[doc = "    },"]
#[doc = "    \"lanesOperational\": {"]
#[doc = "      \"description\": \"Count of operational lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lanesRestricted\": {"]
#[doc = "      \"description\": \"Count of closed/restricted lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"WGS84 point location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"locationDescription\": {"]
#[doc = "      \"description\": \"Free-text location description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modifiedBy\": {"]
#[doc = "      \"description\": \"Who last modified.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"Event phase (unconstrained).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"publicDescription\": {"]
#[doc = "      \"description\": \"Public-facing summary for journey planners.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reportedBy\": {"]
#[doc = "      \"description\": \"Who reported the event.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Event severity.\","]
#[doc = "      \"$ref\": \"#/$defs/SeverityType\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventStatus\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentRequired\": {"]
#[doc = "    \"externalSituationVersion\": ["]
#[doc = "      \"externalSituationId\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncidentEvent {
    #[doc = "Who confirmed."]
    #[serde(
        rename = "confirmedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confirmed_by: ::std::option::Option<::std::string::String>,
    #[doc = "Whether diversion is active."]
    #[serde(
        rename = "diversionInForce",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_in_force: ::std::option::Option<bool>,
    #[doc = "Diversion route description."]
    #[serde(
        rename = "diversionRoute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_route: ::std::option::Option<::std::string::String>,
    #[doc = "When cleared."]
    #[serde(
        rename = "endTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Unique identifier of this event."]
    #[serde(rename = "eventId")]
    pub event_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
    #[serde(
        rename = "externalCountryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_country_code: ::std::option::Option<IncidentEventExternalCountryCode>,
    #[doc = "Cross-ref situation identifier."]
    #[serde(
        rename = "externalSituationId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Cross-ref situation version. Requires external_situation_id."]
    #[serde(
        rename = "externalSituationVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_version: ::std::option::Option<u64>,
    #[doc = "Cross-ref system code."]
    #[serde(
        rename = "externalSystemCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_system_code: ::std::option::Option<::std::string::String>,
    #[doc = "When the incident was reported. Optional: unknown when first reported."]
    #[serde(
        rename = "incidentTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub incident_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Classification of the incident."]
    #[serde(rename = "incidentType")]
    pub incident_type: IncidentType,
    #[doc = "Lane closure pattern."]
    #[serde(
        rename = "lanesAffected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_affected: ::std::option::Option<LanesAffectedType>,
    #[doc = "Count of operational lanes."]
    #[serde(
        rename = "lanesOperational",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_operational: ::std::option::Option<u64>,
    #[doc = "Count of closed/restricted lanes."]
    #[serde(
        rename = "lanesRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_restricted: ::std::option::Option<u64>,
    #[doc = "WGS84 point location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Free-text location description."]
    #[serde(
        rename = "locationDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who last modified."]
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event phase (unconstrained)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<::std::string::String>,
    #[doc = "Public-facing summary for journey planners."]
    #[serde(
        rename = "publicDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who reported the event."]
    #[serde(
        rename = "reportedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reported_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event severity."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<SeverityType>,
    pub status: EventStatus,
}
impl IncidentEvent {
    pub fn builder() -> builder::IncidentEvent {
        Default::default()
    }
}
#[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct IncidentEventExternalCountryCode(::std::string::String);
impl ::std::ops::Deref for IncidentEventExternalCountryCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IncidentEventExternalCountryCode> for ::std::string::String {
    fn from(value: IncidentEventExternalCountryCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for IncidentEventExternalCountryCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for IncidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IncidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IncidentEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for IncidentEventExternalCountryCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Classification of incident (78 values from the UTMC specification).\n\nThe XSD carries only a catalogue reference, so the value set is not in it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Classification of incident (78 values from the UTMC specification).\\n\\nThe XSD carries only a catalogue reference, so the value set is not in it.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"abnormalLoad\","]
#[doc = "    \"airCrash\","]
#[doc = "    \"airRaid\","]
#[doc = "    \"animalsOnRoad\","]
#[doc = "    \"attackOnVehicle\","]
#[doc = "    \"avalanche\","]
#[doc = "    \"blasting\","]
#[doc = "    \"bridgeBlocked\","]
#[doc = "    \"bridgeClosed\","]
#[doc = "    \"bridgeDemolition\","]
#[doc = "    \"bridgeMaintenance\","]
#[doc = "    \"bridgeOpening\","]
#[doc = "    \"brokenDownBus\","]
#[doc = "    \"brokenDownLorry\","]
#[doc = "    \"brokenDownVehicle\","]
#[doc = "    \"burstPipe\","]
#[doc = "    \"burstWaterMain\","]
#[doc = "    \"busDisruption\","]
#[doc = "    \"busLaneBlocked\","]
#[doc = "    \"busLaneClosed\","]
#[doc = "    \"centreLaneBlocked\","]
#[doc = "    \"centreLaneClosed\","]
#[doc = "    \"chemicalSpillage\","]
#[doc = "    \"childrenOnRoad\","]
#[doc = "    \"civilEmergency\","]
#[doc = "    \"civilEmergencyCancelled\","]
#[doc = "    \"smogAlert\","]
#[doc = "    \"congestion\","]
#[doc = "    \"contraflow\","]
#[doc = "    \"convoy\","]
#[doc = "    \"crowd\","]
#[doc = "    \"cyclists\","]
#[doc = "    \"diversion\","]
#[doc = "    \"emergencyAlert\","]
#[doc = "    \"facilitiesClosed\","]
#[doc = "    \"fault\","]
#[doc = "    \"fire\","]
#[doc = "    \"flood\","]
#[doc = "    \"flow\","]
#[doc = "    \"fog\","]
#[doc = "    \"gawkingTraffic\","]
#[doc = "    \"gunfire\","]
#[doc = "    \"hail\","]
#[doc = "    \"heavyFlow\","]
#[doc = "    \"ice\","]
#[doc = "    \"laneClosure\","]
#[doc = "    \"lightFault\","]
#[doc = "    \"obstruction\","]
#[doc = "    \"peopleOnRoad\","]
#[doc = "    \"pollution\","]
#[doc = "    \"powerFailure\","]
#[doc = "    \"publicDisturbance\","]
#[doc = "    \"queue\","]
#[doc = "    \"radioactiveLeak\","]
#[doc = "    \"rain\","]
#[doc = "    \"riot\","]
#[doc = "    \"roadClosure\","]
#[doc = "    \"roadsideFire\","]
#[doc = "    \"sandstorms\","]
#[doc = "    \"securityAlert\","]
#[doc = "    \"securityIncident\","]
#[doc = "    \"sightseers\","]
#[doc = "    \"snow\","]
#[doc = "    \"speedRestriction\","]
#[doc = "    \"spillage\","]
#[doc = "    \"sprayHazard\","]
#[doc = "    \"structuralDamage\","]
#[doc = "    \"telephoneFault\","]
#[doc = "    \"temporarySignal\","]
#[doc = "    \"terroristIncident\","]
#[doc = "    \"tornadoes\","]
#[doc = "    \"trafficSignalFault\","]
#[doc = "    \"trainDisruption\","]
#[doc = "    \"unexplainedDamage\","]
#[doc = "    \"unexplainedEvent\","]
#[doc = "    \"washout\","]
#[doc = "    \"wind\","]
#[doc = "    \"other\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum IncidentType {
    #[serde(rename = "abnormalLoad")]
    AbnormalLoad,
    #[serde(rename = "airCrash")]
    AirCrash,
    #[serde(rename = "airRaid")]
    AirRaid,
    #[serde(rename = "animalsOnRoad")]
    AnimalsOnRoad,
    #[serde(rename = "attackOnVehicle")]
    AttackOnVehicle,
    #[serde(rename = "avalanche")]
    Avalanche,
    #[serde(rename = "blasting")]
    Blasting,
    #[serde(rename = "bridgeBlocked")]
    BridgeBlocked,
    #[serde(rename = "bridgeClosed")]
    BridgeClosed,
    #[serde(rename = "bridgeDemolition")]
    BridgeDemolition,
    #[serde(rename = "bridgeMaintenance")]
    BridgeMaintenance,
    #[serde(rename = "bridgeOpening")]
    BridgeOpening,
    #[serde(rename = "brokenDownBus")]
    BrokenDownBus,
    #[serde(rename = "brokenDownLorry")]
    BrokenDownLorry,
    #[serde(rename = "brokenDownVehicle")]
    BrokenDownVehicle,
    #[serde(rename = "burstPipe")]
    BurstPipe,
    #[serde(rename = "burstWaterMain")]
    BurstWaterMain,
    #[serde(rename = "busDisruption")]
    BusDisruption,
    #[serde(rename = "busLaneBlocked")]
    BusLaneBlocked,
    #[serde(rename = "busLaneClosed")]
    BusLaneClosed,
    #[serde(rename = "centreLaneBlocked")]
    CentreLaneBlocked,
    #[serde(rename = "centreLaneClosed")]
    CentreLaneClosed,
    #[serde(rename = "chemicalSpillage")]
    ChemicalSpillage,
    #[serde(rename = "childrenOnRoad")]
    ChildrenOnRoad,
    #[serde(rename = "civilEmergency")]
    CivilEmergency,
    #[serde(rename = "civilEmergencyCancelled")]
    CivilEmergencyCancelled,
    #[serde(rename = "smogAlert")]
    SmogAlert,
    #[serde(rename = "congestion")]
    Congestion,
    #[serde(rename = "contraflow")]
    Contraflow,
    #[serde(rename = "convoy")]
    Convoy,
    #[serde(rename = "crowd")]
    Crowd,
    #[serde(rename = "cyclists")]
    Cyclists,
    #[serde(rename = "diversion")]
    Diversion,
    #[serde(rename = "emergencyAlert")]
    EmergencyAlert,
    #[serde(rename = "facilitiesClosed")]
    FacilitiesClosed,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "flood")]
    Flood,
    #[serde(rename = "flow")]
    Flow,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "gawkingTraffic")]
    GawkingTraffic,
    #[serde(rename = "gunfire")]
    Gunfire,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "heavyFlow")]
    HeavyFlow,
    #[serde(rename = "ice")]
    Ice,
    #[serde(rename = "laneClosure")]
    LaneClosure,
    #[serde(rename = "lightFault")]
    LightFault,
    #[serde(rename = "obstruction")]
    Obstruction,
    #[serde(rename = "peopleOnRoad")]
    PeopleOnRoad,
    #[serde(rename = "pollution")]
    Pollution,
    #[serde(rename = "powerFailure")]
    PowerFailure,
    #[serde(rename = "publicDisturbance")]
    PublicDisturbance,
    #[serde(rename = "queue")]
    Queue,
    #[serde(rename = "radioactiveLeak")]
    RadioactiveLeak,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "riot")]
    Riot,
    #[serde(rename = "roadClosure")]
    RoadClosure,
    #[serde(rename = "roadsideFire")]
    RoadsideFire,
    #[serde(rename = "sandstorms")]
    Sandstorms,
    #[serde(rename = "securityAlert")]
    SecurityAlert,
    #[serde(rename = "securityIncident")]
    SecurityIncident,
    #[serde(rename = "sightseers")]
    Sightseers,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "speedRestriction")]
    SpeedRestriction,
    #[serde(rename = "spillage")]
    Spillage,
    #[serde(rename = "sprayHazard")]
    SprayHazard,
    #[serde(rename = "structuralDamage")]
    StructuralDamage,
    #[serde(rename = "telephoneFault")]
    TelephoneFault,
    #[serde(rename = "temporarySignal")]
    TemporarySignal,
    #[serde(rename = "terroristIncident")]
    TerroristIncident,
    #[serde(rename = "tornadoes")]
    Tornadoes,
    #[serde(rename = "trafficSignalFault")]
    TrafficSignalFault,
    #[serde(rename = "trainDisruption")]
    TrainDisruption,
    #[serde(rename = "unexplainedDamage")]
    UnexplainedDamage,
    #[serde(rename = "unexplainedEvent")]
    UnexplainedEvent,
    #[serde(rename = "washout")]
    Washout,
    #[serde(rename = "wind")]
    Wind,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for IncidentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AbnormalLoad => f.write_str("abnormalLoad"),
            Self::AirCrash => f.write_str("airCrash"),
            Self::AirRaid => f.write_str("airRaid"),
            Self::AnimalsOnRoad => f.write_str("animalsOnRoad"),
            Self::AttackOnVehicle => f.write_str("attackOnVehicle"),
            Self::Avalanche => f.write_str("avalanche"),
            Self::Blasting => f.write_str("blasting"),
            Self::BridgeBlocked => f.write_str("bridgeBlocked"),
            Self::BridgeClosed => f.write_str("bridgeClosed"),
            Self::BridgeDemolition => f.write_str("bridgeDemolition"),
            Self::BridgeMaintenance => f.write_str("bridgeMaintenance"),
            Self::BridgeOpening => f.write_str("bridgeOpening"),
            Self::BrokenDownBus => f.write_str("brokenDownBus"),
            Self::BrokenDownLorry => f.write_str("brokenDownLorry"),
            Self::BrokenDownVehicle => f.write_str("brokenDownVehicle"),
            Self::BurstPipe => f.write_str("burstPipe"),
            Self::BurstWaterMain => f.write_str("burstWaterMain"),
            Self::BusDisruption => f.write_str("busDisruption"),
            Self::BusLaneBlocked => f.write_str("busLaneBlocked"),
            Self::BusLaneClosed => f.write_str("busLaneClosed"),
            Self::CentreLaneBlocked => f.write_str("centreLaneBlocked"),
            Self::CentreLaneClosed => f.write_str("centreLaneClosed"),
            Self::ChemicalSpillage => f.write_str("chemicalSpillage"),
            Self::ChildrenOnRoad => f.write_str("childrenOnRoad"),
            Self::CivilEmergency => f.write_str("civilEmergency"),
            Self::CivilEmergencyCancelled => f.write_str("civilEmergencyCancelled"),
            Self::SmogAlert => f.write_str("smogAlert"),
            Self::Congestion => f.write_str("congestion"),
            Self::Contraflow => f.write_str("contraflow"),
            Self::Convoy => f.write_str("convoy"),
            Self::Crowd => f.write_str("crowd"),
            Self::Cyclists => f.write_str("cyclists"),
            Self::Diversion => f.write_str("diversion"),
            Self::EmergencyAlert => f.write_str("emergencyAlert"),
            Self::FacilitiesClosed => f.write_str("facilitiesClosed"),
            Self::Fault => f.write_str("fault"),
            Self::Fire => f.write_str("fire"),
            Self::Flood => f.write_str("flood"),
            Self::Flow => f.write_str("flow"),
            Self::Fog => f.write_str("fog"),
            Self::GawkingTraffic => f.write_str("gawkingTraffic"),
            Self::Gunfire => f.write_str("gunfire"),
            Self::Hail => f.write_str("hail"),
            Self::HeavyFlow => f.write_str("heavyFlow"),
            Self::Ice => f.write_str("ice"),
            Self::LaneClosure => f.write_str("laneClosure"),
            Self::LightFault => f.write_str("lightFault"),
            Self::Obstruction => f.write_str("obstruction"),
            Self::PeopleOnRoad => f.write_str("peopleOnRoad"),
            Self::Pollution => f.write_str("pollution"),
            Self::PowerFailure => f.write_str("powerFailure"),
            Self::PublicDisturbance => f.write_str("publicDisturbance"),
            Self::Queue => f.write_str("queue"),
            Self::RadioactiveLeak => f.write_str("radioactiveLeak"),
            Self::Rain => f.write_str("rain"),
            Self::Riot => f.write_str("riot"),
            Self::RoadClosure => f.write_str("roadClosure"),
            Self::RoadsideFire => f.write_str("roadsideFire"),
            Self::Sandstorms => f.write_str("sandstorms"),
            Self::SecurityAlert => f.write_str("securityAlert"),
            Self::SecurityIncident => f.write_str("securityIncident"),
            Self::Sightseers => f.write_str("sightseers"),
            Self::Snow => f.write_str("snow"),
            Self::SpeedRestriction => f.write_str("speedRestriction"),
            Self::Spillage => f.write_str("spillage"),
            Self::SprayHazard => f.write_str("sprayHazard"),
            Self::StructuralDamage => f.write_str("structuralDamage"),
            Self::TelephoneFault => f.write_str("telephoneFault"),
            Self::TemporarySignal => f.write_str("temporarySignal"),
            Self::TerroristIncident => f.write_str("terroristIncident"),
            Self::Tornadoes => f.write_str("tornadoes"),
            Self::TrafficSignalFault => f.write_str("trafficSignalFault"),
            Self::TrainDisruption => f.write_str("trainDisruption"),
            Self::UnexplainedDamage => f.write_str("unexplainedDamage"),
            Self::UnexplainedEvent => f.write_str("unexplainedEvent"),
            Self::Washout => f.write_str("washout"),
            Self::Wind => f.write_str("wind"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for IncidentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "abnormalLoad" => Ok(Self::AbnormalLoad),
            "airCrash" => Ok(Self::AirCrash),
            "airRaid" => Ok(Self::AirRaid),
            "animalsOnRoad" => Ok(Self::AnimalsOnRoad),
            "attackOnVehicle" => Ok(Self::AttackOnVehicle),
            "avalanche" => Ok(Self::Avalanche),
            "blasting" => Ok(Self::Blasting),
            "bridgeBlocked" => Ok(Self::BridgeBlocked),
            "bridgeClosed" => Ok(Self::BridgeClosed),
            "bridgeDemolition" => Ok(Self::BridgeDemolition),
            "bridgeMaintenance" => Ok(Self::BridgeMaintenance),
            "bridgeOpening" => Ok(Self::BridgeOpening),
            "brokenDownBus" => Ok(Self::BrokenDownBus),
            "brokenDownLorry" => Ok(Self::BrokenDownLorry),
            "brokenDownVehicle" => Ok(Self::BrokenDownVehicle),
            "burstPipe" => Ok(Self::BurstPipe),
            "burstWaterMain" => Ok(Self::BurstWaterMain),
            "busDisruption" => Ok(Self::BusDisruption),
            "busLaneBlocked" => Ok(Self::BusLaneBlocked),
            "busLaneClosed" => Ok(Self::BusLaneClosed),
            "centreLaneBlocked" => Ok(Self::CentreLaneBlocked),
            "centreLaneClosed" => Ok(Self::CentreLaneClosed),
            "chemicalSpillage" => Ok(Self::ChemicalSpillage),
            "childrenOnRoad" => Ok(Self::ChildrenOnRoad),
            "civilEmergency" => Ok(Self::CivilEmergency),
            "civilEmergencyCancelled" => Ok(Self::CivilEmergencyCancelled),
            "smogAlert" => Ok(Self::SmogAlert),
            "congestion" => Ok(Self::Congestion),
            "contraflow" => Ok(Self::Contraflow),
            "convoy" => Ok(Self::Convoy),
            "crowd" => Ok(Self::Crowd),
            "cyclists" => Ok(Self::Cyclists),
            "diversion" => Ok(Self::Diversion),
            "emergencyAlert" => Ok(Self::EmergencyAlert),
            "facilitiesClosed" => Ok(Self::FacilitiesClosed),
            "fault" => Ok(Self::Fault),
            "fire" => Ok(Self::Fire),
            "flood" => Ok(Self::Flood),
            "flow" => Ok(Self::Flow),
            "fog" => Ok(Self::Fog),
            "gawkingTraffic" => Ok(Self::GawkingTraffic),
            "gunfire" => Ok(Self::Gunfire),
            "hail" => Ok(Self::Hail),
            "heavyFlow" => Ok(Self::HeavyFlow),
            "ice" => Ok(Self::Ice),
            "laneClosure" => Ok(Self::LaneClosure),
            "lightFault" => Ok(Self::LightFault),
            "obstruction" => Ok(Self::Obstruction),
            "peopleOnRoad" => Ok(Self::PeopleOnRoad),
            "pollution" => Ok(Self::Pollution),
            "powerFailure" => Ok(Self::PowerFailure),
            "publicDisturbance" => Ok(Self::PublicDisturbance),
            "queue" => Ok(Self::Queue),
            "radioactiveLeak" => Ok(Self::RadioactiveLeak),
            "rain" => Ok(Self::Rain),
            "riot" => Ok(Self::Riot),
            "roadClosure" => Ok(Self::RoadClosure),
            "roadsideFire" => Ok(Self::RoadsideFire),
            "sandstorms" => Ok(Self::Sandstorms),
            "securityAlert" => Ok(Self::SecurityAlert),
            "securityIncident" => Ok(Self::SecurityIncident),
            "sightseers" => Ok(Self::Sightseers),
            "snow" => Ok(Self::Snow),
            "speedRestriction" => Ok(Self::SpeedRestriction),
            "spillage" => Ok(Self::Spillage),
            "sprayHazard" => Ok(Self::SprayHazard),
            "structuralDamage" => Ok(Self::StructuralDamage),
            "telephoneFault" => Ok(Self::TelephoneFault),
            "temporarySignal" => Ok(Self::TemporarySignal),
            "terroristIncident" => Ok(Self::TerroristIncident),
            "tornadoes" => Ok(Self::Tornadoes),
            "trafficSignalFault" => Ok(Self::TrafficSignalFault),
            "trainDisruption" => Ok(Self::TrainDisruption),
            "unexplainedDamage" => Ok(Self::UnexplainedDamage),
            "unexplainedEvent" => Ok(Self::UnexplainedEvent),
            "washout" => Ok(Self::Washout),
            "wind" => Ok(Self::Wind),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IncidentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IncidentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IncidentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Lane closure pattern.\n\nI = open lane, T = closed lane, read nearside to offside."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Lane closure pattern.\\n\\nI = open lane, T = closed lane, read nearside to offside.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"I\","]
#[doc = "    \"T\","]
#[doc = "    \"II\","]
#[doc = "    \"IT\","]
#[doc = "    \"TI\","]
#[doc = "    \"TT\","]
#[doc = "    \"III\","]
#[doc = "    \"IIT\","]
#[doc = "    \"ITI\","]
#[doc = "    \"ITT\","]
#[doc = "    \"TII\","]
#[doc = "    \"TIT\","]
#[doc = "    \"TTI\","]
#[doc = "    \"TTT\","]
#[doc = "    \"IIII\","]
#[doc = "    \"IIIT\","]
#[doc = "    \"IITT\","]
#[doc = "    \"ITTT\","]
#[doc = "    \"TIII\","]
#[doc = "    \"TTTT\","]
#[doc = "    \"allOpen\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LanesAffectedType {
    I,
    T,
    #[serde(rename = "II")]
    Ii,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "TI")]
    Ti,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "III")]
    Iii,
    #[serde(rename = "IIT")]
    Iit,
    #[serde(rename = "ITI")]
    Iti,
    #[serde(rename = "ITT")]
    Itt,
    #[serde(rename = "TII")]
    Tii,
    #[serde(rename = "TIT")]
    Tit,
    #[serde(rename = "TTI")]
    Tti,
    #[serde(rename = "TTT")]
    Ttt,
    #[serde(rename = "IIII")]
    Iiii,
    #[serde(rename = "IIIT")]
    Iiit,
    #[serde(rename = "IITT")]
    Iitt,
    #[serde(rename = "ITTT")]
    Ittt,
    #[serde(rename = "TIII")]
    Tiii,
    #[serde(rename = "TTTT")]
    Tttt,
    #[serde(rename = "allOpen")]
    AllOpen,
}
impl ::std::fmt::Display for LanesAffectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::I => f.write_str("I"),
            Self::T => f.write_str("T"),
            Self::Ii => f.write_str("II"),
            Self::It => f.write_str("IT"),
            Self::Ti => f.write_str("TI"),
            Self::Tt => f.write_str("TT"),
            Self::Iii => f.write_str("III"),
            Self::Iit => f.write_str("IIT"),
            Self::Iti => f.write_str("ITI"),
            Self::Itt => f.write_str("ITT"),
            Self::Tii => f.write_str("TII"),
            Self::Tit => f.write_str("TIT"),
            Self::Tti => f.write_str("TTI"),
            Self::Ttt => f.write_str("TTT"),
            Self::Iiii => f.write_str("IIII"),
            Self::Iiit => f.write_str("IIIT"),
            Self::Iitt => f.write_str("IITT"),
            Self::Ittt => f.write_str("ITTT"),
            Self::Tiii => f.write_str("TIII"),
            Self::Tttt => f.write_str("TTTT"),
            Self::AllOpen => f.write_str("allOpen"),
        }
    }
}
impl ::std::str::FromStr for LanesAffectedType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "I" => Ok(Self::I),
            "T" => Ok(Self::T),
            "II" => Ok(Self::Ii),
            "IT" => Ok(Self::It),
            "TI" => Ok(Self::Ti),
            "TT" => Ok(Self::Tt),
            "III" => Ok(Self::Iii),
            "IIT" => Ok(Self::Iit),
            "ITI" => Ok(Self::Iti),
            "ITT" => Ok(Self::Itt),
            "TII" => Ok(Self::Tii),
            "TIT" => Ok(Self::Tit),
            "TTI" => Ok(Self::Tti),
            "TTT" => Ok(Self::Ttt),
            "IIII" => Ok(Self::Iiii),
            "IIIT" => Ok(Self::Iiit),
            "IITT" => Ok(Self::Iitt),
            "ITTT" => Ok(Self::Ittt),
            "TIII" => Ok(Self::Tiii),
            "TTTT" => Ok(Self::Tttt),
            "allOpen" => Ok(Self::AllOpen),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LanesAffectedType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LanesAffectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LanesAffectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Unique identifier of this event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Unique identifier of this event.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ObjectId(::std::string::String);
impl ::std::ops::Deref for ObjectId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ObjectId> for ::std::string::String {
    fn from(value: ObjectId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ObjectId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ObjectId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ObjectId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ObjectId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Planned event traffic event payload."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Event Planned Event\","]
#[doc = "  \"description\": \"Planned event traffic event payload.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"eventId\","]
#[doc = "    \"plannedEventType\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actualEnd\": {"]
#[doc = "      \"description\": \"Actual end time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"actualStart\": {"]
#[doc = "      \"description\": \"Actual start time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"confirmedBy\": {"]
#[doc = "      \"description\": \"Who confirmed.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"diversionInForce\": {"]
#[doc = "      \"description\": \"Whether diversion is active.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"diversionRoute\": {"]
#[doc = "      \"description\": \"Diversion route description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this event.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"externalCountryCode\": {"]
#[doc = "      \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"externalSituationId\": {"]
#[doc = "      \"description\": \"Cross-ref situation identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"externalSituationVersion\": {"]
#[doc = "      \"description\": \"Cross-ref situation version. Requires external_situation_id.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"externalSystemCode\": {"]
#[doc = "      \"description\": \"Cross-ref system code.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lanesAffected\": {"]
#[doc = "      \"description\": \"Lane closure pattern.\","]
#[doc = "      \"$ref\": \"#/$defs/LanesAffectedType\""]
#[doc = "    },"]
#[doc = "    \"lanesOperational\": {"]
#[doc = "      \"description\": \"Count of operational lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lanesRestricted\": {"]
#[doc = "      \"description\": \"Count of closed/restricted lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"WGS84 point location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"locationDescription\": {"]
#[doc = "      \"description\": \"Free-text location description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modifiedBy\": {"]
#[doc = "      \"description\": \"Who last modified.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"organiser\": {"]
#[doc = "      \"description\": \"Event organiser.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"Event phase (unconstrained).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"plannedEnd\": {"]
#[doc = "      \"description\": \"Planned end time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"plannedEventType\": {"]
#[doc = "      \"description\": \"Classification of the event.\","]
#[doc = "      \"$ref\": \"#/$defs/PlannedEventType\""]
#[doc = "    },"]
#[doc = "    \"plannedStart\": {"]
#[doc = "      \"description\": \"Planned start time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"publicDescription\": {"]
#[doc = "      \"description\": \"Public-facing summary for journey planners.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reportedBy\": {"]
#[doc = "      \"description\": \"Who reported the event.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Event severity.\","]
#[doc = "      \"$ref\": \"#/$defs/SeverityType\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventStatus\""]
#[doc = "    },"]
#[doc = "    \"venueName\": {"]
#[doc = "      \"description\": \"Venue name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentRequired\": {"]
#[doc = "    \"externalSituationVersion\": ["]
#[doc = "      \"externalSituationId\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlannedEvent {
    #[doc = "Actual end time."]
    #[serde(
        rename = "actualEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub actual_end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Actual start time."]
    #[serde(
        rename = "actualStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub actual_start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Who confirmed."]
    #[serde(
        rename = "confirmedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confirmed_by: ::std::option::Option<::std::string::String>,
    #[doc = "Whether diversion is active."]
    #[serde(
        rename = "diversionInForce",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_in_force: ::std::option::Option<bool>,
    #[doc = "Diversion route description."]
    #[serde(
        rename = "diversionRoute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_route: ::std::option::Option<::std::string::String>,
    #[doc = "Unique identifier of this event."]
    #[serde(rename = "eventId")]
    pub event_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
    #[serde(
        rename = "externalCountryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_country_code: ::std::option::Option<PlannedEventExternalCountryCode>,
    #[doc = "Cross-ref situation identifier."]
    #[serde(
        rename = "externalSituationId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Cross-ref situation version. Requires external_situation_id."]
    #[serde(
        rename = "externalSituationVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_version: ::std::option::Option<u64>,
    #[doc = "Cross-ref system code."]
    #[serde(
        rename = "externalSystemCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_system_code: ::std::option::Option<::std::string::String>,
    #[doc = "Lane closure pattern."]
    #[serde(
        rename = "lanesAffected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_affected: ::std::option::Option<LanesAffectedType>,
    #[doc = "Count of operational lanes."]
    #[serde(
        rename = "lanesOperational",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_operational: ::std::option::Option<u64>,
    #[doc = "Count of closed/restricted lanes."]
    #[serde(
        rename = "lanesRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_restricted: ::std::option::Option<u64>,
    #[doc = "WGS84 point location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Free-text location description."]
    #[serde(
        rename = "locationDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who last modified."]
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event organiser."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub organiser: ::std::option::Option<::std::string::String>,
    #[doc = "Event phase (unconstrained)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<::std::string::String>,
    #[doc = "Planned end time."]
    #[serde(
        rename = "plannedEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Classification of the event."]
    #[serde(rename = "plannedEventType")]
    pub planned_event_type: PlannedEventType,
    #[doc = "Planned start time."]
    #[serde(
        rename = "plannedStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Public-facing summary for journey planners."]
    #[serde(
        rename = "publicDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who reported the event."]
    #[serde(
        rename = "reportedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reported_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event severity."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<SeverityType>,
    pub status: EventStatus,
    #[doc = "Venue name."]
    #[serde(
        rename = "venueName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub venue_name: ::std::option::Option<::std::string::String>,
}
impl PlannedEvent {
    pub fn builder() -> builder::PlannedEvent {
        Default::default()
    }
}
#[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PlannedEventExternalCountryCode(::std::string::String);
impl ::std::ops::Deref for PlannedEventExternalCountryCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PlannedEventExternalCountryCode> for ::std::string::String {
    fn from(value: PlannedEventExternalCountryCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PlannedEventExternalCountryCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PlannedEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlannedEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlannedEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PlannedEventExternalCountryCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Classification of planned event (32 values from the UTMC specification).\n\nThe XSD carries only a catalogue reference, so the value set is not in it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Classification of planned event (32 values from the UTMC specification).\\n\\nThe XSD carries only a catalogue reference, so the value set is not in it.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ballGame\","]
#[doc = "    \"boxing\","]
#[doc = "    \"athletics\","]
#[doc = "    \"cycleRace\","]
#[doc = "    \"demonstration\","]
#[doc = "    \"exhibition\","]
#[doc = "    \"fair\","]
#[doc = "    \"festival\","]
#[doc = "    \"football\","]
#[doc = "    \"funfair\","]
#[doc = "    \"golf\","]
#[doc = "    \"internationalSports\","]
#[doc = "    \"marathon\","]
#[doc = "    \"march\","]
#[doc = "    \"market\","]
#[doc = "    \"match\","]
#[doc = "    \"parade\","]
#[doc = "    \"procession\","]
#[doc = "    \"race\","]
#[doc = "    \"rugby\","]
#[doc = "    \"show\","]
#[doc = "    \"showJumping\","]
#[doc = "    \"sportsEvent\","]
#[doc = "    \"sportsMeeting\","]
#[doc = "    \"stateOccasion\","]
#[doc = "    \"strike\","]
#[doc = "    \"tennis\","]
#[doc = "    \"tournament\","]
#[doc = "    \"tradeFair\","]
#[doc = "    \"waterSports\","]
#[doc = "    \"winterSports\","]
#[doc = "    \"other\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PlannedEventType {
    #[serde(rename = "ballGame")]
    BallGame,
    #[serde(rename = "boxing")]
    Boxing,
    #[serde(rename = "athletics")]
    Athletics,
    #[serde(rename = "cycleRace")]
    CycleRace,
    #[serde(rename = "demonstration")]
    Demonstration,
    #[serde(rename = "exhibition")]
    Exhibition,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "festival")]
    Festival,
    #[serde(rename = "football")]
    Football,
    #[serde(rename = "funfair")]
    Funfair,
    #[serde(rename = "golf")]
    Golf,
    #[serde(rename = "internationalSports")]
    InternationalSports,
    #[serde(rename = "marathon")]
    Marathon,
    #[serde(rename = "march")]
    March,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "parade")]
    Parade,
    #[serde(rename = "procession")]
    Procession,
    #[serde(rename = "race")]
    Race,
    #[serde(rename = "rugby")]
    Rugby,
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "showJumping")]
    ShowJumping,
    #[serde(rename = "sportsEvent")]
    SportsEvent,
    #[serde(rename = "sportsMeeting")]
    SportsMeeting,
    #[serde(rename = "stateOccasion")]
    StateOccasion,
    #[serde(rename = "strike")]
    Strike,
    #[serde(rename = "tennis")]
    Tennis,
    #[serde(rename = "tournament")]
    Tournament,
    #[serde(rename = "tradeFair")]
    TradeFair,
    #[serde(rename = "waterSports")]
    WaterSports,
    #[serde(rename = "winterSports")]
    WinterSports,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for PlannedEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BallGame => f.write_str("ballGame"),
            Self::Boxing => f.write_str("boxing"),
            Self::Athletics => f.write_str("athletics"),
            Self::CycleRace => f.write_str("cycleRace"),
            Self::Demonstration => f.write_str("demonstration"),
            Self::Exhibition => f.write_str("exhibition"),
            Self::Fair => f.write_str("fair"),
            Self::Festival => f.write_str("festival"),
            Self::Football => f.write_str("football"),
            Self::Funfair => f.write_str("funfair"),
            Self::Golf => f.write_str("golf"),
            Self::InternationalSports => f.write_str("internationalSports"),
            Self::Marathon => f.write_str("marathon"),
            Self::March => f.write_str("march"),
            Self::Market => f.write_str("market"),
            Self::Match => f.write_str("match"),
            Self::Parade => f.write_str("parade"),
            Self::Procession => f.write_str("procession"),
            Self::Race => f.write_str("race"),
            Self::Rugby => f.write_str("rugby"),
            Self::Show => f.write_str("show"),
            Self::ShowJumping => f.write_str("showJumping"),
            Self::SportsEvent => f.write_str("sportsEvent"),
            Self::SportsMeeting => f.write_str("sportsMeeting"),
            Self::StateOccasion => f.write_str("stateOccasion"),
            Self::Strike => f.write_str("strike"),
            Self::Tennis => f.write_str("tennis"),
            Self::Tournament => f.write_str("tournament"),
            Self::TradeFair => f.write_str("tradeFair"),
            Self::WaterSports => f.write_str("waterSports"),
            Self::WinterSports => f.write_str("winterSports"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for PlannedEventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ballGame" => Ok(Self::BallGame),
            "boxing" => Ok(Self::Boxing),
            "athletics" => Ok(Self::Athletics),
            "cycleRace" => Ok(Self::CycleRace),
            "demonstration" => Ok(Self::Demonstration),
            "exhibition" => Ok(Self::Exhibition),
            "fair" => Ok(Self::Fair),
            "festival" => Ok(Self::Festival),
            "football" => Ok(Self::Football),
            "funfair" => Ok(Self::Funfair),
            "golf" => Ok(Self::Golf),
            "internationalSports" => Ok(Self::InternationalSports),
            "marathon" => Ok(Self::Marathon),
            "march" => Ok(Self::March),
            "market" => Ok(Self::Market),
            "match" => Ok(Self::Match),
            "parade" => Ok(Self::Parade),
            "procession" => Ok(Self::Procession),
            "race" => Ok(Self::Race),
            "rugby" => Ok(Self::Rugby),
            "show" => Ok(Self::Show),
            "showJumping" => Ok(Self::ShowJumping),
            "sportsEvent" => Ok(Self::SportsEvent),
            "sportsMeeting" => Ok(Self::SportsMeeting),
            "stateOccasion" => Ok(Self::StateOccasion),
            "strike" => Ok(Self::Strike),
            "tennis" => Ok(Self::Tennis),
            "tournament" => Ok(Self::Tournament),
            "tradeFair" => Ok(Self::TradeFair),
            "waterSports" => Ok(Self::WaterSports),
            "winterSports" => Ok(Self::WinterSports),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlannedEventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlannedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlannedEventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A geographic point: a coordinate plus optional altitude."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A geographic point: a coordinate plus optional altitude.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"coordinates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"altitudeMeters\": {"]
#[doc = "      \"description\": \"Altitude in metres above mean sea level. Optional. Bounds [-1000, 5000] inherited from utmc_location-v1.0.xsd AltitudeType.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 5000.0,"]
#[doc = "      \"minimum\": -1000.0"]
#[doc = "    },"]
#[doc = "    \"coordinates\": {"]
#[doc = "      \"description\": \"WGS84 coordinates.\","]
#[doc = "      \"$ref\": \"#/$defs/Coordinates\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Point {
    #[doc = "Altitude in metres above mean sea level. Optional. Bounds [-1000, 5000] inherited from utmc_location-v1.0.xsd AltitudeType."]
    #[serde(
        rename = "altitudeMeters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub altitude_meters: ::std::option::Option<f64>,
    #[doc = "WGS84 coordinates."]
    pub coordinates: Coordinates,
}
impl Point {
    pub fn builder() -> builder::Point {
        Default::default()
    }
}
#[doc = "Roadworks traffic event payload."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Event Roadworks\","]
#[doc = "  \"description\": \"Roadworks traffic event payload.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"eventId\","]
#[doc = "    \"roadworksType\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actualEnd\": {"]
#[doc = "      \"description\": \"Actual end time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"actualStart\": {"]
#[doc = "      \"description\": \"Actual start time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"confirmedBy\": {"]
#[doc = "      \"description\": \"Who confirmed.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"confirmedDate\": {"]
#[doc = "      \"description\": \"When the roadworks were confirmed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"contractor\": {"]
#[doc = "      \"description\": \"Contractor name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"contraflow\": {"]
#[doc = "      \"description\": \"Whether contraflow is in operation.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"diversionInForce\": {"]
#[doc = "      \"description\": \"Whether diversion is active.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"diversionRoute\": {"]
#[doc = "      \"description\": \"Diversion route description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"eventId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this event.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"externalCountryCode\": {"]
#[doc = "      \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"externalSituationId\": {"]
#[doc = "      \"description\": \"Cross-ref situation identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"externalSituationVersion\": {"]
#[doc = "      \"description\": \"Cross-ref situation version. Requires external_situation_id.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"externalSystemCode\": {"]
#[doc = "      \"description\": \"Cross-ref system code.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lanesAffected\": {"]
#[doc = "      \"description\": \"Lane closure pattern.\","]
#[doc = "      \"$ref\": \"#/$defs/LanesAffectedType\""]
#[doc = "    },"]
#[doc = "    \"lanesOperational\": {"]
#[doc = "      \"description\": \"Count of operational lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"lanesRestricted\": {"]
#[doc = "      \"description\": \"Count of closed/restricted lanes.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"WGS84 point location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"locationDescription\": {"]
#[doc = "      \"description\": \"Free-text location description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"modifiedBy\": {"]
#[doc = "      \"description\": \"Who last modified.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"Event phase (unconstrained).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"plannedEnd\": {"]
#[doc = "      \"description\": \"Planned end time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"plannedStart\": {"]
#[doc = "      \"description\": \"Planned start time.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"publicDescription\": {"]
#[doc = "      \"description\": \"Public-facing summary for journey planners.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reportedBy\": {"]
#[doc = "      \"description\": \"Who reported the event.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"roadworksType\": {"]
#[doc = "      \"description\": \"Classification of the roadworks.\","]
#[doc = "      \"$ref\": \"#/$defs/RoadworksType\""]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Event severity.\","]
#[doc = "      \"$ref\": \"#/$defs/SeverityType\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/EventStatus\""]
#[doc = "    },"]
#[doc = "    \"trafficSignals\": {"]
#[doc = "      \"description\": \"Whether temporary traffic signals are in use.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentRequired\": {"]
#[doc = "    \"externalSituationVersion\": ["]
#[doc = "      \"externalSituationId\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RoadworksEvent {
    #[doc = "Actual end time."]
    #[serde(
        rename = "actualEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub actual_end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Actual start time."]
    #[serde(
        rename = "actualStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub actual_start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Who confirmed."]
    #[serde(
        rename = "confirmedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confirmed_by: ::std::option::Option<::std::string::String>,
    #[doc = "When the roadworks were confirmed."]
    #[serde(
        rename = "confirmedDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub confirmed_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Contractor name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contractor: ::std::option::Option<::std::string::String>,
    #[doc = "Whether contraflow is in operation."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contraflow: ::std::option::Option<bool>,
    #[doc = "Whether diversion is active."]
    #[serde(
        rename = "diversionInForce",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_in_force: ::std::option::Option<bool>,
    #[doc = "Diversion route description."]
    #[serde(
        rename = "diversionRoute",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub diversion_route: ::std::option::Option<::std::string::String>,
    #[doc = "Unique identifier of this event."]
    #[serde(rename = "eventId")]
    pub event_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
    #[serde(
        rename = "externalCountryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_country_code: ::std::option::Option<RoadworksEventExternalCountryCode>,
    #[doc = "Cross-ref situation identifier."]
    #[serde(
        rename = "externalSituationId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_id: ::std::option::Option<::std::string::String>,
    #[doc = "Cross-ref situation version. Requires external_situation_id."]
    #[serde(
        rename = "externalSituationVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_situation_version: ::std::option::Option<u64>,
    #[doc = "Cross-ref system code."]
    #[serde(
        rename = "externalSystemCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub external_system_code: ::std::option::Option<::std::string::String>,
    #[doc = "Lane closure pattern."]
    #[serde(
        rename = "lanesAffected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_affected: ::std::option::Option<LanesAffectedType>,
    #[doc = "Count of operational lanes."]
    #[serde(
        rename = "lanesOperational",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_operational: ::std::option::Option<u64>,
    #[doc = "Count of closed/restricted lanes."]
    #[serde(
        rename = "lanesRestricted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lanes_restricted: ::std::option::Option<u64>,
    #[doc = "WGS84 point location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Free-text location description."]
    #[serde(
        rename = "locationDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub location_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who last modified."]
    #[serde(
        rename = "modifiedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_by: ::std::option::Option<::std::string::String>,
    #[doc = "Event phase (unconstrained)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub phase: ::std::option::Option<::std::string::String>,
    #[doc = "Planned end time."]
    #[serde(
        rename = "plannedEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_end: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Planned start time."]
    #[serde(
        rename = "plannedStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub planned_start: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Public-facing summary for journey planners."]
    #[serde(
        rename = "publicDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub public_description: ::std::option::Option<::std::string::String>,
    #[doc = "Who reported the event."]
    #[serde(
        rename = "reportedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reported_by: ::std::option::Option<::std::string::String>,
    #[doc = "Classification of the roadworks."]
    #[serde(rename = "roadworksType")]
    pub roadworks_type: RoadworksType,
    #[doc = "Event severity."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<SeverityType>,
    pub status: EventStatus,
    #[doc = "Whether temporary traffic signals are in use."]
    #[serde(
        rename = "trafficSignals",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signals: ::std::option::Option<bool>,
}
impl RoadworksEvent {
    pub fn builder() -> builder::RoadworksEvent {
        Default::default()
    }
}
#[doc = "Cross-ref country code (an XML NMTOKEN: no whitespace)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Cross-ref country code (an XML NMTOKEN: no whitespace).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RoadworksEventExternalCountryCode(::std::string::String);
impl ::std::ops::Deref for RoadworksEventExternalCountryCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RoadworksEventExternalCountryCode> for ::std::string::String {
    fn from(value: RoadworksEventExternalCountryCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RoadworksEventExternalCountryCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9._:-]+$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9._:-]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RoadworksEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RoadworksEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RoadworksEventExternalCountryCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RoadworksEventExternalCountryCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Classification of roadworks (7 values from XSD)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Classification of roadworks (7 values from XSD).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"centralReservation\","]
#[doc = "    \"general\","]
#[doc = "    \"overhead\","]
#[doc = "    \"surface\","]
#[doc = "    \"verge\","]
#[doc = "    \"waterMainWork\","]
#[doc = "    \"other\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RoadworksType {
    #[serde(rename = "centralReservation")]
    CentralReservation,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "overhead")]
    Overhead,
    #[serde(rename = "surface")]
    Surface,
    #[serde(rename = "verge")]
    Verge,
    #[serde(rename = "waterMainWork")]
    WaterMainWork,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for RoadworksType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CentralReservation => f.write_str("centralReservation"),
            Self::General => f.write_str("general"),
            Self::Overhead => f.write_str("overhead"),
            Self::Surface => f.write_str("surface"),
            Self::Verge => f.write_str("verge"),
            Self::WaterMainWork => f.write_str("waterMainWork"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for RoadworksType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "centralReservation" => Ok(Self::CentralReservation),
            "general" => Ok(Self::General),
            "overhead" => Ok(Self::Overhead),
            "surface" => Ok(Self::Surface),
            "verge" => Ok(Self::Verge),
            "waterMainWork" => Ok(Self::WaterMainWork),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RoadworksType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RoadworksType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RoadworksType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Event severity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Event severity.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unknown\","]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SeverityType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for SeverityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unknown => f.write_str("unknown"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for SeverityType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SeverityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SeverityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SeverityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Data categories a sync may be restricted to.\n\nMirrors the schema-per-category pattern: a subscriber that only holds\na fault dashboard asks for ``fault`` alone rather than provoking a\nfull republication of every definition and configuration it already\nhas."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Data categories a sync may be restricted to.\\n\\nMirrors the schema-per-category pattern: a subscriber that only holds\\na fault dashboard asks for ``fault`` alone rather than provoking a\\nfull republication of every definition and configuration it already\\nhas.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"definition\","]
#[doc = "    \"configuration\","]
#[doc = "    \"measurement\","]
#[doc = "    \"fault\","]
#[doc = "    \"type\","]
#[doc = "    \"status\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SyncCategoryEnum {
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "configuration")]
    Configuration,
    #[serde(rename = "measurement")]
    Measurement,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "status")]
    Status,
}
impl ::std::fmt::Display for SyncCategoryEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Definition => f.write_str("definition"),
            Self::Configuration => f.write_str("configuration"),
            Self::Measurement => f.write_str("measurement"),
            Self::Fault => f.write_str("fault"),
            Self::Type => f.write_str("type"),
            Self::Status => f.write_str("status"),
        }
    }
}
impl ::std::str::FromStr for SyncCategoryEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "definition" => Ok(Self::Definition),
            "configuration" => Ok(Self::Configuration),
            "measurement" => Ok(Self::Measurement),
            "fault" => Ok(Self::Fault),
            "type" => Ok(Self::Type),
            "status" => Ok(Self::Status),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncCategoryEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncCategoryEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncCategoryEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Terminal message ending a traffic event sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic event Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a traffic event sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"completedTime\","]
#[doc = "    \"objectCount\","]
#[doc = "    \"status\","]
#[doc = "    \"syncCorrelationId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"completedTime\": {"]
#[doc = "      \"description\": \"When the publisher finished the sync.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable explanation, for rejected or failed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"objectCount\": {"]
#[doc = "      \"description\": \"Number of objects delivered in this batch. Zero is a valid answer and is the reason this message exists.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome of the sync.\","]
#[doc = "      \"$ref\": \"#/$defs/SyncStatus\""]
#[doc = "    },"]
#[doc = "    \"syncCorrelationId\": {"]
#[doc = "      \"description\": \"Copied unchanged from the SyncRequest this completes, so a requester with several syncs in flight can tell them apart.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"syncId\": {"]
#[doc = "      \"description\": \"The x-sync-id shared by the messages of this batch. Normally absent when no objects matched, since there was no batch to identify. Only a rejected sync is required to omit it; a success or a failed sync may carry one (a failed sync began a batch).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SyncComplete {
    #[doc = "When the publisher finished the sync."]
    #[serde(rename = "completedTime")]
    pub completed_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or forward-compatibility extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Human-readable explanation, for rejected or failed."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<SyncCompleteMessage>,
    #[doc = "Number of objects delivered in this batch. Zero is a valid answer and is the reason this message exists."]
    #[serde(rename = "objectCount")]
    pub object_count: u64,
    #[doc = "Outcome of the sync."]
    pub status: SyncStatus,
    #[doc = "Copied unchanged from the SyncRequest this completes, so a requester with several syncs in flight can tell them apart."]
    #[serde(rename = "syncCorrelationId")]
    pub sync_correlation_id: i64,
    #[doc = "The x-sync-id shared by the messages of this batch. Normally absent when no objects matched, since there was no batch to identify. Only a rejected sync is required to omit it; a success or a failed sync may carry one (a failed sync began a batch)."]
    #[serde(
        rename = "syncId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sync_id: ::std::option::Option<SyncCompleteSyncId>,
}
impl SyncComplete {
    pub fn builder() -> builder::SyncComplete {
        Default::default()
    }
}
#[doc = "Human-readable explanation, for rejected or failed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Human-readable explanation, for rejected or failed.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SyncCompleteMessage(::std::string::String);
impl ::std::ops::Deref for SyncCompleteMessage {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SyncCompleteMessage> for ::std::string::String {
    fn from(value: SyncCompleteMessage) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SyncCompleteMessage {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SyncCompleteMessage {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncCompleteMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncCompleteMessage {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SyncCompleteMessage {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The x-sync-id shared by the messages of this batch. Normally absent when no objects matched, since there was no batch to identify. Only a rejected sync is required to omit it; a success or a failed sync may carry one (a failed sync began a batch)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The x-sync-id shared by the messages of this batch. Normally absent when no objects matched, since there was no batch to identify. Only a rejected sync is required to omit it; a success or a failed sync may carry one (a failed sync began a batch).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SyncCompleteSyncId(::std::string::String);
impl ::std::ops::Deref for SyncCompleteSyncId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SyncCompleteSyncId> for ::std::string::String {
    fn from(value: SyncCompleteSyncId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SyncCompleteSyncId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SyncCompleteSyncId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncCompleteSyncId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncCompleteSyncId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SyncCompleteSyncId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Request the current state of traffic event objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic event Sync Request\","]
#[doc = "  \"description\": \"Request the current state of traffic event objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requestTime\","]
#[doc = "    \"syncCorrelationId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"activeOnly\": {"]
#[doc = "      \"description\": \"Restrict the sync to objects in an active state — for faults, those not yet cleared. Omitted is equivalent to false. This is the filter behind \\\"send me every active fault\\\": without it a subscriber would receive the full fault history it does not want.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"categories\": {"]
#[doc = "      \"description\": \"Restrict the sync to these data categories. Omitted means every category the domain publishes.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncCategoryEnum\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"objectIds\": {"]
#[doc = "      \"description\": \"Restrict the sync to these objects. Omitted means every object of the requested categories.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"ObjectId\","]
#[doc = "        \"description\": \"Unique identifier of an object. String of 1-32 characters from A-Z, a-z, 0-9 and . _ : - (no spaces), unique within object type within system.\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 32,"]
#[doc = "        \"minLength\": 1,"]
#[doc = "        \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the request was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"since\": {"]
#[doc = "      \"description\": \"Restrict the sync to objects changed at or after this time. Intended for a subscriber recovering from a known-length outage; omitted means no lower bound.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"subArea\": {"]
#[doc = "      \"description\": \"Restrict the sync to objects in this sub-area. Omitted means every sub-area.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"syncCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier correlating this request with the sync batch that answers it. The Data Publisher echoes it in the batch's x-sync-request-id header. Not an identifier of the sync itself — that is x-sync-id. Named to avoid collision with the AMQP 1.0 correlation-id message property, as for commandCorrelationId.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Restrict the sync to objects of this type. Omitted means every type.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SyncRequest {
    #[doc = "Restrict the sync to objects in an active state — for faults, those not yet cleared. Omitted is equivalent to false. This is the filter behind \"send me every active fault\": without it a subscriber would receive the full fault history it does not want."]
    #[serde(
        rename = "activeOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub active_only: ::std::option::Option<bool>,
    #[doc = "Restrict the sync to these data categories. Omitted means every category the domain publishes."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub categories: ::std::vec::Vec<SyncCategoryEnum>,
    #[doc = "Vendor or forward-compatibility extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Restrict the sync to these objects. Omitted means every object of the requested categories."]
    #[serde(
        rename = "objectIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub object_ids: ::std::vec::Vec<ObjectId>,
    #[doc = "When the request was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Restrict the sync to objects changed at or after this time. Intended for a subscriber recovering from a known-length outage; omitted means no lower bound."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub since: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Restrict the sync to objects in this sub-area. Omitted means every sub-area."]
    #[serde(
        rename = "subArea",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_area: ::std::option::Option<SyncRequestSubArea>,
    #[doc = "Caller-assigned identifier correlating this request with the sync batch that answers it. The Data Publisher echoes it in the batch's x-sync-request-id header. Not an identifier of the sync itself — that is x-sync-id. Named to avoid collision with the AMQP 1.0 correlation-id message property, as for commandCorrelationId."]
    #[serde(rename = "syncCorrelationId")]
    pub sync_correlation_id: i64,
    #[doc = "Restrict the sync to objects of this type. Omitted means every type."]
    #[serde(
        rename = "typeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_id: ::std::option::Option<TypeId>,
}
impl SyncRequest {
    pub fn builder() -> builder::SyncRequest {
        Default::default()
    }
}
#[doc = "Restrict the sync to objects in this sub-area. Omitted means every sub-area."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Restrict the sync to objects in this sub-area. Omitted means every sub-area.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SyncRequestSubArea(::std::string::String);
impl ::std::ops::Deref for SyncRequestSubArea {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SyncRequestSubArea> for ::std::string::String {
    fn from(value: SyncRequestSubArea) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SyncRequestSubArea {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SyncRequestSubArea {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncRequestSubArea {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncRequestSubArea {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SyncRequestSubArea {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Outcome of a sync request (A12)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome of a sync request (A12).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\","]
#[doc = "    \"rejected\","]
#[doc = "    \"failed\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SyncStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => f.write_str("success"),
            Self::Rejected => f.write_str("rejected"),
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for SyncStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            "rejected" => Ok(Self::Rejected),
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Restrict the sync to objects of this type. Omitted means every type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeId\","]
#[doc = "  \"description\": \"Restrict the sync to objects of this type. Omitted means every type.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeId(::std::string::String);
impl ::std::ops::Deref for TypeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeId> for ::std::string::String {
    fn from(value: TypeId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TypeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^\\S(?:[\\s\\S]*\\S)?$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\S(?:[\\s\\S]*\\S)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AccidentEvent {
        accident_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        accident_type: ::std::result::Result<super::AccidentType, ::std::string::String>,
        confirmed_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        diversion_in_force:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        diversion_route: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        external_country_code: ::std::result::Result<
            ::std::option::Option<super::AccidentEventExternalCountryCode>,
            ::std::string::String,
        >,
        external_situation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_situation_version:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        external_system_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lanes_affected: ::std::result::Result<
            ::std::option::Option<super::LanesAffectedType>,
            ::std::string::String,
        >,
        lanes_operational: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        lanes_restricted: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        light_conditions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        location_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        phase: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        public_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reported_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        road_conditions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        severity: ::std::result::Result<
            ::std::option::Option<super::SeverityType>,
            ::std::string::String,
        >,
        statistic_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::EventStatus, ::std::string::String>,
        vehicle_count: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        weather_conditions: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AccidentEvent {
        fn default() -> Self {
            Self {
                accident_time: Ok(Default::default()),
                accident_type: Err("no value supplied for accident_type".to_string()),
                confirmed_by: Ok(Default::default()),
                diversion_in_force: Ok(Default::default()),
                diversion_route: Ok(Default::default()),
                end_time: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                extensions: Ok(Default::default()),
                external_country_code: Ok(Default::default()),
                external_situation_id: Ok(Default::default()),
                external_situation_version: Ok(Default::default()),
                external_system_code: Ok(Default::default()),
                lanes_affected: Ok(Default::default()),
                lanes_operational: Ok(Default::default()),
                lanes_restricted: Ok(Default::default()),
                light_conditions: Ok(Default::default()),
                location: Ok(Default::default()),
                location_description: Ok(Default::default()),
                modified_by: Ok(Default::default()),
                phase: Ok(Default::default()),
                public_description: Ok(Default::default()),
                reported_by: Ok(Default::default()),
                road_conditions: Ok(Default::default()),
                severity: Ok(Default::default()),
                statistic_date: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                vehicle_count: Ok(Default::default()),
                weather_conditions: Ok(Default::default()),
            }
        }
    }
    impl AccidentEvent {
        pub fn accident_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.accident_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accident_time: {e}"));
            self
        }
        pub fn accident_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccidentType>,
            T::Error: ::std::fmt::Display,
        {
            self.accident_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accident_type: {e}"));
            self
        }
        pub fn confirmed_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.confirmed_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirmed_by: {e}"));
            self
        }
        pub fn diversion_in_force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_in_force = value.try_into().map_err(|e| {
                format!("error converting supplied value for diversion_in_force: {e}")
            });
            self
        }
        pub fn diversion_route<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_route = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diversion_route: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn external_country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::AccidentEventExternalCountryCode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_country_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_country_code: {e}")
            });
            self
        }
        pub fn external_situation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_id: {e}")
            });
            self
        }
        pub fn external_situation_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_version = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_version: {e}")
            });
            self
        }
        pub fn external_system_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_system_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_system_code: {e}")
            });
            self
        }
        pub fn lanes_affected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanesAffectedType>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_affected: {e}"));
            self
        }
        pub fn lanes_operational<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_operational = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_operational: {e}"));
            self
        }
        pub fn lanes_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_restricted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_restricted: {e}"));
            self
        }
        pub fn light_conditions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.light_conditions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for light_conditions: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn location_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for location_description: {e}")
            });
            self
        }
        pub fn modified_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified_by: {e}"));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {e}"));
            self
        }
        pub fn public_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.public_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for public_description: {e}")
            });
            self
        }
        pub fn reported_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reported_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reported_by: {e}"));
            self
        }
        pub fn road_conditions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.road_conditions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for road_conditions: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SeverityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
            self
        }
        pub fn statistic_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.statistic_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for statistic_date: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn vehicle_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.vehicle_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vehicle_count: {e}"));
            self
        }
        pub fn weather_conditions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.weather_conditions = value.try_into().map_err(|e| {
                format!("error converting supplied value for weather_conditions: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AccidentEvent> for super::AccidentEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AccidentEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accident_time: value.accident_time?,
                accident_type: value.accident_type?,
                confirmed_by: value.confirmed_by?,
                diversion_in_force: value.diversion_in_force?,
                diversion_route: value.diversion_route?,
                end_time: value.end_time?,
                event_id: value.event_id?,
                extensions: value.extensions?,
                external_country_code: value.external_country_code?,
                external_situation_id: value.external_situation_id?,
                external_situation_version: value.external_situation_version?,
                external_system_code: value.external_system_code?,
                lanes_affected: value.lanes_affected?,
                lanes_operational: value.lanes_operational?,
                lanes_restricted: value.lanes_restricted?,
                light_conditions: value.light_conditions?,
                location: value.location?,
                location_description: value.location_description?,
                modified_by: value.modified_by?,
                phase: value.phase?,
                public_description: value.public_description?,
                reported_by: value.reported_by?,
                road_conditions: value.road_conditions?,
                severity: value.severity?,
                statistic_date: value.statistic_date?,
                status: value.status?,
                vehicle_count: value.vehicle_count?,
                weather_conditions: value.weather_conditions?,
            })
        }
    }
    impl ::std::convert::From<super::AccidentEvent> for AccidentEvent {
        fn from(value: super::AccidentEvent) -> Self {
            Self {
                accident_time: Ok(value.accident_time),
                accident_type: Ok(value.accident_type),
                confirmed_by: Ok(value.confirmed_by),
                diversion_in_force: Ok(value.diversion_in_force),
                diversion_route: Ok(value.diversion_route),
                end_time: Ok(value.end_time),
                event_id: Ok(value.event_id),
                extensions: Ok(value.extensions),
                external_country_code: Ok(value.external_country_code),
                external_situation_id: Ok(value.external_situation_id),
                external_situation_version: Ok(value.external_situation_version),
                external_system_code: Ok(value.external_system_code),
                lanes_affected: Ok(value.lanes_affected),
                lanes_operational: Ok(value.lanes_operational),
                lanes_restricted: Ok(value.lanes_restricted),
                light_conditions: Ok(value.light_conditions),
                location: Ok(value.location),
                location_description: Ok(value.location_description),
                modified_by: Ok(value.modified_by),
                phase: Ok(value.phase),
                public_description: Ok(value.public_description),
                reported_by: Ok(value.reported_by),
                road_conditions: Ok(value.road_conditions),
                severity: Ok(value.severity),
                statistic_date: Ok(value.statistic_date),
                status: Ok(value.status),
                vehicle_count: Ok(value.vehicle_count),
                weather_conditions: Ok(value.weather_conditions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AllTypes {
        accident_event: ::std::result::Result<
            ::std::option::Option<super::AccidentEvent>,
            ::std::string::String,
        >,
        accident_type: ::std::result::Result<
            ::std::option::Option<super::AccidentType>,
            ::std::string::String,
        >,
        coordinates:
            ::std::result::Result<::std::option::Option<super::Coordinates>, ::std::string::String>,
        event_status:
            ::std::result::Result<::std::option::Option<super::EventStatus>, ::std::string::String>,
        incident_event: ::std::result::Result<
            ::std::option::Option<super::IncidentEvent>,
            ::std::string::String,
        >,
        incident_type: ::std::result::Result<
            ::std::option::Option<super::IncidentType>,
            ::std::string::String,
        >,
        lanes_affected_type: ::std::result::Result<
            ::std::option::Option<super::LanesAffectedType>,
            ::std::string::String,
        >,
        planned_event: ::std::result::Result<
            ::std::option::Option<super::PlannedEvent>,
            ::std::string::String,
        >,
        planned_event_type: ::std::result::Result<
            ::std::option::Option<super::PlannedEventType>,
            ::std::string::String,
        >,
        point: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        roadworks_event: ::std::result::Result<
            ::std::option::Option<super::RoadworksEvent>,
            ::std::string::String,
        >,
        roadworks_type: ::std::result::Result<
            ::std::option::Option<super::RoadworksType>,
            ::std::string::String,
        >,
        severity_type: ::std::result::Result<
            ::std::option::Option<super::SeverityType>,
            ::std::string::String,
        >,
        sync_category_enum: ::std::result::Result<
            ::std::option::Option<super::SyncCategoryEnum>,
            ::std::string::String,
        >,
        sync_complete: ::std::result::Result<
            ::std::option::Option<super::SyncComplete>,
            ::std::string::String,
        >,
        sync_request:
            ::std::result::Result<::std::option::Option<super::SyncRequest>, ::std::string::String>,
        sync_status:
            ::std::result::Result<::std::option::Option<super::SyncStatus>, ::std::string::String>,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                accident_event: Ok(Default::default()),
                accident_type: Ok(Default::default()),
                coordinates: Ok(Default::default()),
                event_status: Ok(Default::default()),
                incident_event: Ok(Default::default()),
                incident_type: Ok(Default::default()),
                lanes_affected_type: Ok(Default::default()),
                planned_event: Ok(Default::default()),
                planned_event_type: Ok(Default::default()),
                point: Ok(Default::default()),
                roadworks_event: Ok(Default::default()),
                roadworks_type: Ok(Default::default()),
                severity_type: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
            }
        }
    }
    impl AllTypes {
        pub fn accident_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AccidentEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.accident_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accident_event: {e}"));
            self
        }
        pub fn accident_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AccidentType>>,
            T::Error: ::std::fmt::Display,
        {
            self.accident_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accident_type: {e}"));
            self
        }
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Coordinates>>,
            T::Error: ::std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {e}"));
            self
        }
        pub fn event_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EventStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_status: {e}"));
            self
        }
        pub fn incident_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IncidentEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.incident_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incident_event: {e}"));
            self
        }
        pub fn incident_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::IncidentType>>,
            T::Error: ::std::fmt::Display,
        {
            self.incident_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incident_type: {e}"));
            self
        }
        pub fn lanes_affected_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanesAffectedType>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_affected_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for lanes_affected_type: {e}")
            });
            self
        }
        pub fn planned_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlannedEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.planned_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planned_event: {e}"));
            self
        }
        pub fn planned_event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlannedEventType>>,
            T::Error: ::std::fmt::Display,
        {
            self.planned_event_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for planned_event_type: {e}")
            });
            self
        }
        pub fn point<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.point = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for point: {e}"));
            self
        }
        pub fn roadworks_event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RoadworksEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.roadworks_event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roadworks_event: {e}"));
            self
        }
        pub fn roadworks_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RoadworksType>>,
            T::Error: ::std::fmt::Display,
        {
            self.roadworks_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roadworks_type: {e}"));
            self
        }
        pub fn severity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SeverityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity_type: {e}"));
            self
        }
        pub fn sync_category_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncCategoryEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_category_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for sync_category_enum: {e}")
            });
            self
        }
        pub fn sync_complete<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncComplete>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_complete = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_complete: {e}"));
            self
        }
        pub fn sync_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_request = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_request: {e}"));
            self
        }
        pub fn sync_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accident_event: value.accident_event?,
                accident_type: value.accident_type?,
                coordinates: value.coordinates?,
                event_status: value.event_status?,
                incident_event: value.incident_event?,
                incident_type: value.incident_type?,
                lanes_affected_type: value.lanes_affected_type?,
                planned_event: value.planned_event?,
                planned_event_type: value.planned_event_type?,
                point: value.point?,
                roadworks_event: value.roadworks_event?,
                roadworks_type: value.roadworks_type?,
                severity_type: value.severity_type?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                accident_event: Ok(value.accident_event),
                accident_type: Ok(value.accident_type),
                coordinates: Ok(value.coordinates),
                event_status: Ok(value.event_status),
                incident_event: Ok(value.incident_event),
                incident_type: Ok(value.incident_type),
                lanes_affected_type: Ok(value.lanes_affected_type),
                planned_event: Ok(value.planned_event),
                planned_event_type: Ok(value.planned_event_type),
                point: Ok(value.point),
                roadworks_event: Ok(value.roadworks_event),
                roadworks_type: Ok(value.roadworks_type),
                severity_type: Ok(value.severity_type),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Coordinates {
        latitude: ::std::result::Result<f64, ::std::string::String>,
        longitude: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Coordinates {
        fn default() -> Self {
            Self {
                latitude: Err("no value supplied for latitude".to_string()),
                longitude: Err("no value supplied for longitude".to_string()),
            }
        }
    }
    impl Coordinates {
        pub fn latitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.latitude = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for latitude: {e}"));
            self
        }
        pub fn longitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.longitude = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for longitude: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Coordinates> for super::Coordinates {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Coordinates,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                latitude: value.latitude?,
                longitude: value.longitude?,
            })
        }
    }
    impl ::std::convert::From<super::Coordinates> for Coordinates {
        fn from(value: super::Coordinates) -> Self {
            Self {
                latitude: Ok(value.latitude),
                longitude: Ok(value.longitude),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IncidentEvent {
        confirmed_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        diversion_in_force:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        diversion_route: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        end_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        external_country_code: ::std::result::Result<
            ::std::option::Option<super::IncidentEventExternalCountryCode>,
            ::std::string::String,
        >,
        external_situation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_situation_version:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        external_system_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        incident_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        incident_type: ::std::result::Result<super::IncidentType, ::std::string::String>,
        lanes_affected: ::std::result::Result<
            ::std::option::Option<super::LanesAffectedType>,
            ::std::string::String,
        >,
        lanes_operational: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        lanes_restricted: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        location_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        phase: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        public_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reported_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        severity: ::std::result::Result<
            ::std::option::Option<super::SeverityType>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::EventStatus, ::std::string::String>,
    }
    impl ::std::default::Default for IncidentEvent {
        fn default() -> Self {
            Self {
                confirmed_by: Ok(Default::default()),
                diversion_in_force: Ok(Default::default()),
                diversion_route: Ok(Default::default()),
                end_time: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                extensions: Ok(Default::default()),
                external_country_code: Ok(Default::default()),
                external_situation_id: Ok(Default::default()),
                external_situation_version: Ok(Default::default()),
                external_system_code: Ok(Default::default()),
                incident_time: Ok(Default::default()),
                incident_type: Err("no value supplied for incident_type".to_string()),
                lanes_affected: Ok(Default::default()),
                lanes_operational: Ok(Default::default()),
                lanes_restricted: Ok(Default::default()),
                location: Ok(Default::default()),
                location_description: Ok(Default::default()),
                modified_by: Ok(Default::default()),
                phase: Ok(Default::default()),
                public_description: Ok(Default::default()),
                reported_by: Ok(Default::default()),
                severity: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl IncidentEvent {
        pub fn confirmed_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.confirmed_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirmed_by: {e}"));
            self
        }
        pub fn diversion_in_force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_in_force = value.try_into().map_err(|e| {
                format!("error converting supplied value for diversion_in_force: {e}")
            });
            self
        }
        pub fn diversion_route<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_route = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diversion_route: {e}"));
            self
        }
        pub fn end_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.end_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_time: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn external_country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::IncidentEventExternalCountryCode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_country_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_country_code: {e}")
            });
            self
        }
        pub fn external_situation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_id: {e}")
            });
            self
        }
        pub fn external_situation_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_version = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_version: {e}")
            });
            self
        }
        pub fn external_system_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_system_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_system_code: {e}")
            });
            self
        }
        pub fn incident_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.incident_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incident_time: {e}"));
            self
        }
        pub fn incident_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IncidentType>,
            T::Error: ::std::fmt::Display,
        {
            self.incident_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for incident_type: {e}"));
            self
        }
        pub fn lanes_affected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanesAffectedType>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_affected: {e}"));
            self
        }
        pub fn lanes_operational<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_operational = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_operational: {e}"));
            self
        }
        pub fn lanes_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_restricted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_restricted: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn location_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for location_description: {e}")
            });
            self
        }
        pub fn modified_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified_by: {e}"));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {e}"));
            self
        }
        pub fn public_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.public_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for public_description: {e}")
            });
            self
        }
        pub fn reported_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reported_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reported_by: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SeverityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IncidentEvent> for super::IncidentEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IncidentEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                confirmed_by: value.confirmed_by?,
                diversion_in_force: value.diversion_in_force?,
                diversion_route: value.diversion_route?,
                end_time: value.end_time?,
                event_id: value.event_id?,
                extensions: value.extensions?,
                external_country_code: value.external_country_code?,
                external_situation_id: value.external_situation_id?,
                external_situation_version: value.external_situation_version?,
                external_system_code: value.external_system_code?,
                incident_time: value.incident_time?,
                incident_type: value.incident_type?,
                lanes_affected: value.lanes_affected?,
                lanes_operational: value.lanes_operational?,
                lanes_restricted: value.lanes_restricted?,
                location: value.location?,
                location_description: value.location_description?,
                modified_by: value.modified_by?,
                phase: value.phase?,
                public_description: value.public_description?,
                reported_by: value.reported_by?,
                severity: value.severity?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::IncidentEvent> for IncidentEvent {
        fn from(value: super::IncidentEvent) -> Self {
            Self {
                confirmed_by: Ok(value.confirmed_by),
                diversion_in_force: Ok(value.diversion_in_force),
                diversion_route: Ok(value.diversion_route),
                end_time: Ok(value.end_time),
                event_id: Ok(value.event_id),
                extensions: Ok(value.extensions),
                external_country_code: Ok(value.external_country_code),
                external_situation_id: Ok(value.external_situation_id),
                external_situation_version: Ok(value.external_situation_version),
                external_system_code: Ok(value.external_system_code),
                incident_time: Ok(value.incident_time),
                incident_type: Ok(value.incident_type),
                lanes_affected: Ok(value.lanes_affected),
                lanes_operational: Ok(value.lanes_operational),
                lanes_restricted: Ok(value.lanes_restricted),
                location: Ok(value.location),
                location_description: Ok(value.location_description),
                modified_by: Ok(value.modified_by),
                phase: Ok(value.phase),
                public_description: Ok(value.public_description),
                reported_by: Ok(value.reported_by),
                severity: Ok(value.severity),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PlannedEvent {
        actual_end: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        actual_start: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        confirmed_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        diversion_in_force:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        diversion_route: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        external_country_code: ::std::result::Result<
            ::std::option::Option<super::PlannedEventExternalCountryCode>,
            ::std::string::String,
        >,
        external_situation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_situation_version:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        external_system_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lanes_affected: ::std::result::Result<
            ::std::option::Option<super::LanesAffectedType>,
            ::std::string::String,
        >,
        lanes_operational: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        lanes_restricted: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        location_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        organiser: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        phase: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        planned_end: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        planned_event_type: ::std::result::Result<super::PlannedEventType, ::std::string::String>,
        planned_start: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        public_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reported_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        severity: ::std::result::Result<
            ::std::option::Option<super::SeverityType>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::EventStatus, ::std::string::String>,
        venue_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PlannedEvent {
        fn default() -> Self {
            Self {
                actual_end: Ok(Default::default()),
                actual_start: Ok(Default::default()),
                confirmed_by: Ok(Default::default()),
                diversion_in_force: Ok(Default::default()),
                diversion_route: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                extensions: Ok(Default::default()),
                external_country_code: Ok(Default::default()),
                external_situation_id: Ok(Default::default()),
                external_situation_version: Ok(Default::default()),
                external_system_code: Ok(Default::default()),
                lanes_affected: Ok(Default::default()),
                lanes_operational: Ok(Default::default()),
                lanes_restricted: Ok(Default::default()),
                location: Ok(Default::default()),
                location_description: Ok(Default::default()),
                modified_by: Ok(Default::default()),
                organiser: Ok(Default::default()),
                phase: Ok(Default::default()),
                planned_end: Ok(Default::default()),
                planned_event_type: Err("no value supplied for planned_event_type".to_string()),
                planned_start: Ok(Default::default()),
                public_description: Ok(Default::default()),
                reported_by: Ok(Default::default()),
                severity: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                venue_name: Ok(Default::default()),
            }
        }
    }
    impl PlannedEvent {
        pub fn actual_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.actual_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actual_end: {e}"));
            self
        }
        pub fn actual_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.actual_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actual_start: {e}"));
            self
        }
        pub fn confirmed_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.confirmed_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirmed_by: {e}"));
            self
        }
        pub fn diversion_in_force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_in_force = value.try_into().map_err(|e| {
                format!("error converting supplied value for diversion_in_force: {e}")
            });
            self
        }
        pub fn diversion_route<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_route = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diversion_route: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn external_country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::PlannedEventExternalCountryCode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_country_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_country_code: {e}")
            });
            self
        }
        pub fn external_situation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_id: {e}")
            });
            self
        }
        pub fn external_situation_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_version = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_version: {e}")
            });
            self
        }
        pub fn external_system_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_system_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_system_code: {e}")
            });
            self
        }
        pub fn lanes_affected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanesAffectedType>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_affected: {e}"));
            self
        }
        pub fn lanes_operational<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_operational = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_operational: {e}"));
            self
        }
        pub fn lanes_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_restricted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_restricted: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn location_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for location_description: {e}")
            });
            self
        }
        pub fn modified_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified_by: {e}"));
            self
        }
        pub fn organiser<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.organiser = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organiser: {e}"));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {e}"));
            self
        }
        pub fn planned_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.planned_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planned_end: {e}"));
            self
        }
        pub fn planned_event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PlannedEventType>,
            T::Error: ::std::fmt::Display,
        {
            self.planned_event_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for planned_event_type: {e}")
            });
            self
        }
        pub fn planned_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.planned_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planned_start: {e}"));
            self
        }
        pub fn public_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.public_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for public_description: {e}")
            });
            self
        }
        pub fn reported_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reported_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reported_by: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SeverityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn venue_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.venue_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for venue_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PlannedEvent> for super::PlannedEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PlannedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actual_end: value.actual_end?,
                actual_start: value.actual_start?,
                confirmed_by: value.confirmed_by?,
                diversion_in_force: value.diversion_in_force?,
                diversion_route: value.diversion_route?,
                event_id: value.event_id?,
                extensions: value.extensions?,
                external_country_code: value.external_country_code?,
                external_situation_id: value.external_situation_id?,
                external_situation_version: value.external_situation_version?,
                external_system_code: value.external_system_code?,
                lanes_affected: value.lanes_affected?,
                lanes_operational: value.lanes_operational?,
                lanes_restricted: value.lanes_restricted?,
                location: value.location?,
                location_description: value.location_description?,
                modified_by: value.modified_by?,
                organiser: value.organiser?,
                phase: value.phase?,
                planned_end: value.planned_end?,
                planned_event_type: value.planned_event_type?,
                planned_start: value.planned_start?,
                public_description: value.public_description?,
                reported_by: value.reported_by?,
                severity: value.severity?,
                status: value.status?,
                venue_name: value.venue_name?,
            })
        }
    }
    impl ::std::convert::From<super::PlannedEvent> for PlannedEvent {
        fn from(value: super::PlannedEvent) -> Self {
            Self {
                actual_end: Ok(value.actual_end),
                actual_start: Ok(value.actual_start),
                confirmed_by: Ok(value.confirmed_by),
                diversion_in_force: Ok(value.diversion_in_force),
                diversion_route: Ok(value.diversion_route),
                event_id: Ok(value.event_id),
                extensions: Ok(value.extensions),
                external_country_code: Ok(value.external_country_code),
                external_situation_id: Ok(value.external_situation_id),
                external_situation_version: Ok(value.external_situation_version),
                external_system_code: Ok(value.external_system_code),
                lanes_affected: Ok(value.lanes_affected),
                lanes_operational: Ok(value.lanes_operational),
                lanes_restricted: Ok(value.lanes_restricted),
                location: Ok(value.location),
                location_description: Ok(value.location_description),
                modified_by: Ok(value.modified_by),
                organiser: Ok(value.organiser),
                phase: Ok(value.phase),
                planned_end: Ok(value.planned_end),
                planned_event_type: Ok(value.planned_event_type),
                planned_start: Ok(value.planned_start),
                public_description: Ok(value.public_description),
                reported_by: Ok(value.reported_by),
                severity: Ok(value.severity),
                status: Ok(value.status),
                venue_name: Ok(value.venue_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Point {
        altitude_meters: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        coordinates: ::std::result::Result<super::Coordinates, ::std::string::String>,
    }
    impl ::std::default::Default for Point {
        fn default() -> Self {
            Self {
                altitude_meters: Ok(Default::default()),
                coordinates: Err("no value supplied for coordinates".to_string()),
            }
        }
    }
    impl Point {
        pub fn altitude_meters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.altitude_meters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for altitude_meters: {e}"));
            self
        }
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Coordinates>,
            T::Error: ::std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Point> for super::Point {
        type Error = super::error::ConversionError;
        fn try_from(value: Point) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                altitude_meters: value.altitude_meters?,
                coordinates: value.coordinates?,
            })
        }
    }
    impl ::std::convert::From<super::Point> for Point {
        fn from(value: super::Point) -> Self {
            Self {
                altitude_meters: Ok(value.altitude_meters),
                coordinates: Ok(value.coordinates),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RoadworksEvent {
        actual_end: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        actual_start: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        confirmed_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        confirmed_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        contractor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        contraflow: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        diversion_in_force:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        diversion_route: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        event_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        external_country_code: ::std::result::Result<
            ::std::option::Option<super::RoadworksEventExternalCountryCode>,
            ::std::string::String,
        >,
        external_situation_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        external_situation_version:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        external_system_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lanes_affected: ::std::result::Result<
            ::std::option::Option<super::LanesAffectedType>,
            ::std::string::String,
        >,
        lanes_operational: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        lanes_restricted: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        location_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        phase: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        planned_end: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        planned_start: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        public_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reported_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        roadworks_type: ::std::result::Result<super::RoadworksType, ::std::string::String>,
        severity: ::std::result::Result<
            ::std::option::Option<super::SeverityType>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::EventStatus, ::std::string::String>,
        traffic_signals: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for RoadworksEvent {
        fn default() -> Self {
            Self {
                actual_end: Ok(Default::default()),
                actual_start: Ok(Default::default()),
                confirmed_by: Ok(Default::default()),
                confirmed_date: Ok(Default::default()),
                contractor: Ok(Default::default()),
                contraflow: Ok(Default::default()),
                diversion_in_force: Ok(Default::default()),
                diversion_route: Ok(Default::default()),
                event_id: Err("no value supplied for event_id".to_string()),
                extensions: Ok(Default::default()),
                external_country_code: Ok(Default::default()),
                external_situation_id: Ok(Default::default()),
                external_situation_version: Ok(Default::default()),
                external_system_code: Ok(Default::default()),
                lanes_affected: Ok(Default::default()),
                lanes_operational: Ok(Default::default()),
                lanes_restricted: Ok(Default::default()),
                location: Ok(Default::default()),
                location_description: Ok(Default::default()),
                modified_by: Ok(Default::default()),
                phase: Ok(Default::default()),
                planned_end: Ok(Default::default()),
                planned_start: Ok(Default::default()),
                public_description: Ok(Default::default()),
                reported_by: Ok(Default::default()),
                roadworks_type: Err("no value supplied for roadworks_type".to_string()),
                severity: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                traffic_signals: Ok(Default::default()),
            }
        }
    }
    impl RoadworksEvent {
        pub fn actual_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.actual_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actual_end: {e}"));
            self
        }
        pub fn actual_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.actual_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actual_start: {e}"));
            self
        }
        pub fn confirmed_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.confirmed_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirmed_by: {e}"));
            self
        }
        pub fn confirmed_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.confirmed_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for confirmed_date: {e}"));
            self
        }
        pub fn contractor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.contractor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contractor: {e}"));
            self
        }
        pub fn contraflow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.contraflow = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contraflow: {e}"));
            self
        }
        pub fn diversion_in_force<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_in_force = value.try_into().map_err(|e| {
                format!("error converting supplied value for diversion_in_force: {e}")
            });
            self
        }
        pub fn diversion_route<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.diversion_route = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for diversion_route: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn external_country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RoadworksEventExternalCountryCode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.external_country_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_country_code: {e}")
            });
            self
        }
        pub fn external_situation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_id: {e}")
            });
            self
        }
        pub fn external_situation_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_situation_version = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_situation_version: {e}")
            });
            self
        }
        pub fn external_system_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.external_system_code = value.try_into().map_err(|e| {
                format!("error converting supplied value for external_system_code: {e}")
            });
            self
        }
        pub fn lanes_affected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LanesAffectedType>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_affected: {e}"));
            self
        }
        pub fn lanes_operational<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_operational = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_operational: {e}"));
            self
        }
        pub fn lanes_restricted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.lanes_restricted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lanes_restricted: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn location_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.location_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for location_description: {e}")
            });
            self
        }
        pub fn modified_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified_by: {e}"));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {e}"));
            self
        }
        pub fn planned_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.planned_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planned_end: {e}"));
            self
        }
        pub fn planned_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.planned_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for planned_start: {e}"));
            self
        }
        pub fn public_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.public_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for public_description: {e}")
            });
            self
        }
        pub fn reported_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reported_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reported_by: {e}"));
            self
        }
        pub fn roadworks_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RoadworksType>,
            T::Error: ::std::fmt::Display,
        {
            self.roadworks_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roadworks_type: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SeverityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn traffic_signals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signals = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signals: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RoadworksEvent> for super::RoadworksEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RoadworksEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actual_end: value.actual_end?,
                actual_start: value.actual_start?,
                confirmed_by: value.confirmed_by?,
                confirmed_date: value.confirmed_date?,
                contractor: value.contractor?,
                contraflow: value.contraflow?,
                diversion_in_force: value.diversion_in_force?,
                diversion_route: value.diversion_route?,
                event_id: value.event_id?,
                extensions: value.extensions?,
                external_country_code: value.external_country_code?,
                external_situation_id: value.external_situation_id?,
                external_situation_version: value.external_situation_version?,
                external_system_code: value.external_system_code?,
                lanes_affected: value.lanes_affected?,
                lanes_operational: value.lanes_operational?,
                lanes_restricted: value.lanes_restricted?,
                location: value.location?,
                location_description: value.location_description?,
                modified_by: value.modified_by?,
                phase: value.phase?,
                planned_end: value.planned_end?,
                planned_start: value.planned_start?,
                public_description: value.public_description?,
                reported_by: value.reported_by?,
                roadworks_type: value.roadworks_type?,
                severity: value.severity?,
                status: value.status?,
                traffic_signals: value.traffic_signals?,
            })
        }
    }
    impl ::std::convert::From<super::RoadworksEvent> for RoadworksEvent {
        fn from(value: super::RoadworksEvent) -> Self {
            Self {
                actual_end: Ok(value.actual_end),
                actual_start: Ok(value.actual_start),
                confirmed_by: Ok(value.confirmed_by),
                confirmed_date: Ok(value.confirmed_date),
                contractor: Ok(value.contractor),
                contraflow: Ok(value.contraflow),
                diversion_in_force: Ok(value.diversion_in_force),
                diversion_route: Ok(value.diversion_route),
                event_id: Ok(value.event_id),
                extensions: Ok(value.extensions),
                external_country_code: Ok(value.external_country_code),
                external_situation_id: Ok(value.external_situation_id),
                external_situation_version: Ok(value.external_situation_version),
                external_system_code: Ok(value.external_system_code),
                lanes_affected: Ok(value.lanes_affected),
                lanes_operational: Ok(value.lanes_operational),
                lanes_restricted: Ok(value.lanes_restricted),
                location: Ok(value.location),
                location_description: Ok(value.location_description),
                modified_by: Ok(value.modified_by),
                phase: Ok(value.phase),
                planned_end: Ok(value.planned_end),
                planned_start: Ok(value.planned_start),
                public_description: Ok(value.public_description),
                reported_by: Ok(value.reported_by),
                roadworks_type: Ok(value.roadworks_type),
                severity: Ok(value.severity),
                status: Ok(value.status),
                traffic_signals: Ok(value.traffic_signals),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncComplete {
        completed_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<super::SyncCompleteMessage>,
            ::std::string::String,
        >,
        object_count: ::std::result::Result<u64, ::std::string::String>,
        status: ::std::result::Result<super::SyncStatus, ::std::string::String>,
        sync_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        sync_id: ::std::result::Result<
            ::std::option::Option<super::SyncCompleteSyncId>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SyncComplete {
        fn default() -> Self {
            Self {
                completed_time: Err("no value supplied for completed_time".to_string()),
                extensions: Ok(Default::default()),
                message: Ok(Default::default()),
                object_count: Err("no value supplied for object_count".to_string()),
                status: Err("no value supplied for status".to_string()),
                sync_correlation_id: Err("no value supplied for sync_correlation_id".to_string()),
                sync_id: Ok(Default::default()),
            }
        }
    }
    impl SyncComplete {
        pub fn completed_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.completed_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for completed_time: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncCompleteMessage>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn object_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.object_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object_count: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SyncStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn sync_correlation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_correlation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for sync_correlation_id: {e}")
            });
            self
        }
        pub fn sync_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncCompleteSyncId>>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sync_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncComplete> for super::SyncComplete {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncComplete,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                completed_time: value.completed_time?,
                extensions: value.extensions?,
                message: value.message?,
                object_count: value.object_count?,
                status: value.status?,
                sync_correlation_id: value.sync_correlation_id?,
                sync_id: value.sync_id?,
            })
        }
    }
    impl ::std::convert::From<super::SyncComplete> for SyncComplete {
        fn from(value: super::SyncComplete) -> Self {
            Self {
                completed_time: Ok(value.completed_time),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                object_count: Ok(value.object_count),
                status: Ok(value.status),
                sync_correlation_id: Ok(value.sync_correlation_id),
                sync_id: Ok(value.sync_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SyncRequest {
        active_only: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        categories:
            ::std::result::Result<::std::vec::Vec<super::SyncCategoryEnum>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        object_ids: ::std::result::Result<::std::vec::Vec<super::ObjectId>, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        since: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        sub_area: ::std::result::Result<
            ::std::option::Option<super::SyncRequestSubArea>,
            ::std::string::String,
        >,
        sync_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        type_id: ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
    }
    impl ::std::default::Default for SyncRequest {
        fn default() -> Self {
            Self {
                active_only: Ok(Default::default()),
                categories: Ok(Default::default()),
                extensions: Ok(Default::default()),
                object_ids: Ok(Default::default()),
                request_time: Err("no value supplied for request_time".to_string()),
                since: Ok(Default::default()),
                sub_area: Ok(Default::default()),
                sync_correlation_id: Err("no value supplied for sync_correlation_id".to_string()),
                type_id: Ok(Default::default()),
            }
        }
    }
    impl SyncRequest {
        pub fn active_only<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.active_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for active_only: {e}"));
            self
        }
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SyncCategoryEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn object_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.object_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for object_ids: {e}"));
            self
        }
        pub fn request_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_time: {e}"));
            self
        }
        pub fn since<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.since = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for since: {e}"));
            self
        }
        pub fn sub_area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SyncRequestSubArea>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub_area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_area: {e}"));
            self
        }
        pub fn sync_correlation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.sync_correlation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for sync_correlation_id: {e}")
            });
            self
        }
        pub fn type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TypeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SyncRequest> for super::SyncRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SyncRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active_only: value.active_only?,
                categories: value.categories?,
                extensions: value.extensions?,
                object_ids: value.object_ids?,
                request_time: value.request_time?,
                since: value.since?,
                sub_area: value.sub_area?,
                sync_correlation_id: value.sync_correlation_id?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::SyncRequest> for SyncRequest {
        fn from(value: super::SyncRequest) -> Self {
            Self {
                active_only: Ok(value.active_only),
                categories: Ok(value.categories),
                extensions: Ok(value.extensions),
                object_ids: Ok(value.object_ids),
                request_time: Ok(value.request_time),
                since: Ok(value.since),
                sub_area: Ok(value.sub_area),
                sync_correlation_id: Ok(value.sync_correlation_id),
                type_id: Ok(value.type_id),
            }
        }
    }
}
