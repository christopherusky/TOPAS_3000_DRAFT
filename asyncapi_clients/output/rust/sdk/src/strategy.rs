/*
 * File:        output/rust/sdk/src/strategy.rs
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
#[doc = "`AllTypes`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"_AllTypes\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"CommandResponseStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"GeoJsonPoint\": {"]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
#[doc = "    },"]
#[doc = "    \"StrategyCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/StrategyCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"StrategyDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/StrategyDefinition\""]
#[doc = "    },"]
#[doc = "    \"StrategyStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/StrategyStatus\""]
#[doc = "    },"]
#[doc = "    \"StrategyStatusEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/StrategyStatusEnum\""]
#[doc = "    },"]
#[doc = "    \"StrategyTrigger\": {"]
#[doc = "      \"$ref\": \"#/$defs/StrategyTrigger\""]
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
#[doc = "    },"]
#[doc = "    \"TriggerStateEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TriggerStateEnum\""]
#[doc = "    },"]
#[doc = "    \"TriggerUpdateErrorEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TriggerUpdateErrorEnum\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AllTypes {
    #[serde(
        rename = "CommandResponseStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub command_response_status: ::std::option::Option<CommandResponseStatus>,
    #[serde(
        rename = "GeoJsonPoint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geo_json_point: ::std::option::Option<GeoJsonPoint>,
    #[serde(
        rename = "StrategyCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_command_response: ::std::option::Option<StrategyCommandResponse>,
    #[serde(
        rename = "StrategyDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_definition: ::std::option::Option<StrategyDefinition>,
    #[serde(
        rename = "StrategyStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_status: ::std::option::Option<StrategyStatus>,
    #[serde(
        rename = "StrategyStatusEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_status_enum: ::std::option::Option<StrategyStatusEnum>,
    #[serde(
        rename = "StrategyTrigger",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_trigger: ::std::option::Option<StrategyTrigger>,
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
    #[serde(
        rename = "TriggerStateEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_state_enum: ::std::option::Option<TriggerStateEnum>,
    #[serde(
        rename = "TriggerUpdateErrorEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_update_error_enum: ::std::option::Option<TriggerUpdateErrorEnum>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            command_response_status: Default::default(),
            geo_json_point: Default::default(),
            strategy_command_response: Default::default(),
            strategy_definition: Default::default(),
            strategy_status: Default::default(),
            strategy_status_enum: Default::default(),
            strategy_trigger: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
            trigger_state_enum: Default::default(),
            trigger_update_error_enum: Default::default(),
        }
    }
}
impl AllTypes {
    pub fn builder() -> builder::AllTypes {
        Default::default()
    }
}
#[doc = "Outcome of a command execution.\n\nEvery value except ``PENDING`` is terminal. Consumers SHOULD test for\n\"not pending\" rather than enumerating the terminal values, so that a\nfuture profile adding one does not strand them."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Outcome of a command execution.\\n\\nEvery value except ``PENDING`` is terminal. Consumers SHOULD test for\\n\\\"not pending\\\" rather than enumerating the terminal values, so that a\\nfuture profile adding one does not strand them.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\","]
#[doc = "    \"failed\","]
#[doc = "    \"timedOut\","]
#[doc = "    \"invalid\","]
#[doc = "    \"notSupported\","]
#[doc = "    \"pending\""]
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
pub enum CommandResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "timedOut")]
    TimedOut,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "notSupported")]
    NotSupported,
    #[serde(rename = "pending")]
    Pending,
}
impl ::std::fmt::Display for CommandResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => f.write_str("success"),
            Self::Failed => f.write_str("failed"),
            Self::TimedOut => f.write_str("timedOut"),
            Self::Invalid => f.write_str("invalid"),
            Self::NotSupported => f.write_str("notSupported"),
            Self::Pending => f.write_str("pending"),
        }
    }
}
impl ::std::str::FromStr for CommandResponseStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            "failed" => Ok(Self::Failed),
            "timedOut" => Ok(Self::TimedOut),
            "invalid" => Ok(Self::Invalid),
            "notSupported" => Ok(Self::NotSupported),
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CommandResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CommandResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CommandResponseStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A GeoJSON Point geometry (RFC 7946).\n\nCoordinates are [longitude, latitude] per GeoJSON specification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A GeoJSON Point geometry (RFC 7946).\\n\\nCoordinates are [longitude, latitude] per GeoJSON specification.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"coordinates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"coordinates\": {"]
#[doc = "      \"description\": \"[longitude, latitude] in WGS84 decimal degrees.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"maxItems\": 2,"]
#[doc = "      \"minItems\": 2,"]
#[doc = "      \"prefixItems\": ["]
#[doc = "        {"]
#[doc = "          \"maximum\": 180.0,"]
#[doc = "          \"minimum\": -180.0,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"maximum\": 90.0,"]
#[doc = "          \"minimum\": -90.0,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"default\": \"Point\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Point\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GeoJsonPoint {
    #[doc = "[longitude, latitude] in WGS84 decimal degrees."]
    pub coordinates: [::serde_json::Value; 2usize],
    #[serde(rename = "type", default = "defaults::geo_json_point_type")]
    pub type_: GeoJsonPointType,
}
impl GeoJsonPoint {
    pub fn builder() -> builder::GeoJsonPoint {
        Default::default()
    }
}
#[doc = "`GeoJsonPointType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"Point\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Point\""]
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
pub enum GeoJsonPointType {
    Point,
}
impl ::std::fmt::Display for GeoJsonPointType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Point => f.write_str("Point"),
        }
    }
}
impl ::std::str::FromStr for GeoJsonPointType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Point" => Ok(Self::Point),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GeoJsonPointType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GeoJsonPointType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GeoJsonPointType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for GeoJsonPointType {
    fn default() -> Self {
        GeoJsonPointType::Point
    }
}
#[doc = "Unique identifier of an object. String of 1-32 characters from A-Z, a-z, 0-9 and . _ : - (no spaces), unique within object type within system."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Unique identifier of an object. String of 1-32 characters from A-Z, a-z, 0-9 and . _ : - (no spaces), unique within object type within system.\","]
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
#[doc = "Strategy command response payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Command Response\","]
#[doc = "  \"description\": \"Strategy command response payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\","]
#[doc = "    \"strategyId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Machine-readable response code for programmatic handling.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"errorCode\": {"]
#[doc = "      \"description\": \"Strategy-specific rejection category. Only permitted when the outcome is not success or pending.\","]
#[doc = "      \"$ref\": \"#/$defs/TriggerUpdateErrorEnum\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable detail.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"responseTime\": {"]
#[doc = "      \"description\": \"When the response was generated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"serviceRequester\": {"]
#[doc = "      \"description\": \"Audit label copied from the command, if it carried one. Never used for authorisation or idempotency.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome of the command execution.\","]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"strategyId\": {"]
#[doc = "      \"description\": \"Identifier of the strategy the command targeted.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StrategyCommandResponse {
    #[doc = "Machine-readable response code for programmatic handling."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Strategy-specific rejection category. Only permitted when the outcome is not success or pending."]
    #[serde(
        rename = "errorCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_code: ::std::option::Option<TriggerUpdateErrorEnum>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Human-readable detail."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "When the response was generated."]
    #[serde(rename = "responseTime")]
    pub response_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Audit label copied from the command, if it carried one. Never used for authorisation or idempotency."]
    #[serde(
        rename = "serviceRequester",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_requester: ::std::option::Option<StrategyCommandResponseServiceRequester>,
    #[doc = "Outcome of the command execution."]
    pub status: CommandResponseStatus,
    #[doc = "Identifier of the strategy the command targeted."]
    #[serde(rename = "strategyId")]
    pub strategy_id: StrategyCommandResponseStrategyId,
}
impl StrategyCommandResponse {
    pub fn builder() -> builder::StrategyCommandResponse {
        Default::default()
    }
}
#[doc = "Audit label copied from the command, if it carried one. Never used for authorisation or idempotency."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Audit label copied from the command, if it carried one. Never used for authorisation or idempotency.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyCommandResponseServiceRequester(::std::string::String);
impl ::std::ops::Deref for StrategyCommandResponseServiceRequester {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyCommandResponseServiceRequester> for ::std::string::String {
    fn from(value: StrategyCommandResponseServiceRequester) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyCommandResponseServiceRequester {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyCommandResponseServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyCommandResponseServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyCommandResponseServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyCommandResponseServiceRequester {
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
#[doc = "Identifier of the strategy the command targeted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier of the strategy the command targeted.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyCommandResponseStrategyId(::std::string::String);
impl ::std::ops::Deref for StrategyCommandResponseStrategyId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyCommandResponseStrategyId> for ::std::string::String {
    fn from(value: StrategyCommandResponseStrategyId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyCommandResponseStrategyId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyCommandResponseStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyCommandResponseStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyCommandResponseStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyCommandResponseStrategyId {
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
#[doc = "Traffic management strategy static metadata."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Definition\","]
#[doc = "  \"description\": \"Traffic management strategy static metadata.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"strategyId\","]
#[doc = "    \"strategyName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Geographic centre of the strategy area as WGS84 GeoJSON Point.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
#[doc = "    },"]
#[doc = "    \"strategyDescription\": {"]
#[doc = "      \"description\": \"Longer description of what the strategy does.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"strategyId\": {"]
#[doc = "      \"description\": \"Strategy identifier (e.g. 'STR00000001').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"strategyName\": {"]
#[doc = "      \"description\": \"Human-readable name of the strategy.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StrategyDefinition {
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Geographic centre of the strategy area as WGS84 GeoJSON Point."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<GeoJsonPoint>,
    #[doc = "Longer description of what the strategy does."]
    #[serde(
        rename = "strategyDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub strategy_description: ::std::option::Option<::std::string::String>,
    #[doc = "Strategy identifier (e.g. 'STR00000001')."]
    #[serde(rename = "strategyId")]
    pub strategy_id: StrategyDefinitionStrategyId,
    #[doc = "Human-readable name of the strategy."]
    #[serde(rename = "strategyName")]
    pub strategy_name: StrategyDefinitionStrategyName,
}
impl StrategyDefinition {
    pub fn builder() -> builder::StrategyDefinition {
        Default::default()
    }
}
#[doc = "Strategy identifier (e.g. 'STR00000001')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Strategy identifier (e.g. 'STR00000001').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyDefinitionStrategyId(::std::string::String);
impl ::std::ops::Deref for StrategyDefinitionStrategyId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyDefinitionStrategyId> for ::std::string::String {
    fn from(value: StrategyDefinitionStrategyId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyDefinitionStrategyId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyDefinitionStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyDefinitionStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyDefinitionStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyDefinitionStrategyId {
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
#[doc = "Human-readable name of the strategy."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Human-readable name of the strategy.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyDefinitionStrategyName(::std::string::String);
impl ::std::ops::Deref for StrategyDefinitionStrategyName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyDefinitionStrategyName> for ::std::string::String {
    fn from(value: StrategyDefinitionStrategyName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyDefinitionStrategyName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyDefinitionStrategyName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyDefinitionStrategyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyDefinitionStrategyName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyDefinitionStrategyName {
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
#[doc = "Traffic management strategy current status."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Status\","]
#[doc = "  \"description\": \"Traffic management strategy current status.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"strategyChangeStateTime\","]
#[doc = "    \"strategyId\","]
#[doc = "    \"strategyStatus\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"errorMessage\": {"]
#[doc = "      \"description\": \"Error information (e.g. 'Strategy will never become active').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"statusMessage\": {"]
#[doc = "      \"description\": \"Non-error status information (e.g. 'Strategy applied 6 of 10 measures').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"strategyChangeStateTime\": {"]
#[doc = "      \"description\": \"When the strategy status changed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"strategyId\": {"]
#[doc = "      \"description\": \"Identifier of the strategy this status is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"strategyStatus\": {"]
#[doc = "      \"description\": \"Current status (active/inactive).\","]
#[doc = "      \"$ref\": \"#/$defs/StrategyStatusEnum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StrategyStatus {
    #[doc = "Error information (e.g. 'Strategy will never become active')."]
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Non-error status information (e.g. 'Strategy applied 6 of 10 measures')."]
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[doc = "When the strategy status changed."]
    #[serde(rename = "strategyChangeStateTime")]
    pub strategy_change_state_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the strategy this status is about."]
    #[serde(rename = "strategyId")]
    pub strategy_id: StrategyStatusStrategyId,
    #[doc = "Current status (active/inactive)."]
    #[serde(rename = "strategyStatus")]
    pub strategy_status: StrategyStatusEnum,
}
impl StrategyStatus {
    pub fn builder() -> builder::StrategyStatus {
        Default::default()
    }
}
#[doc = "Status of a traffic management strategy."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Status of a traffic management strategy.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"active\","]
#[doc = "    \"inactive\""]
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
pub enum StrategyStatusEnum {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}
impl ::std::fmt::Display for StrategyStatusEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Inactive => f.write_str("inactive"),
        }
    }
}
impl ::std::str::FromStr for StrategyStatusEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StrategyStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyStatusEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the strategy this status is about."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier of the strategy this status is about.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyStatusStrategyId(::std::string::String);
impl ::std::ops::Deref for StrategyStatusStrategyId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyStatusStrategyId> for ::std::string::String {
    fn from(value: StrategyStatusStrategyId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyStatusStrategyId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyStatusStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyStatusStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyStatusStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyStatusStrategyId {
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
#[doc = "Strategy trigger command payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Command\","]
#[doc = "  \"description\": \"Strategy trigger command payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"strategyId\","]
#[doc = "    \"triggerState\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"serviceRequester\": {"]
#[doc = "      \"description\": \"Audit label naming the requesting system. Sender-supplied, so it is never used for authorisation or idempotency.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"strategyId\": {"]
#[doc = "      \"description\": \"Identifier of the strategy to trigger.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"triggerState\": {"]
#[doc = "      \"description\": \"Desired trigger state (enabled/disabled).\","]
#[doc = "      \"$ref\": \"#/$defs/TriggerStateEnum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StrategyTrigger {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Audit label naming the requesting system. Sender-supplied, so it is never used for authorisation or idempotency."]
    #[serde(
        rename = "serviceRequester",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub service_requester: ::std::option::Option<StrategyTriggerServiceRequester>,
    #[doc = "Identifier of the strategy to trigger."]
    #[serde(rename = "strategyId")]
    pub strategy_id: StrategyTriggerStrategyId,
    #[doc = "Desired trigger state (enabled/disabled)."]
    #[serde(rename = "triggerState")]
    pub trigger_state: TriggerStateEnum,
}
impl StrategyTrigger {
    pub fn builder() -> builder::StrategyTrigger {
        Default::default()
    }
}
#[doc = "Audit label naming the requesting system. Sender-supplied, so it is never used for authorisation or idempotency."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Audit label naming the requesting system. Sender-supplied, so it is never used for authorisation or idempotency.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyTriggerServiceRequester(::std::string::String);
impl ::std::ops::Deref for StrategyTriggerServiceRequester {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyTriggerServiceRequester> for ::std::string::String {
    fn from(value: StrategyTriggerServiceRequester) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyTriggerServiceRequester {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyTriggerServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyTriggerServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyTriggerServiceRequester {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyTriggerServiceRequester {
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
#[doc = "Identifier of the strategy to trigger."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier of the strategy to trigger.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StrategyTriggerStrategyId(::std::string::String);
impl ::std::ops::Deref for StrategyTriggerStrategyId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StrategyTriggerStrategyId> for ::std::string::String {
    fn from(value: StrategyTriggerStrategyId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StrategyTriggerStrategyId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StrategyTriggerStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StrategyTriggerStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StrategyTriggerStrategyId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StrategyTriggerStrategyId {
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
#[doc = "Terminal message ending a strategy sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a strategy sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of strategy objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Strategy Sync Request\","]
#[doc = "  \"description\": \"Request the current state of strategy objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Desired state when triggering a strategy."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Desired state when triggering a strategy.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"enabled\","]
#[doc = "    \"disabled\""]
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
pub enum TriggerStateEnum {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for TriggerStateEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Enabled => f.write_str("enabled"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for TriggerStateEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TriggerStateEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TriggerStateEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TriggerStateEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Categorisation of strategy trigger rejection reason."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Categorisation of strategy trigger rejection reason.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"accessDenied\","]
#[doc = "    \"notAuthenticated\","]
#[doc = "    \"strategyIdDoesNotExist\","]
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
pub enum TriggerUpdateErrorEnum {
    #[serde(rename = "accessDenied")]
    AccessDenied,
    #[serde(rename = "notAuthenticated")]
    NotAuthenticated,
    #[serde(rename = "strategyIdDoesNotExist")]
    StrategyIdDoesNotExist,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for TriggerUpdateErrorEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AccessDenied => f.write_str("accessDenied"),
            Self::NotAuthenticated => f.write_str("notAuthenticated"),
            Self::StrategyIdDoesNotExist => f.write_str("strategyIdDoesNotExist"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for TriggerUpdateErrorEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "accessDenied" => Ok(Self::AccessDenied),
            "notAuthenticated" => Ok(Self::NotAuthenticated),
            "strategyIdDoesNotExist" => Ok(Self::StrategyIdDoesNotExist),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TriggerUpdateErrorEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TriggerUpdateErrorEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TriggerUpdateErrorEnum {
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
    pub struct AllTypes {
        command_response_status: ::std::result::Result<
            ::std::option::Option<super::CommandResponseStatus>,
            ::std::string::String,
        >,
        geo_json_point: ::std::result::Result<
            ::std::option::Option<super::GeoJsonPoint>,
            ::std::string::String,
        >,
        strategy_command_response: ::std::result::Result<
            ::std::option::Option<super::StrategyCommandResponse>,
            ::std::string::String,
        >,
        strategy_definition: ::std::result::Result<
            ::std::option::Option<super::StrategyDefinition>,
            ::std::string::String,
        >,
        strategy_status: ::std::result::Result<
            ::std::option::Option<super::StrategyStatus>,
            ::std::string::String,
        >,
        strategy_status_enum: ::std::result::Result<
            ::std::option::Option<super::StrategyStatusEnum>,
            ::std::string::String,
        >,
        strategy_trigger: ::std::result::Result<
            ::std::option::Option<super::StrategyTrigger>,
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
        trigger_state_enum: ::std::result::Result<
            ::std::option::Option<super::TriggerStateEnum>,
            ::std::string::String,
        >,
        trigger_update_error_enum: ::std::result::Result<
            ::std::option::Option<super::TriggerUpdateErrorEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                command_response_status: Ok(Default::default()),
                geo_json_point: Ok(Default::default()),
                strategy_command_response: Ok(Default::default()),
                strategy_definition: Ok(Default::default()),
                strategy_status: Ok(Default::default()),
                strategy_status_enum: Ok(Default::default()),
                strategy_trigger: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
                trigger_state_enum: Ok(Default::default()),
                trigger_update_error_enum: Ok(Default::default()),
            }
        }
    }
    impl AllTypes {
        pub fn command_response_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CommandResponseStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.command_response_status = value.try_into().map_err(|e| {
                format!("error converting supplied value for command_response_status: {e}")
            });
            self
        }
        pub fn geo_json_point<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonPoint>>,
            T::Error: ::std::fmt::Display,
        {
            self.geo_json_point = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geo_json_point: {e}"));
            self
        }
        pub fn strategy_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrategyCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for strategy_command_response: {e}")
            });
            self
        }
        pub fn strategy_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrategyDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for strategy_definition: {e}")
            });
            self
        }
        pub fn strategy_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrategyStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_status: {e}"));
            self
        }
        pub fn strategy_status_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrategyStatusEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_status_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for strategy_status_enum: {e}")
            });
            self
        }
        pub fn strategy_trigger<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StrategyTrigger>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_trigger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_trigger: {e}"));
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
        pub fn trigger_state_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TriggerStateEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.trigger_state_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for trigger_state_enum: {e}")
            });
            self
        }
        pub fn trigger_update_error_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TriggerUpdateErrorEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.trigger_update_error_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for trigger_update_error_enum: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_response_status: value.command_response_status?,
                geo_json_point: value.geo_json_point?,
                strategy_command_response: value.strategy_command_response?,
                strategy_definition: value.strategy_definition?,
                strategy_status: value.strategy_status?,
                strategy_status_enum: value.strategy_status_enum?,
                strategy_trigger: value.strategy_trigger?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
                trigger_state_enum: value.trigger_state_enum?,
                trigger_update_error_enum: value.trigger_update_error_enum?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                command_response_status: Ok(value.command_response_status),
                geo_json_point: Ok(value.geo_json_point),
                strategy_command_response: Ok(value.strategy_command_response),
                strategy_definition: Ok(value.strategy_definition),
                strategy_status: Ok(value.strategy_status),
                strategy_status_enum: Ok(value.strategy_status_enum),
                strategy_trigger: Ok(value.strategy_trigger),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
                trigger_state_enum: Ok(value.trigger_state_enum),
                trigger_update_error_enum: Ok(value.trigger_update_error_enum),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeoJsonPoint {
        coordinates: ::std::result::Result<[::serde_json::Value; 2usize], ::std::string::String>,
        type_: ::std::result::Result<super::GeoJsonPointType, ::std::string::String>,
    }
    impl ::std::default::Default for GeoJsonPoint {
        fn default() -> Self {
            Self {
                coordinates: Err("no value supplied for coordinates".to_string()),
                type_: Ok(super::defaults::geo_json_point_type()),
            }
        }
    }
    impl GeoJsonPoint {
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[::serde_json::Value; 2usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GeoJsonPointType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GeoJsonPoint> for super::GeoJsonPoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeoJsonPoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                coordinates: value.coordinates?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GeoJsonPoint> for GeoJsonPoint {
        fn from(value: super::GeoJsonPoint) -> Self {
            Self {
                coordinates: Ok(value.coordinates),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StrategyCommandResponse {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        error_code: ::std::result::Result<
            ::std::option::Option<super::TriggerUpdateErrorEnum>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        response_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        service_requester: ::std::result::Result<
            ::std::option::Option<super::StrategyCommandResponseServiceRequester>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::CommandResponseStatus, ::std::string::String>,
        strategy_id:
            ::std::result::Result<super::StrategyCommandResponseStrategyId, ::std::string::String>,
    }
    impl ::std::default::Default for StrategyCommandResponse {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                error_code: Ok(Default::default()),
                extensions: Ok(Default::default()),
                message: Ok(Default::default()),
                response_time: Err("no value supplied for response_time".to_string()),
                service_requester: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                strategy_id: Err("no value supplied for strategy_id".to_string()),
            }
        }
    }
    impl StrategyCommandResponse {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn command_correlation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.command_correlation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for command_correlation_id: {e}")
            });
            self
        }
        pub fn error_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TriggerUpdateErrorEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_code: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn response_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.response_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_time: {e}"));
            self
        }
        pub fn service_requester<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StrategyCommandResponseServiceRequester>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.service_requester = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_requester: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CommandResponseStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn strategy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyCommandResponseStrategyId>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StrategyCommandResponse> for super::StrategyCommandResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StrategyCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                error_code: value.error_code?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                service_requester: value.service_requester?,
                status: value.status?,
                strategy_id: value.strategy_id?,
            })
        }
    }
    impl ::std::convert::From<super::StrategyCommandResponse> for StrategyCommandResponse {
        fn from(value: super::StrategyCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                error_code: Ok(value.error_code),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                service_requester: Ok(value.service_requester),
                status: Ok(value.status),
                strategy_id: Ok(value.strategy_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StrategyDefinition {
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        location: ::std::result::Result<
            ::std::option::Option<super::GeoJsonPoint>,
            ::std::string::String,
        >,
        strategy_description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        strategy_id:
            ::std::result::Result<super::StrategyDefinitionStrategyId, ::std::string::String>,
        strategy_name:
            ::std::result::Result<super::StrategyDefinitionStrategyName, ::std::string::String>,
    }
    impl ::std::default::Default for StrategyDefinition {
        fn default() -> Self {
            Self {
                extensions: Ok(Default::default()),
                location: Ok(Default::default()),
                strategy_description: Ok(Default::default()),
                strategy_id: Err("no value supplied for strategy_id".to_string()),
                strategy_name: Err("no value supplied for strategy_name".to_string()),
            }
        }
    }
    impl StrategyDefinition {
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
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonPoint>>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn strategy_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_description = value.try_into().map_err(|e| {
                format!("error converting supplied value for strategy_description: {e}")
            });
            self
        }
        pub fn strategy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyDefinitionStrategyId>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_id: {e}"));
            self
        }
        pub fn strategy_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyDefinitionStrategyName>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StrategyDefinition> for super::StrategyDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StrategyDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extensions: value.extensions?,
                location: value.location?,
                strategy_description: value.strategy_description?,
                strategy_id: value.strategy_id?,
                strategy_name: value.strategy_name?,
            })
        }
    }
    impl ::std::convert::From<super::StrategyDefinition> for StrategyDefinition {
        fn from(value: super::StrategyDefinition) -> Self {
            Self {
                extensions: Ok(value.extensions),
                location: Ok(value.location),
                strategy_description: Ok(value.strategy_description),
                strategy_id: Ok(value.strategy_id),
                strategy_name: Ok(value.strategy_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StrategyStatus {
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        status_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        strategy_change_state_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        strategy_id: ::std::result::Result<super::StrategyStatusStrategyId, ::std::string::String>,
        strategy_status: ::std::result::Result<super::StrategyStatusEnum, ::std::string::String>,
    }
    impl ::std::default::Default for StrategyStatus {
        fn default() -> Self {
            Self {
                error_message: Ok(Default::default()),
                extensions: Ok(Default::default()),
                status_message: Ok(Default::default()),
                strategy_change_state_time: Err(
                    "no value supplied for strategy_change_state_time".to_string()
                ),
                strategy_id: Err("no value supplied for strategy_id".to_string()),
                strategy_status: Err("no value supplied for strategy_status".to_string()),
            }
        }
    }
    impl StrategyStatus {
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {e}"));
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
        pub fn status_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_message: {e}"));
            self
        }
        pub fn strategy_change_state_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_change_state_time = value.try_into().map_err(|e| {
                format!("error converting supplied value for strategy_change_state_time: {e}")
            });
            self
        }
        pub fn strategy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyStatusStrategyId>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_id: {e}"));
            self
        }
        pub fn strategy_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyStatusEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_status: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StrategyStatus> for super::StrategyStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StrategyStatus,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error_message: value.error_message?,
                extensions: value.extensions?,
                status_message: value.status_message?,
                strategy_change_state_time: value.strategy_change_state_time?,
                strategy_id: value.strategy_id?,
                strategy_status: value.strategy_status?,
            })
        }
    }
    impl ::std::convert::From<super::StrategyStatus> for StrategyStatus {
        fn from(value: super::StrategyStatus) -> Self {
            Self {
                error_message: Ok(value.error_message),
                extensions: Ok(value.extensions),
                status_message: Ok(value.status_message),
                strategy_change_state_time: Ok(value.strategy_change_state_time),
                strategy_id: Ok(value.strategy_id),
                strategy_status: Ok(value.strategy_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StrategyTrigger {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        service_requester: ::std::result::Result<
            ::std::option::Option<super::StrategyTriggerServiceRequester>,
            ::std::string::String,
        >,
        strategy_id: ::std::result::Result<super::StrategyTriggerStrategyId, ::std::string::String>,
        trigger_state: ::std::result::Result<super::TriggerStateEnum, ::std::string::String>,
    }
    impl ::std::default::Default for StrategyTrigger {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                request_time: Err("no value supplied for request_time".to_string()),
                service_requester: Ok(Default::default()),
                strategy_id: Err("no value supplied for strategy_id".to_string()),
                trigger_state: Err("no value supplied for trigger_state".to_string()),
            }
        }
    }
    impl StrategyTrigger {
        pub fn command_correlation_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.command_correlation_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for command_correlation_id: {e}")
            });
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
        pub fn service_requester<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::StrategyTriggerServiceRequester>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.service_requester = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for service_requester: {e}"));
            self
        }
        pub fn strategy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StrategyTriggerStrategyId>,
            T::Error: ::std::fmt::Display,
        {
            self.strategy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for strategy_id: {e}"));
            self
        }
        pub fn trigger_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TriggerStateEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.trigger_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trigger_state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StrategyTrigger> for super::StrategyTrigger {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StrategyTrigger,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                request_time: value.request_time?,
                service_requester: value.service_requester?,
                strategy_id: value.strategy_id?,
                trigger_state: value.trigger_state?,
            })
        }
    }
    impl ::std::convert::From<super::StrategyTrigger> for StrategyTrigger {
        fn from(value: super::StrategyTrigger) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                request_time: Ok(value.request_time),
                service_requester: Ok(value.service_requester),
                strategy_id: Ok(value.strategy_id),
                trigger_state: Ok(value.trigger_state),
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
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn geo_json_point_type() -> super::GeoJsonPointType {
        super::GeoJsonPointType::Point
    }
}
