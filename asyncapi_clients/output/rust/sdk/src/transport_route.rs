/*
 * File:        output/rust/sdk/src/transport_route.rs
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
#[doc = "Acknowledgement state of a device fault.\n\nMaps from legacy XSD integer values:\n- 0 -> notAcknowledged\n- 1 -> acknowledged\n- 2 -> deferred\n- 3 -> recordedForAction"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Acknowledgement state of a device fault.\\n\\nMaps from legacy XSD integer values:\\n- 0 -> notAcknowledged\\n- 1 -> acknowledged\\n- 2 -> deferred\\n- 3 -> recordedForAction\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"notAcknowledged\","]
#[doc = "    \"acknowledged\","]
#[doc = "    \"deferred\","]
#[doc = "    \"recordedForAction\""]
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
pub enum AcknowledgementState {
    #[serde(rename = "notAcknowledged")]
    NotAcknowledged,
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "recordedForAction")]
    RecordedForAction,
}
impl ::std::fmt::Display for AcknowledgementState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotAcknowledged => f.write_str("notAcknowledged"),
            Self::Acknowledged => f.write_str("acknowledged"),
            Self::Deferred => f.write_str("deferred"),
            Self::RecordedForAction => f.write_str("recordedForAction"),
        }
    }
}
impl ::std::str::FromStr for AcknowledgementState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "notAcknowledged" => Ok(Self::NotAcknowledged),
            "acknowledged" => Ok(Self::Acknowledged),
            "deferred" => Ok(Self::Deferred),
            "recordedForAction" => Ok(Self::RecordedForAction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AcknowledgementState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AcknowledgementState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AcknowledgementState {
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
#[doc = "    \"AcknowledgementState\": {"]
#[doc = "      \"$ref\": \"#/$defs/AcknowledgementState\""]
#[doc = "    },"]
#[doc = "    \"CommandResponseStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"Coordinates\": {"]
#[doc = "      \"$ref\": \"#/$defs/Coordinates\""]
#[doc = "    },"]
#[doc = "    \"FaultStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultStatus\""]
#[doc = "    },"]
#[doc = "    \"Point\": {"]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"RequestTransportRouteStatusCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/RequestTransportRouteStatusCommand\""]
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
#[doc = "    \"TransportRouteCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteCommand\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteDefinition\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteDynamic\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteFault\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteFault\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteHistoryEntry\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteHistoryEntry\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteSegmentConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteSegmentConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteStatus\""]
#[doc = "    },"]
#[doc = "    \"TransportRouteType\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteType\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AllTypes {
    #[serde(
        rename = "AcknowledgementState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_state: ::std::option::Option<AcknowledgementState>,
    #[serde(
        rename = "CommandResponseStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub command_response_status: ::std::option::Option<CommandResponseStatus>,
    #[serde(
        rename = "Coordinates",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub coordinates: ::std::option::Option<Coordinates>,
    #[serde(
        rename = "FaultStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_status: ::std::option::Option<FaultStatus>,
    #[serde(
        rename = "Point",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub point: ::std::option::Option<Point>,
    #[serde(
        rename = "Quality",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quality: ::std::option::Option<Quality>,
    #[serde(
        rename = "RequestTransportRouteStatusCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_transport_route_status_command:
        ::std::option::Option<RequestTransportRouteStatusCommand>,
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
        rename = "TransportRouteCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_command: ::std::option::Option<TransportRouteCommand>,
    #[serde(
        rename = "TransportRouteCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_command_response: ::std::option::Option<TransportRouteCommandResponse>,
    #[serde(
        rename = "TransportRouteConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_configuration: ::std::option::Option<TransportRouteConfiguration>,
    #[serde(
        rename = "TransportRouteDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_definition: ::std::option::Option<TransportRouteDefinition>,
    #[serde(
        rename = "TransportRouteDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_dynamic: ::std::option::Option<TransportRouteDynamic>,
    #[serde(
        rename = "TransportRouteFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_fault: ::std::option::Option<TransportRouteFault>,
    #[serde(
        rename = "TransportRouteHistoryEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_history_entry: ::std::option::Option<TransportRouteHistoryEntry>,
    #[serde(
        rename = "TransportRouteSegmentConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_segment_configuration:
        ::std::option::Option<TransportRouteSegmentConfiguration>,
    #[serde(
        rename = "TransportRouteStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_status: ::std::option::Option<TransportRouteStatus>,
    #[serde(
        rename = "TransportRouteType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_route_type: ::std::option::Option<TransportRouteType>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            acknowledgement_state: Default::default(),
            command_response_status: Default::default(),
            coordinates: Default::default(),
            fault_status: Default::default(),
            point: Default::default(),
            quality: Default::default(),
            request_transport_route_status_command: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
            transport_route_command: Default::default(),
            transport_route_command_response: Default::default(),
            transport_route_configuration: Default::default(),
            transport_route_definition: Default::default(),
            transport_route_dynamic: Default::default(),
            transport_route_fault: Default::default(),
            transport_route_history_entry: Default::default(),
            transport_route_segment_configuration: Default::default(),
            transport_route_status: Default::default(),
            transport_route_type: Default::default(),
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
#[doc = "Whether a fault is still active or has been cleared.\n\nThis is the only fault lifecycle the payload carries. Record archival\n— distinguishing a live record from a retired one — is not modelled\nhere and is a matter for the publishing system."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Whether a fault is still active or has been cleared.\\n\\nThis is the only fault lifecycle the payload carries. Record archival\\n— distinguishing a live record from a retired one — is not modelled\\nhere and is a matter for the publishing system.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"active\","]
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
pub enum FaultStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "cleared")]
    Cleared,
}
impl ::std::fmt::Display for FaultStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Cleared => f.write_str("cleared"),
        }
    }
}
impl ::std::str::FromStr for FaultStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "cleared" => Ok(Self::Cleared),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FaultStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FaultStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FaultStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the route receiving the command."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Identifier of the route receiving the command.\","]
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
#[doc = "Identifier of the UTMC participant system that owns this quality statement."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ParticipantId\","]
#[doc = "  \"description\": \"Identifier of the UTMC participant system that owns this quality statement.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ParticipantId(::std::string::String);
impl ::std::ops::Deref for ParticipantId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParticipantId> for ::std::string::String {
    fn from(value: ParticipantId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ParticipantId {
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
impl ::std::convert::TryFrom<&str> for ParticipantId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParticipantId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParticipantId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParticipantId {
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
#[doc = "Quality statement attached to a UTMC data object or measurement."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Quality statement attached to a UTMC data object or measurement.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"qualityId\","]
#[doc = "    \"sourceName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"lastUpdated\": {"]
#[doc = "      \"description\": \"When this quality statement was last updated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"participantId\": {"]
#[doc = "      \"title\": \"ParticipantId\","]
#[doc = "      \"description\": \"Identifier of the UTMC participant system that owns this quality statement.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"qualityId\": {"]
#[doc = "      \"title\": \"QualityId\","]
#[doc = "      \"description\": \"Identifier of this Quality statement.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"sourceName\": {"]
#[doc = "      \"description\": \"Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no source cannot be interpreted.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"sourceType\": {"]
#[doc = "      \"description\": \"Free-text description of the source kind.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Quality {
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "When this quality statement was last updated."]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Identifier of the UTMC participant system that owns this quality statement."]
    #[serde(
        rename = "participantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Identifier of this Quality statement."]
    #[serde(rename = "qualityId")]
    pub quality_id: i64,
    #[doc = "Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no source cannot be interpreted."]
    #[serde(rename = "sourceName")]
    pub source_name: QualitySourceName,
    #[doc = "Free-text description of the source kind."]
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_type: ::std::option::Option<::std::string::String>,
}
impl Quality {
    pub fn builder() -> builder::Quality {
        Default::default()
    }
}
#[doc = "Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no source cannot be interpreted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no source cannot be interpreted.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct QualitySourceName(::std::string::String);
impl ::std::ops::Deref for QualitySourceName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<QualitySourceName> for ::std::string::String {
    fn from(value: QualitySourceName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for QualitySourceName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for QualitySourceName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for QualitySourceName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QualitySourceName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for QualitySourceName {
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
#[doc = "Request the current status of a transport route."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request the current status of a transport route.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"transportRouteId\""]
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
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestTransportRouteStatusCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"requestStatus\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the route receiving the command.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RequestTransportRouteStatusCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestTransportRouteStatusCommand payload; it is not free-text command content."]
    pub kind: RequestTransportRouteStatusCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the route receiving the command."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl RequestTransportRouteStatusCommand {
    pub fn builder() -> builder::RequestTransportRouteStatusCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestTransportRouteStatusCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestTransportRouteStatusCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"requestStatus\""]
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
pub enum RequestTransportRouteStatusCommandKind {
    #[serde(rename = "requestStatus")]
    RequestStatus,
}
impl ::std::fmt::Display for RequestTransportRouteStatusCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RequestStatus => f.write_str("requestStatus"),
        }
    }
}
impl ::std::str::FromStr for RequestTransportRouteStatusCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requestStatus" => Ok(Self::RequestStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequestTransportRouteStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequestTransportRouteStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestTransportRouteStatusCommandKind {
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
#[doc = "Terminal message ending a transport route sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport route Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a transport route sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of transport route objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport route Sync Request\","]
#[doc = "  \"description\": \"Request the current state of transport route objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Transport route command payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Command\","]
#[doc = "  \"description\": \"Transport route command payloads.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RequestTransportRouteStatusCommand\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"requestStatus\": \"#/$defs/RequestTransportRouteStatusCommand\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TransportRouteCommand(pub RequestTransportRouteStatusCommand);
impl ::std::ops::Deref for TransportRouteCommand {
    type Target = RequestTransportRouteStatusCommand;
    fn deref(&self) -> &RequestTransportRouteStatusCommand {
        &self.0
    }
}
impl ::std::convert::From<TransportRouteCommand> for RequestTransportRouteStatusCommand {
    fn from(value: TransportRouteCommand) -> Self {
        value.0
    }
}
impl ::std::convert::From<RequestTransportRouteStatusCommand> for TransportRouteCommand {
    fn from(value: RequestTransportRouteStatusCommand) -> Self {
        Self(value)
    }
}
#[doc = "Transport route command response payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Command Response\","]
#[doc = "  \"description\": \"Transport route command response payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Machine-readable response code.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable response message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"responseTime\": {"]
#[doc = "      \"description\": \"When the response was generated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Outcome of the command execution.\","]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the route the command targeted.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteCommandResponse {
    #[doc = "Machine-readable response code."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Human-readable response message."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "When the response was generated."]
    #[serde(rename = "responseTime")]
    pub response_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Outcome of the command execution."]
    pub status: CommandResponseStatus,
    #[doc = "Identifier of the route the command targeted."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteCommandResponse {
    pub fn builder() -> builder::TransportRouteCommandResponse {
        Default::default()
    }
}
#[doc = "Transport route configuration and segment mapping."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Configuration\","]
#[doc = "  \"description\": \"Transport route configuration and segment mapping.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"routeEnd\": {"]
#[doc = "      \"description\": \"Route end description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"routeLengthMeters\": {"]
#[doc = "      \"description\": \"Route length in meters.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"routeName\": {"]
#[doc = "      \"description\": \"Human-readable route name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"routeStart\": {"]
#[doc = "      \"description\": \"Route start description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"routeVia\": {"]
#[doc = "      \"description\": \"Via point description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Route this configuration applies to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteConfiguration {
    #[doc = "When this configuration became effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Route end description."]
    #[serde(
        rename = "routeEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_end: ::std::option::Option<::std::string::String>,
    #[doc = "Route length in meters."]
    #[serde(
        rename = "routeLengthMeters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_length_meters: ::std::option::Option<f64>,
    #[doc = "Human-readable route name."]
    #[serde(
        rename = "routeName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_name: ::std::option::Option<::std::string::String>,
    #[doc = "Route start description."]
    #[serde(
        rename = "routeStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_start: ::std::option::Option<::std::string::String>,
    #[doc = "Via point description."]
    #[serde(
        rename = "routeVia",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_via: ::std::option::Option<::std::string::String>,
    #[doc = "Route this configuration applies to."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteConfiguration {
    pub fn builder() -> builder::TransportRouteConfiguration {
        Default::default()
    }
}
#[doc = "Transport route equipment details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Definition\","]
#[doc = "  \"description\": \"Transport route equipment details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"transportRouteId\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When this definition was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"endPoint\": {"]
#[doc = "      \"description\": \"Route endpoint location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Start point location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"longDescription\": {"]
#[doc = "      \"description\": \"Longer description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 2000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"originatingObjectId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"ID on originating system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"originatingParticipantId\": {"]
#[doc = "      \"title\": \"ParticipantId\","]
#[doc = "      \"description\": \"Upstream system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"ownerOperator\": {"]
#[doc = "      \"description\": \"Owner or operator.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"participantId\": {"]
#[doc = "      \"title\": \"ParticipantId\","]
#[doc = "      \"description\": \"Owning system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"description\": \"Short name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this route.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Unique identifier for a transport route type (road, bus, rail, ...).\","]
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
pub struct TransportRouteDefinition {
    #[doc = "When this definition was created."]
    #[serde(rename = "creationDate")]
    pub creation_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Route endpoint location."]
    #[serde(
        rename = "endPoint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_point: ::std::option::Option<Point>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Start point location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Longer description."]
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<TransportRouteDefinitionLongDescription>,
    #[doc = "ID on originating system."]
    #[serde(
        rename = "originatingObjectId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub originating_object_id: ::std::option::Option<ObjectId>,
    #[doc = "Upstream system."]
    #[serde(
        rename = "originatingParticipantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub originating_participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Owner or operator."]
    #[serde(
        rename = "ownerOperator",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub owner_operator: ::std::option::Option<::std::string::String>,
    #[doc = "Owning system."]
    #[serde(
        rename = "participantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Short name."]
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<TransportRouteDefinitionShortDescription>,
    #[doc = "Unique identifier of this route."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
    #[doc = "Unique identifier for a transport route type (road, bus, rail, ...)."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl TransportRouteDefinition {
    pub fn builder() -> builder::TransportRouteDefinition {
        Default::default()
    }
}
#[doc = "Longer description."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Longer description.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 2000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportRouteDefinitionLongDescription(::std::string::String);
impl ::std::ops::Deref for TransportRouteDefinitionLongDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportRouteDefinitionLongDescription> for ::std::string::String {
    fn from(value: TransportRouteDefinitionLongDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportRouteDefinitionLongDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 2000usize {
            return Err("longer than 2000 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TransportRouteDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportRouteDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportRouteDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportRouteDefinitionLongDescription {
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
#[doc = "Short name."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short name.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportRouteDefinitionShortDescription(::std::string::String);
impl ::std::ops::Deref for TransportRouteDefinitionShortDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportRouteDefinitionShortDescription> for ::std::string::String {
    fn from(value: TransportRouteDefinitionShortDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportRouteDefinitionShortDescription {
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
impl ::std::convert::TryFrom<&str> for TransportRouteDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportRouteDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportRouteDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportRouteDefinitionShortDescription {
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
#[doc = "Transport route dynamic measurement data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Measurement\","]
#[doc = "  \"description\": \"Transport route dynamic measurement data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"timestamp\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"currentJourneyTimeSeconds\": {"]
#[doc = "      \"description\": \"Current journey time in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"estimatedSpeedKph\": {"]
#[doc = "      \"description\": \"Estimated speed along the route in km/h.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"routeStatus\": {"]
#[doc = "      \"description\": \"Route status (inRange/outOfRange).\","]
#[doc = "      \"$ref\": \"#/$defs/TransportRouteStatus\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time of the observation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the route.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteDynamic {
    #[doc = "Current journey time in seconds."]
    #[serde(
        rename = "currentJourneyTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_journey_time_seconds: ::std::option::Option<f64>,
    #[doc = "Estimated speed along the route in km/h."]
    #[serde(
        rename = "estimatedSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub estimated_speed_kph: ::std::option::Option<f64>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Optional quality statement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Route status (inRange/outOfRange)."]
    #[serde(
        rename = "routeStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub route_status: ::std::option::Option<TransportRouteStatus>,
    #[doc = "Time of the observation."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the route."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteDynamic {
    pub fn builder() -> builder::TransportRouteDynamic {
        Default::default()
    }
}
#[doc = "Transport route fault reports."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Fault\","]
#[doc = "  \"description\": \"Transport route fault reports.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"faultId\","]
#[doc = "    \"faultTypeId\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"acknowledgementDate\": {"]
#[doc = "      \"description\": \"When acknowledged.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"acknowledgementState\": {"]
#[doc = "      \"description\": \"Acknowledgement state.\","]
#[doc = "      \"$ref\": \"#/$defs/AcknowledgementState\""]
#[doc = "    },"]
#[doc = "    \"clearedDate\": {"]
#[doc = "      \"description\": \"When cleared.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"communicationsFault\": {"]
#[doc = "      \"description\": \"Whether this is a communications fault. See equipmentFault for how it relates to the catalogue category.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When the fault was first raised.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Free-text description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"equipmentFault\": {"]
#[doc = "      \"description\": \"Whether this is an equipment fault. The fault type's catalogue category classifies the type; this flag classifies this fault.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"faultId\": {"]
#[doc = "      \"description\": \"Unique identifier of this fault report.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"faultTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Reference to a FaultTypeDefinition entry.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"lastUpdated\": {"]
#[doc = "      \"description\": \"When last modified.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"description\": \"Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Fault severity.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Active or cleared.\","]
#[doc = "      \"$ref\": \"#/$defs/FaultStatus\""]
#[doc = "    },"]
#[doc = "    \"subsystemTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Reference to a SubsystemDefinition entry: the subsystem the fault arose in.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"supplierFaultNumber\": {"]
#[doc = "      \"description\": \"Fault identifier allocated by the supplier or configuration manager for maintenance purposes, if required.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the route this fault is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteFault {
    #[doc = "When acknowledged."]
    #[serde(
        rename = "acknowledgementDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Acknowledgement state."]
    #[serde(
        rename = "acknowledgementState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_state: ::std::option::Option<AcknowledgementState>,
    #[doc = "When cleared."]
    #[serde(
        rename = "clearedDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cleared_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Whether this is a communications fault. See equipmentFault for how it relates to the catalogue category."]
    #[serde(
        rename = "communicationsFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub communications_fault: ::std::option::Option<bool>,
    #[doc = "When the fault was first raised."]
    #[serde(rename = "creationDate")]
    pub creation_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Free-text description."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Whether this is an equipment fault. The fault type's catalogue category classifies the type; this flag classifies this fault."]
    #[serde(
        rename = "equipmentFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub equipment_fault: ::std::option::Option<bool>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Unique identifier of this fault report."]
    #[serde(rename = "faultId")]
    pub fault_id: u64,
    #[doc = "Reference to a FaultTypeDefinition entry."]
    #[serde(rename = "faultTypeId")]
    pub fault_type_id: TypeId,
    #[doc = "When last modified."]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<::std::string::String>,
    #[doc = "Fault severity."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<u64>,
    #[doc = "Active or cleared."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<FaultStatus>,
    #[doc = "Reference to a SubsystemDefinition entry: the subsystem the fault arose in."]
    #[serde(
        rename = "subsystemTypeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subsystem_type_id: ::std::option::Option<TypeId>,
    #[doc = "Fault identifier allocated by the supplier or configuration manager for maintenance purposes, if required."]
    #[serde(
        rename = "supplierFaultNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supplier_fault_number: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the route this fault is about."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteFault {
    pub fn builder() -> builder::TransportRouteFault {
        Default::default()
    }
}
#[doc = "Transport route history entries."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route History\","]
#[doc = "  \"description\": \"Transport route history entries.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"logDate\","]
#[doc = "    \"logEntry\","]
#[doc = "    \"logId\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"eventType\": {"]
#[doc = "      \"description\": \"Category of the history event.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"logDate\": {"]
#[doc = "      \"description\": \"When the log entry was recorded.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"logEntry\": {"]
#[doc = "      \"description\": \"Equipment log data.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"logId\": {"]
#[doc = "      \"description\": \"Unique identifier of this log entry.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"logUser\": {"]
#[doc = "      \"description\": \"User who created the entry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the route.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteHistoryEntry {
    #[doc = "Category of the history event."]
    #[serde(
        rename = "eventType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event_type: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "When the log entry was recorded."]
    #[serde(rename = "logDate")]
    pub log_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Equipment log data."]
    #[serde(rename = "logEntry")]
    pub log_entry: ::std::string::String,
    #[doc = "Unique identifier of this log entry."]
    #[serde(rename = "logId")]
    pub log_id: u64,
    #[doc = "User who created the entry."]
    #[serde(
        rename = "logUser",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub log_user: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the route."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteHistoryEntry {
    pub fn builder() -> builder::TransportRouteHistoryEntry {
        Default::default()
    }
}
#[doc = "Segment configuration — links a route segment to a transport link.\n\nSibling type in the configuration file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Segment configuration — links a route segment to a transport link.\\n\\nSibling type in the configuration file.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"routeSegmentId\","]
#[doc = "    \"transportLinkId\","]
#[doc = "    \"transportRouteId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"routeSegmentId\": {"]
#[doc = "      \"description\": \"Ordered segment identifier within the route.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Cross-domain reference to a transport link.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"transportRouteId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Route this segment belongs to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportRouteSegmentConfiguration {
    #[doc = "When effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Ordered segment identifier within the route."]
    #[serde(rename = "routeSegmentId")]
    pub route_segment_id: u64,
    #[doc = "Cross-domain reference to a transport link."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
    #[doc = "Route this segment belongs to."]
    #[serde(rename = "transportRouteId")]
    pub transport_route_id: ObjectId,
}
impl TransportRouteSegmentConfiguration {
    pub fn builder() -> builder::TransportRouteSegmentConfiguration {
        Default::default()
    }
}
#[doc = "Route status (a closed string enum, not an extensible TypeId catalogue)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Route status (a closed string enum, not an extensible TypeId catalogue).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inRange\","]
#[doc = "    \"outOfRange\""]
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
pub enum TransportRouteStatus {
    #[serde(rename = "inRange")]
    InRange,
    #[serde(rename = "outOfRange")]
    OutOfRange,
}
impl ::std::fmt::Display for TransportRouteStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::InRange => f.write_str("inRange"),
            Self::OutOfRange => f.write_str("outOfRange"),
        }
    }
}
impl ::std::str::FromStr for TransportRouteStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inRange" => Ok(Self::InRange),
            "outOfRange" => Ok(Self::OutOfRange),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportRouteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportRouteStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportRouteStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Transport route type definitions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Route Type\","]
#[doc = "  \"description\": \"Transport route type definitions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"description\": \"Additional notes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"specificationLink\": {"]
#[doc = "      \"description\": \"URL to specification.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this transport route type.\","]
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
pub struct TransportRouteType {
    #[doc = "Human-readable description."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Additional notes."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "URL to specification."]
    #[serde(
        rename = "specificationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_link: ::std::option::Option<TransportRouteTypeSpecificationLink>,
    #[doc = "Identifier of this transport route type."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl TransportRouteType {
    pub fn builder() -> builder::TransportRouteType {
        Default::default()
    }
}
#[doc = "URL to specification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL to specification.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportRouteTypeSpecificationLink(::std::string::String);
impl ::std::ops::Deref for TransportRouteTypeSpecificationLink {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportRouteTypeSpecificationLink> for ::std::string::String {
    fn from(value: TransportRouteTypeSpecificationLink) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportRouteTypeSpecificationLink {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TransportRouteTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportRouteTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportRouteTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportRouteTypeSpecificationLink {
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
        acknowledgement_state: ::std::result::Result<
            ::std::option::Option<super::AcknowledgementState>,
            ::std::string::String,
        >,
        command_response_status: ::std::result::Result<
            ::std::option::Option<super::CommandResponseStatus>,
            ::std::string::String,
        >,
        coordinates:
            ::std::result::Result<::std::option::Option<super::Coordinates>, ::std::string::String>,
        fault_status:
            ::std::result::Result<::std::option::Option<super::FaultStatus>, ::std::string::String>,
        point: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        request_transport_route_status_command: ::std::result::Result<
            ::std::option::Option<super::RequestTransportRouteStatusCommand>,
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
        transport_route_command: ::std::result::Result<
            ::std::option::Option<super::TransportRouteCommand>,
            ::std::string::String,
        >,
        transport_route_command_response: ::std::result::Result<
            ::std::option::Option<super::TransportRouteCommandResponse>,
            ::std::string::String,
        >,
        transport_route_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportRouteConfiguration>,
            ::std::string::String,
        >,
        transport_route_definition: ::std::result::Result<
            ::std::option::Option<super::TransportRouteDefinition>,
            ::std::string::String,
        >,
        transport_route_dynamic: ::std::result::Result<
            ::std::option::Option<super::TransportRouteDynamic>,
            ::std::string::String,
        >,
        transport_route_fault: ::std::result::Result<
            ::std::option::Option<super::TransportRouteFault>,
            ::std::string::String,
        >,
        transport_route_history_entry: ::std::result::Result<
            ::std::option::Option<super::TransportRouteHistoryEntry>,
            ::std::string::String,
        >,
        transport_route_segment_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportRouteSegmentConfiguration>,
            ::std::string::String,
        >,
        transport_route_status: ::std::result::Result<
            ::std::option::Option<super::TransportRouteStatus>,
            ::std::string::String,
        >,
        transport_route_type: ::std::result::Result<
            ::std::option::Option<super::TransportRouteType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                acknowledgement_state: Ok(Default::default()),
                command_response_status: Ok(Default::default()),
                coordinates: Ok(Default::default()),
                fault_status: Ok(Default::default()),
                point: Ok(Default::default()),
                quality: Ok(Default::default()),
                request_transport_route_status_command: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
                transport_route_command: Ok(Default::default()),
                transport_route_command_response: Ok(Default::default()),
                transport_route_configuration: Ok(Default::default()),
                transport_route_definition: Ok(Default::default()),
                transport_route_dynamic: Ok(Default::default()),
                transport_route_fault: Ok(Default::default()),
                transport_route_history_entry: Ok(Default::default()),
                transport_route_segment_configuration: Ok(Default::default()),
                transport_route_status: Ok(Default::default()),
                transport_route_type: Ok(Default::default()),
            }
        }
    }
    impl AllTypes {
        pub fn acknowledgement_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AcknowledgementState>>,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledgement_state = value.try_into().map_err(|e| {
                format!("error converting supplied value for acknowledgement_state: {e}")
            });
            self
        }
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
        pub fn fault_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fault_status: {e}"));
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
        pub fn quality<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Quality>>,
            T::Error: ::std::fmt::Display,
        {
            self.quality = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quality: {e}"));
            self
        }
        pub fn request_transport_route_status_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RequestTransportRouteStatusCommand>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . request_transport_route_status_command = value . try_into () . map_err (| e | format ! ("error converting supplied value for request_transport_route_status_command: {e}")) ;
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
        pub fn transport_route_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_command: {e}")
            });
            self
        }
        pub fn transport_route_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_command_response: {e}")
            });
            self
        }
        pub fn transport_route_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_configuration: {e}")
            });
            self
        }
        pub fn transport_route_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_definition: {e}")
            });
            self
        }
        pub fn transport_route_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_dynamic = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_dynamic: {e}")
            });
            self
        }
        pub fn transport_route_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteFault>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_fault = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_fault: {e}")
            });
            self
        }
        pub fn transport_route_history_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteHistoryEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_history_entry = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_history_entry: {e}")
            });
            self
        }
        pub fn transport_route_segment_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportRouteSegmentConfiguration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . transport_route_segment_configuration = value . try_into () . map_err (| e | format ! ("error converting supplied value for transport_route_segment_configuration: {e}")) ;
            self
        }
        pub fn transport_route_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_status = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_status: {e}")
            });
            self
        }
        pub fn transport_route_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteType>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_type: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_state: value.acknowledgement_state?,
                command_response_status: value.command_response_status?,
                coordinates: value.coordinates?,
                fault_status: value.fault_status?,
                point: value.point?,
                quality: value.quality?,
                request_transport_route_status_command: value
                    .request_transport_route_status_command?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
                transport_route_command: value.transport_route_command?,
                transport_route_command_response: value.transport_route_command_response?,
                transport_route_configuration: value.transport_route_configuration?,
                transport_route_definition: value.transport_route_definition?,
                transport_route_dynamic: value.transport_route_dynamic?,
                transport_route_fault: value.transport_route_fault?,
                transport_route_history_entry: value.transport_route_history_entry?,
                transport_route_segment_configuration: value
                    .transport_route_segment_configuration?,
                transport_route_status: value.transport_route_status?,
                transport_route_type: value.transport_route_type?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                acknowledgement_state: Ok(value.acknowledgement_state),
                command_response_status: Ok(value.command_response_status),
                coordinates: Ok(value.coordinates),
                fault_status: Ok(value.fault_status),
                point: Ok(value.point),
                quality: Ok(value.quality),
                request_transport_route_status_command: Ok(
                    value.request_transport_route_status_command
                ),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
                transport_route_command: Ok(value.transport_route_command),
                transport_route_command_response: Ok(value.transport_route_command_response),
                transport_route_configuration: Ok(value.transport_route_configuration),
                transport_route_definition: Ok(value.transport_route_definition),
                transport_route_dynamic: Ok(value.transport_route_dynamic),
                transport_route_fault: Ok(value.transport_route_fault),
                transport_route_history_entry: Ok(value.transport_route_history_entry),
                transport_route_segment_configuration: Ok(
                    value.transport_route_segment_configuration
                ),
                transport_route_status: Ok(value.transport_route_status),
                transport_route_type: Ok(value.transport_route_type),
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
    pub struct Quality {
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        last_updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        participant_id: ::std::result::Result<
            ::std::option::Option<super::ParticipantId>,
            ::std::string::String,
        >,
        quality_id: ::std::result::Result<i64, ::std::string::String>,
        source_name: ::std::result::Result<super::QualitySourceName, ::std::string::String>,
        source_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Quality {
        fn default() -> Self {
            Self {
                extensions: Ok(Default::default()),
                last_updated: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                quality_id: Err("no value supplied for quality_id".to_string()),
                source_name: Err("no value supplied for source_name".to_string()),
                source_type: Ok(Default::default()),
            }
        }
    }
    impl Quality {
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
        pub fn last_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_updated: {e}"));
            self
        }
        pub fn participant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ParticipantId>>,
            T::Error: ::std::fmt::Display,
        {
            self.participant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participant_id: {e}"));
            self
        }
        pub fn quality_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.quality_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quality_id: {e}"));
            self
        }
        pub fn source_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::QualitySourceName>,
            T::Error: ::std::fmt::Display,
        {
            self.source_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_name: {e}"));
            self
        }
        pub fn source_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Quality> for super::Quality {
        type Error = super::error::ConversionError;
        fn try_from(value: Quality) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extensions: value.extensions?,
                last_updated: value.last_updated?,
                participant_id: value.participant_id?,
                quality_id: value.quality_id?,
                source_name: value.source_name?,
                source_type: value.source_type?,
            })
        }
    }
    impl ::std::convert::From<super::Quality> for Quality {
        fn from(value: super::Quality) -> Self {
            Self {
                extensions: Ok(value.extensions),
                last_updated: Ok(value.last_updated),
                participant_id: Ok(value.participant_id),
                quality_id: Ok(value.quality_id),
                source_name: Ok(value.source_name),
                source_type: Ok(value.source_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestTransportRouteStatusCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<
            super::RequestTransportRouteStatusCommandKind,
            ::std::string::String,
        >,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for RequestTransportRouteStatusCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl RequestTransportRouteStatusCommand {
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RequestTransportRouteStatusCommandKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
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
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<RequestTransportRouteStatusCommand>
        for super::RequestTransportRouteStatusCommand
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestTransportRouteStatusCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::RequestTransportRouteStatusCommand>
        for RequestTransportRouteStatusCommand
    {
        fn from(value: super::RequestTransportRouteStatusCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                transport_route_id: Ok(value.transport_route_id),
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
    #[derive(Clone, Debug)]
    pub struct TransportRouteCommandResponse {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
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
        status: ::std::result::Result<super::CommandResponseStatus, ::std::string::String>,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteCommandResponse {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                message: Ok(Default::default()),
                response_time: Err("no value supplied for response_time".to_string()),
                status: Err("no value supplied for status".to_string()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteCommandResponse {
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
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteCommandResponse>
        for super::TransportRouteCommandResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                status: value.status?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteCommandResponse> for TransportRouteCommandResponse {
        fn from(value: super::TransportRouteCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                status: Ok(value.status),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteConfiguration {
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        route_end: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        route_length_meters:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        route_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        route_start: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        route_via: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteConfiguration {
        fn default() -> Self {
            Self {
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                extensions: Ok(Default::default()),
                route_end: Ok(Default::default()),
                route_length_meters: Ok(Default::default()),
                route_name: Ok(Default::default()),
                route_start: Ok(Default::default()),
                route_via: Ok(Default::default()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteConfiguration {
        pub fn configuration_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.configuration_date = value.try_into().map_err(|e| {
                format!("error converting supplied value for configuration_date: {e}")
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
        pub fn route_end<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_end = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_end: {e}"));
            self
        }
        pub fn route_length_meters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_length_meters = value.try_into().map_err(|e| {
                format!("error converting supplied value for route_length_meters: {e}")
            });
            self
        }
        pub fn route_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_name: {e}"));
            self
        }
        pub fn route_start<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_start: {e}"));
            self
        }
        pub fn route_via<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_via = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_via: {e}"));
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteConfiguration> for super::TransportRouteConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration_date: value.configuration_date?,
                extensions: value.extensions?,
                route_end: value.route_end?,
                route_length_meters: value.route_length_meters?,
                route_name: value.route_name?,
                route_start: value.route_start?,
                route_via: value.route_via?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteConfiguration> for TransportRouteConfiguration {
        fn from(value: super::TransportRouteConfiguration) -> Self {
            Self {
                configuration_date: Ok(value.configuration_date),
                extensions: Ok(value.extensions),
                route_end: Ok(value.route_end),
                route_length_meters: Ok(value.route_length_meters),
                route_name: Ok(value.route_name),
                route_start: Ok(value.route_start),
                route_via: Ok(value.route_via),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteDefinition {
        creation_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        end_point:
            ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        long_description: ::std::result::Result<
            ::std::option::Option<super::TransportRouteDefinitionLongDescription>,
            ::std::string::String,
        >,
        originating_object_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        originating_participant_id: ::std::result::Result<
            ::std::option::Option<super::ParticipantId>,
            ::std::string::String,
        >,
        owner_operator: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        participant_id: ::std::result::Result<
            ::std::option::Option<super::ParticipantId>,
            ::std::string::String,
        >,
        short_description: ::std::result::Result<
            ::std::option::Option<super::TransportRouteDefinitionShortDescription>,
            ::std::string::String,
        >,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteDefinition {
        fn default() -> Self {
            Self {
                creation_date: Err("no value supplied for creation_date".to_string()),
                end_point: Ok(Default::default()),
                extensions: Ok(Default::default()),
                location: Ok(Default::default()),
                long_description: Ok(Default::default()),
                originating_object_id: Ok(Default::default()),
                originating_participant_id: Ok(Default::default()),
                owner_operator: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                short_description: Ok(Default::default()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl TransportRouteDefinition {
        pub fn creation_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.creation_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for creation_date: {e}"));
            self
        }
        pub fn end_point<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.end_point = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_point: {e}"));
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
        pub fn long_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportRouteDefinitionLongDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.long_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_description: {e}"));
            self
        }
        pub fn originating_object_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.originating_object_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for originating_object_id: {e}")
            });
            self
        }
        pub fn originating_participant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ParticipantId>>,
            T::Error: ::std::fmt::Display,
        {
            self.originating_participant_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for originating_participant_id: {e}")
            });
            self
        }
        pub fn owner_operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner_operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner_operator: {e}"));
            self
        }
        pub fn participant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ParticipantId>>,
            T::Error: ::std::fmt::Display,
        {
            self.participant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participant_id: {e}"));
            self
        }
        pub fn short_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportRouteDefinitionShortDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.short_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_description: {e}"));
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
        pub fn type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeId>,
            T::Error: ::std::fmt::Display,
        {
            self.type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteDefinition> for super::TransportRouteDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                creation_date: value.creation_date?,
                end_point: value.end_point?,
                extensions: value.extensions?,
                location: value.location?,
                long_description: value.long_description?,
                originating_object_id: value.originating_object_id?,
                originating_participant_id: value.originating_participant_id?,
                owner_operator: value.owner_operator?,
                participant_id: value.participant_id?,
                short_description: value.short_description?,
                transport_route_id: value.transport_route_id?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteDefinition> for TransportRouteDefinition {
        fn from(value: super::TransportRouteDefinition) -> Self {
            Self {
                creation_date: Ok(value.creation_date),
                end_point: Ok(value.end_point),
                extensions: Ok(value.extensions),
                location: Ok(value.location),
                long_description: Ok(value.long_description),
                originating_object_id: Ok(value.originating_object_id),
                originating_participant_id: Ok(value.originating_participant_id),
                owner_operator: Ok(value.owner_operator),
                participant_id: Ok(value.participant_id),
                short_description: Ok(value.short_description),
                transport_route_id: Ok(value.transport_route_id),
                type_id: Ok(value.type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteDynamic {
        current_journey_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        estimated_speed_kph:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        route_status: ::std::result::Result<
            ::std::option::Option<super::TransportRouteStatus>,
            ::std::string::String,
        >,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteDynamic {
        fn default() -> Self {
            Self {
                current_journey_time_seconds: Ok(Default::default()),
                estimated_speed_kph: Ok(Default::default()),
                extensions: Ok(Default::default()),
                quality: Ok(Default::default()),
                route_status: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteDynamic {
        pub fn current_journey_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.current_journey_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for current_journey_time_seconds: {e}")
            });
            self
        }
        pub fn estimated_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.estimated_speed_kph = value.try_into().map_err(|e| {
                format!("error converting supplied value for estimated_speed_kph: {e}")
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
        pub fn quality<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Quality>>,
            T::Error: ::std::fmt::Display,
        {
            self.quality = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quality: {e}"));
            self
        }
        pub fn route_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportRouteStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.route_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_status: {e}"));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {e}"));
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteDynamic> for super::TransportRouteDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current_journey_time_seconds: value.current_journey_time_seconds?,
                estimated_speed_kph: value.estimated_speed_kph?,
                extensions: value.extensions?,
                quality: value.quality?,
                route_status: value.route_status?,
                timestamp: value.timestamp?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteDynamic> for TransportRouteDynamic {
        fn from(value: super::TransportRouteDynamic) -> Self {
            Self {
                current_journey_time_seconds: Ok(value.current_journey_time_seconds),
                estimated_speed_kph: Ok(value.estimated_speed_kph),
                extensions: Ok(value.extensions),
                quality: Ok(value.quality),
                route_status: Ok(value.route_status),
                timestamp: Ok(value.timestamp),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteFault {
        acknowledgement_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        acknowledgement_state: ::std::result::Result<
            ::std::option::Option<super::AcknowledgementState>,
            ::std::string::String,
        >,
        cleared_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        communications_fault:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        creation_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        equipment_fault: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        fault_id: ::std::result::Result<u64, ::std::string::String>,
        fault_type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
        last_updated: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        parameters:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        severity: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        status:
            ::std::result::Result<::std::option::Option<super::FaultStatus>, ::std::string::String>,
        subsystem_type_id:
            ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
        supplier_fault_number: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteFault {
        fn default() -> Self {
            Self {
                acknowledgement_date: Ok(Default::default()),
                acknowledgement_state: Ok(Default::default()),
                cleared_date: Ok(Default::default()),
                communications_fault: Ok(Default::default()),
                creation_date: Err("no value supplied for creation_date".to_string()),
                description: Ok(Default::default()),
                equipment_fault: Ok(Default::default()),
                extensions: Ok(Default::default()),
                fault_id: Err("no value supplied for fault_id".to_string()),
                fault_type_id: Err("no value supplied for fault_type_id".to_string()),
                last_updated: Ok(Default::default()),
                parameters: Ok(Default::default()),
                severity: Ok(Default::default()),
                status: Ok(Default::default()),
                subsystem_type_id: Ok(Default::default()),
                supplier_fault_number: Ok(Default::default()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteFault {
        pub fn acknowledgement_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledgement_date = value.try_into().map_err(|e| {
                format!("error converting supplied value for acknowledgement_date: {e}")
            });
            self
        }
        pub fn acknowledgement_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AcknowledgementState>>,
            T::Error: ::std::fmt::Display,
        {
            self.acknowledgement_state = value.try_into().map_err(|e| {
                format!("error converting supplied value for acknowledgement_state: {e}")
            });
            self
        }
        pub fn cleared_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cleared_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cleared_date: {e}"));
            self
        }
        pub fn communications_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.communications_fault = value.try_into().map_err(|e| {
                format!("error converting supplied value for communications_fault: {e}")
            });
            self
        }
        pub fn creation_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.creation_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for creation_date: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn equipment_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.equipment_fault = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for equipment_fault: {e}"));
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
        pub fn fault_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fault_id: {e}"));
            self
        }
        pub fn fault_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeId>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fault_type_id: {e}"));
            self
        }
        pub fn last_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_updated: {e}"));
            self
        }
        pub fn parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parameters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parameters: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn subsystem_type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TypeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.subsystem_type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subsystem_type_id: {e}"));
            self
        }
        pub fn supplier_fault_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supplier_fault_number = value.try_into().map_err(|e| {
                format!("error converting supplied value for supplier_fault_number: {e}")
            });
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteFault> for super::TransportRouteFault {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteFault,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_date: value.acknowledgement_date?,
                acknowledgement_state: value.acknowledgement_state?,
                cleared_date: value.cleared_date?,
                communications_fault: value.communications_fault?,
                creation_date: value.creation_date?,
                description: value.description?,
                equipment_fault: value.equipment_fault?,
                extensions: value.extensions?,
                fault_id: value.fault_id?,
                fault_type_id: value.fault_type_id?,
                last_updated: value.last_updated?,
                parameters: value.parameters?,
                severity: value.severity?,
                status: value.status?,
                subsystem_type_id: value.subsystem_type_id?,
                supplier_fault_number: value.supplier_fault_number?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteFault> for TransportRouteFault {
        fn from(value: super::TransportRouteFault) -> Self {
            Self {
                acknowledgement_date: Ok(value.acknowledgement_date),
                acknowledgement_state: Ok(value.acknowledgement_state),
                cleared_date: Ok(value.cleared_date),
                communications_fault: Ok(value.communications_fault),
                creation_date: Ok(value.creation_date),
                description: Ok(value.description),
                equipment_fault: Ok(value.equipment_fault),
                extensions: Ok(value.extensions),
                fault_id: Ok(value.fault_id),
                fault_type_id: Ok(value.fault_type_id),
                last_updated: Ok(value.last_updated),
                parameters: Ok(value.parameters),
                severity: Ok(value.severity),
                status: Ok(value.status),
                subsystem_type_id: Ok(value.subsystem_type_id),
                supplier_fault_number: Ok(value.supplier_fault_number),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteHistoryEntry {
        event_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        log_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        log_entry: ::std::result::Result<::std::string::String, ::std::string::String>,
        log_id: ::std::result::Result<u64, ::std::string::String>,
        log_user: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteHistoryEntry {
        fn default() -> Self {
            Self {
                event_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                log_date: Err("no value supplied for log_date".to_string()),
                log_entry: Err("no value supplied for log_entry".to_string()),
                log_id: Err("no value supplied for log_id".to_string()),
                log_user: Ok(Default::default()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteHistoryEntry {
        pub fn event_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.event_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_type: {e}"));
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
        pub fn log_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.log_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_date: {e}"));
            self
        }
        pub fn log_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.log_entry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_entry: {e}"));
            self
        }
        pub fn log_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.log_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_id: {e}"));
            self
        }
        pub fn log_user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.log_user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for log_user: {e}"));
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteHistoryEntry> for super::TransportRouteHistoryEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteHistoryEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
                extensions: value.extensions?,
                log_date: value.log_date?,
                log_entry: value.log_entry?,
                log_id: value.log_id?,
                log_user: value.log_user?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteHistoryEntry> for TransportRouteHistoryEntry {
        fn from(value: super::TransportRouteHistoryEntry) -> Self {
            Self {
                event_type: Ok(value.event_type),
                extensions: Ok(value.extensions),
                log_date: Ok(value.log_date),
                log_entry: Ok(value.log_entry),
                log_id: Ok(value.log_id),
                log_user: Ok(value.log_user),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteSegmentConfiguration {
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        route_segment_id: ::std::result::Result<u64, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        transport_route_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteSegmentConfiguration {
        fn default() -> Self {
            Self {
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                extensions: Ok(Default::default()),
                route_segment_id: Err("no value supplied for route_segment_id".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
                transport_route_id: Err("no value supplied for transport_route_id".to_string()),
            }
        }
    }
    impl TransportRouteSegmentConfiguration {
        pub fn configuration_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.configuration_date = value.try_into().map_err(|e| {
                format!("error converting supplied value for configuration_date: {e}")
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
        pub fn route_segment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.route_segment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for route_segment_id: {e}"));
            self
        }
        pub fn transport_link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport_link_id: {e}"));
            self
        }
        pub fn transport_route_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_route_id = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_route_id: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteSegmentConfiguration>
        for super::TransportRouteSegmentConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteSegmentConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration_date: value.configuration_date?,
                extensions: value.extensions?,
                route_segment_id: value.route_segment_id?,
                transport_link_id: value.transport_link_id?,
                transport_route_id: value.transport_route_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteSegmentConfiguration>
        for TransportRouteSegmentConfiguration
    {
        fn from(value: super::TransportRouteSegmentConfiguration) -> Self {
            Self {
                configuration_date: Ok(value.configuration_date),
                extensions: Ok(value.extensions),
                route_segment_id: Ok(value.route_segment_id),
                transport_link_id: Ok(value.transport_link_id),
                transport_route_id: Ok(value.transport_route_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportRouteType {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        specification_link: ::std::result::Result<
            ::std::option::Option<super::TransportRouteTypeSpecificationLink>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportRouteType {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                extensions: Ok(Default::default()),
                notes: Ok(Default::default()),
                specification_link: Ok(Default::default()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl TransportRouteType {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
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
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notes: {e}"));
            self
        }
        pub fn specification_link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportRouteTypeSpecificationLink>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.specification_link = value.try_into().map_err(|e| {
                format!("error converting supplied value for specification_link: {e}")
            });
            self
        }
        pub fn type_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeId>,
            T::Error: ::std::fmt::Display,
        {
            self.type_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TransportRouteType> for super::TransportRouteType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportRouteType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                notes: value.notes?,
                specification_link: value.specification_link?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportRouteType> for TransportRouteType {
        fn from(value: super::TransportRouteType) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                notes: Ok(value.notes),
                specification_link: Ok(value.specification_link),
                type_id: Ok(value.type_id),
            }
        }
    }
}
