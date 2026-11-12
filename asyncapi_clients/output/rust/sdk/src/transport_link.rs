/*
 * File:        output/rust/sdk/src/transport_link.rs
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
#[doc = "    \"LinkStatusType\": {"]
#[doc = "      \"$ref\": \"#/$defs/LinkStatusType\""]
#[doc = "    },"]
#[doc = "    \"Point\": {"]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"RecalibrateTransportLinkCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/RecalibrateTransportLinkCommand\""]
#[doc = "    },"]
#[doc = "    \"RequestTransportLinkStatusCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/RequestTransportLinkStatusCommand\""]
#[doc = "    },"]
#[doc = "    \"SetStocCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/SetStocCommand\""]
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
#[doc = "    \"TransportLinkANPRConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkANPRConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkANPRDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkANPRDynamic\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkCommand\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkDefinition\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkFault\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkFault\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkGPSConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkGPSConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkGPSDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkGPSDynamic\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkHistoryEntry\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkHistoryEntry\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkMeasurement\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkMeasurement\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkSCOOTConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkSCOOTConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkSCOOTDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkSCOOTDynamic\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkType\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkType\""]
#[doc = "    },"]
#[doc = "    \"TransportLinkTypeUnion\": {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkTypeUnion\""]
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
        rename = "LinkStatusType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_status_type: ::std::option::Option<LinkStatusType>,
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
        rename = "RecalibrateTransportLinkCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub recalibrate_transport_link_command: ::std::option::Option<RecalibrateTransportLinkCommand>,
    #[serde(
        rename = "RequestTransportLinkStatusCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_transport_link_status_command:
        ::std::option::Option<RequestTransportLinkStatusCommand>,
    #[serde(
        rename = "SetStocCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub set_stoc_command: ::std::option::Option<SetStocCommand>,
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
        rename = "TransportLinkANPRConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_anpr_configuration: ::std::option::Option<TransportLinkAnprConfiguration>,
    #[serde(
        rename = "TransportLinkANPRDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_anpr_dynamic: ::std::option::Option<TransportLinkAnprDynamic>,
    #[serde(
        rename = "TransportLinkCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_command: ::std::option::Option<TransportLinkCommand>,
    #[serde(
        rename = "TransportLinkCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_command_response: ::std::option::Option<TransportLinkCommandResponse>,
    #[serde(
        rename = "TransportLinkConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_configuration: ::std::option::Option<TransportLinkConfiguration>,
    #[serde(
        rename = "TransportLinkDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_definition: ::std::option::Option<TransportLinkDefinition>,
    #[serde(
        rename = "TransportLinkFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_fault: ::std::option::Option<TransportLinkFault>,
    #[serde(
        rename = "TransportLinkGPSConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_gps_configuration: ::std::option::Option<TransportLinkGpsConfiguration>,
    #[serde(
        rename = "TransportLinkGPSDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_gps_dynamic: ::std::option::Option<TransportLinkGpsDynamic>,
    #[serde(
        rename = "TransportLinkHistoryEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_history_entry: ::std::option::Option<TransportLinkHistoryEntry>,
    #[serde(
        rename = "TransportLinkMeasurement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_measurement: ::std::option::Option<TransportLinkMeasurement>,
    #[serde(
        rename = "TransportLinkSCOOTConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_scoot_configuration: ::std::option::Option<TransportLinkScootConfiguration>,
    #[serde(
        rename = "TransportLinkSCOOTDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_scoot_dynamic: ::std::option::Option<TransportLinkScootDynamic>,
    #[serde(
        rename = "TransportLinkType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_type: ::std::option::Option<TransportLinkType>,
    #[serde(
        rename = "TransportLinkTypeUnion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_type_union: ::std::option::Option<TransportLinkTypeUnion>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            acknowledgement_state: Default::default(),
            command_response_status: Default::default(),
            coordinates: Default::default(),
            fault_status: Default::default(),
            link_status_type: Default::default(),
            point: Default::default(),
            quality: Default::default(),
            recalibrate_transport_link_command: Default::default(),
            request_transport_link_status_command: Default::default(),
            set_stoc_command: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
            transport_link_anpr_configuration: Default::default(),
            transport_link_anpr_dynamic: Default::default(),
            transport_link_command: Default::default(),
            transport_link_command_response: Default::default(),
            transport_link_configuration: Default::default(),
            transport_link_definition: Default::default(),
            transport_link_fault: Default::default(),
            transport_link_gps_configuration: Default::default(),
            transport_link_gps_dynamic: Default::default(),
            transport_link_history_entry: Default::default(),
            transport_link_measurement: Default::default(),
            transport_link_scoot_configuration: Default::default(),
            transport_link_scoot_dynamic: Default::default(),
            transport_link_type: Default::default(),
            transport_link_type_union: Default::default(),
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
#[doc = "Catalogue entry for one operational status of a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Catalogue entry for one operational status of a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"kind\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description of the link status (v1: 'normal', 'suspect', 'noVehiclesInPeriod').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"linkStatus\\\" selects the LinkStatusType payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"linkStatus\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"description\": \"Additional notes about this status value.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this link status type.\","]
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
pub struct LinkStatusType {
    #[doc = "Human-readable description of the link status (v1: 'normal', 'suspect', 'noVehiclesInPeriod')."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"linkStatus\" selects the LinkStatusType payload; it is not free-text command content."]
    pub kind: LinkStatusTypeKind,
    #[doc = "Additional notes about this status value."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of this link status type."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl LinkStatusType {
    pub fn builder() -> builder::LinkStatusType {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"linkStatus\" selects the LinkStatusType payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"linkStatus\\\" selects the LinkStatusType payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"linkStatus\""]
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
pub enum LinkStatusTypeKind {
    #[serde(rename = "linkStatus")]
    LinkStatus,
}
impl ::std::fmt::Display for LinkStatusTypeKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LinkStatus => f.write_str("linkStatus"),
        }
    }
}
impl ::std::str::FromStr for LinkStatusTypeKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "linkStatus" => Ok(Self::LinkStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LinkStatusTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LinkStatusTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LinkStatusTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the transport link receiving the command."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Identifier of the transport link receiving the command.\","]
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
#[doc = "Trigger a recalibration on a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Trigger a recalibration on a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"transportLinkId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"recalibrate\\\" selects the RecalibrateTransportLinkCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"recalibrate\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link receiving the command.\","]
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
pub struct RecalibrateTransportLinkCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"recalibrate\" selects the RecalibrateTransportLinkCommand payload; it is not free-text command content."]
    pub kind: RecalibrateTransportLinkCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the transport link receiving the command."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl RecalibrateTransportLinkCommand {
    pub fn builder() -> builder::RecalibrateTransportLinkCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"recalibrate\" selects the RecalibrateTransportLinkCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"recalibrate\\\" selects the RecalibrateTransportLinkCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"recalibrate\""]
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
pub enum RecalibrateTransportLinkCommandKind {
    #[serde(rename = "recalibrate")]
    Recalibrate,
}
impl ::std::fmt::Display for RecalibrateTransportLinkCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Recalibrate => f.write_str("recalibrate"),
        }
    }
}
impl ::std::str::FromStr for RecalibrateTransportLinkCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "recalibrate" => Ok(Self::Recalibrate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RecalibrateTransportLinkCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RecalibrateTransportLinkCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RecalibrateTransportLinkCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Request the current status of a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request the current status of a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"transportLinkId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestTransportLinkStatusCommand payload; it is not free-text command content.\","]
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
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link receiving the command.\","]
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
pub struct RequestTransportLinkStatusCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestTransportLinkStatusCommand payload; it is not free-text command content."]
    pub kind: RequestTransportLinkStatusCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the transport link receiving the command."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl RequestTransportLinkStatusCommand {
    pub fn builder() -> builder::RequestTransportLinkStatusCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestTransportLinkStatusCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestTransportLinkStatusCommand payload; it is not free-text command content.\","]
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
pub enum RequestTransportLinkStatusCommandKind {
    #[serde(rename = "requestStatus")]
    RequestStatus,
}
impl ::std::fmt::Display for RequestTransportLinkStatusCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RequestStatus => f.write_str("requestStatus"),
        }
    }
}
impl ::std::str::FromStr for RequestTransportLinkStatusCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requestStatus" => Ok(Self::RequestStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequestTransportLinkStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequestTransportLinkStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestTransportLinkStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Set the SCOOT STOC threshold value on a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Set the SCOOT STOC threshold value on a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"stocValue\","]
#[doc = "    \"transportLinkId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"setStoc\\\" selects the SetStocCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"setStoc\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"stocValue\": {"]
#[doc = "      \"description\": \"STOC value to set.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link receiving the command.\","]
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
pub struct SetStocCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"setStoc\" selects the SetStocCommand payload; it is not free-text command content."]
    pub kind: SetStocCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "STOC value to set."]
    #[serde(rename = "stocValue")]
    pub stoc_value: ::std::string::String,
    #[doc = "Identifier of the transport link receiving the command."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl SetStocCommand {
    pub fn builder() -> builder::SetStocCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"setStoc\" selects the SetStocCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"setStoc\\\" selects the SetStocCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"setStoc\""]
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
pub enum SetStocCommandKind {
    #[serde(rename = "setStoc")]
    SetStoc,
}
impl ::std::fmt::Display for SetStocCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetStoc => f.write_str("setStoc"),
        }
    }
}
impl ::std::str::FromStr for SetStocCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "setStoc" => Ok(Self::SetStoc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetStocCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetStocCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetStocCommandKind {
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
#[doc = "Terminal message ending a transport link sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport link Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a transport link sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of transport link objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport link Sync Request\","]
#[doc = "  \"description\": \"Request the current state of transport link objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Operational configuration for an ANPR-monitored transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Operational configuration for an ANPR-monitored transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"kind\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capacityVehiclesPerHour\": {"]
#[doc = "      \"description\": \"Maximum flow in vehicles per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration record became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"fullLengthMetres\": {"]
#[doc = "      \"description\": \"Distance from stop line to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"anpr\\\" selects the TransportLinkANPRConfiguration payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"anpr\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lengthMetres\": {"]
#[doc = "      \"description\": \"Distance from detector to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"maximumSpeedKph\": {"]
#[doc = "      \"description\": \"Maximum speed associated with the link in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberOfLanes\": {"]
#[doc = "      \"description\": \"Number of lanes on the link.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this configuration applies to.\","]
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
pub struct TransportLinkAnprConfiguration {
    #[doc = "Maximum flow in vehicles per hour."]
    #[serde(
        rename = "capacityVehiclesPerHour",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_vehicles_per_hour: ::std::option::Option<f64>,
    #[doc = "When this configuration record became effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance from stop line to stop line in metres."]
    #[serde(
        rename = "fullLengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_length_metres: ::std::option::Option<f64>,
    #[doc = "Discriminator for the tagged union. The fixed value \"anpr\" selects the TransportLinkANPRConfiguration payload; it is not free-text command content."]
    pub kind: TransportLinkAnprConfigurationKind,
    #[doc = "Distance from detector to stop line in metres."]
    #[serde(
        rename = "lengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub length_metres: ::std::option::Option<f64>,
    #[doc = "Maximum speed associated with the link in kilometres per hour."]
    #[serde(
        rename = "maximumSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maximum_speed_kph: ::std::option::Option<f64>,
    #[doc = "Number of lanes on the link."]
    #[serde(
        rename = "numberOfLanes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_of_lanes: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Identifier of the transport link this configuration applies to."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkAnprConfiguration {
    pub fn builder() -> builder::TransportLinkAnprConfiguration {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"anpr\" selects the TransportLinkANPRConfiguration payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"anpr\\\" selects the TransportLinkANPRConfiguration payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"anpr\""]
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
pub enum TransportLinkAnprConfigurationKind {
    #[serde(rename = "anpr")]
    Anpr,
}
impl ::std::fmt::Display for TransportLinkAnprConfigurationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Anpr => f.write_str("anpr"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkAnprConfigurationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "anpr" => Ok(Self::Anpr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkAnprConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkAnprConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkAnprConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ANPR-reported live measurement for a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ANPR-reported live measurement for a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"averageSpeedKph\": {"]
#[doc = "      \"description\": \"Estimated average speed in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"anpr\\\" selects the TransportLinkANPRDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"anpr\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"linkTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Current link journey time in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberPlatesIn\": {"]
#[doc = "      \"description\": \"Number of successful plate reads at the start of the link.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberPlatesMatches\": {"]
#[doc = "      \"description\": \"Count of number-plate matches in the measurement period.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberPlatesMatchesUsed\": {"]
#[doc = "      \"description\": \"Number of successful matches actually used in the travel-time calculation.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberPlatesOut\": {"]
#[doc = "      \"description\": \"Number of successful plate reads at the end of the link.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this reading is about.\","]
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
pub struct TransportLinkAnprDynamic {
    #[doc = "Estimated average speed in kilometres per hour."]
    #[serde(
        rename = "averageSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub average_speed_kph: ::std::option::Option<f64>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"anpr\" selects the TransportLinkANPRDynamic payload; it is not free-text command content."]
    pub kind: TransportLinkAnprDynamicKind,
    #[doc = "Current link journey time in seconds."]
    #[serde(
        rename = "linkTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_travel_time_seconds: ::std::option::Option<f64>,
    #[doc = "Number of successful plate reads at the start of the link."]
    #[serde(
        rename = "numberPlatesIn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_plates_in: ::std::option::Option<u64>,
    #[doc = "Count of number-plate matches in the measurement period."]
    #[serde(
        rename = "numberPlatesMatches",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_plates_matches: ::std::option::Option<u64>,
    #[doc = "Number of successful matches actually used in the travel-time calculation."]
    #[serde(
        rename = "numberPlatesMatchesUsed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_plates_matches_used: ::std::option::Option<u64>,
    #[doc = "Number of successful plate reads at the end of the link."]
    #[serde(
        rename = "numberPlatesOut",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_plates_out: ::std::option::Option<u64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the transport link this reading is about."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkAnprDynamic {
    pub fn builder() -> builder::TransportLinkAnprDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"anpr\" selects the TransportLinkANPRDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"anpr\\\" selects the TransportLinkANPRDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"anpr\""]
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
pub enum TransportLinkAnprDynamicKind {
    #[serde(rename = "anpr")]
    Anpr,
}
impl ::std::fmt::Display for TransportLinkAnprDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Anpr => f.write_str("anpr"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkAnprDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "anpr" => Ok(Self::Anpr),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkAnprDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkAnprDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkAnprDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Transport link command payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Command\","]
#[doc = "  \"description\": \"Transport link command payloads.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SetStocCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RecalibrateTransportLinkCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RequestTransportLinkStatusCommand\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"recalibrate\": \"#/$defs/RecalibrateTransportLinkCommand\","]
#[doc = "      \"requestStatus\": \"#/$defs/RequestTransportLinkStatusCommand\","]
#[doc = "      \"setStoc\": \"#/$defs/SetStocCommand\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransportLinkCommand {
    SetStocCommand(SetStocCommand),
    RecalibrateTransportLinkCommand(RecalibrateTransportLinkCommand),
    RequestTransportLinkStatusCommand(RequestTransportLinkStatusCommand),
}
impl ::std::convert::From<SetStocCommand> for TransportLinkCommand {
    fn from(value: SetStocCommand) -> Self {
        Self::SetStocCommand(value)
    }
}
impl ::std::convert::From<RecalibrateTransportLinkCommand> for TransportLinkCommand {
    fn from(value: RecalibrateTransportLinkCommand) -> Self {
        Self::RecalibrateTransportLinkCommand(value)
    }
}
impl ::std::convert::From<RequestTransportLinkStatusCommand> for TransportLinkCommand {
    fn from(value: RequestTransportLinkStatusCommand) -> Self {
        Self::RequestTransportLinkStatusCommand(value)
    }
}
#[doc = "Transport link command response payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Command Response\","]
#[doc = "  \"description\": \"Transport link command response payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\","]
#[doc = "    \"transportLinkId\""]
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
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable response message with details.\","]
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
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link the command targeted.\","]
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
pub struct TransportLinkCommandResponse {
    #[doc = "Machine-readable response code for programmatic handling."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Human-readable response message with details."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[doc = "When the response was generated."]
    #[serde(rename = "responseTime")]
    pub response_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Outcome of the command execution."]
    pub status: CommandResponseStatus,
    #[doc = "Identifier of the transport link the command targeted."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkCommandResponse {
    pub fn builder() -> builder::TransportLinkCommandResponse {
        Default::default()
    }
}
#[doc = "Transport link operational configuration (SCOOT/GPS/ANPR discriminated union)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Configuration\","]
#[doc = "  \"description\": \"Transport link operational configuration (SCOOT/GPS/ANPR discriminated union).\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkSCOOTConfiguration\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkGPSConfiguration\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkANPRConfiguration\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"anpr\": \"#/$defs/TransportLinkANPRConfiguration\","]
#[doc = "      \"gps\": \"#/$defs/TransportLinkGPSConfiguration\","]
#[doc = "      \"scoot\": \"#/$defs/TransportLinkSCOOTConfiguration\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransportLinkConfiguration {
    ScootConfiguration(TransportLinkScootConfiguration),
    GpsConfiguration(TransportLinkGpsConfiguration),
    AnprConfiguration(TransportLinkAnprConfiguration),
}
impl ::std::convert::From<TransportLinkScootConfiguration> for TransportLinkConfiguration {
    fn from(value: TransportLinkScootConfiguration) -> Self {
        Self::ScootConfiguration(value)
    }
}
impl ::std::convert::From<TransportLinkGpsConfiguration> for TransportLinkConfiguration {
    fn from(value: TransportLinkGpsConfiguration) -> Self {
        Self::GpsConfiguration(value)
    }
}
impl ::std::convert::From<TransportLinkAnprConfiguration> for TransportLinkConfiguration {
    fn from(value: TransportLinkAnprConfiguration) -> Self {
        Self::AnprConfiguration(value)
    }
}
#[doc = "Transport link equipment details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Definition\","]
#[doc = "  \"description\": \"Transport link equipment details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"transportLinkId\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When this definition record was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"endPoint\": {"]
#[doc = "      \"description\": \"End of the transport link as a WGS84 point. Optional here although the v1 XSD requires it, matching transport_route.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Start of the transport link as a WGS84 point. Optional.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"longDescription\": {"]
#[doc = "      \"description\": \"Longer description (context, commissioning notes, etc.).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 2000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"originatingObjectId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of this object on the originating (upstream) system. Enables cross-system traceability.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"originatingParticipantId\": {"]
#[doc = "      \"title\": \"ParticipantId\","]
#[doc = "      \"description\": \"Identifier of the upstream system that originally created this object (comparable to the UML DataSourceTypeId).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"participantId\": {"]
#[doc = "      \"title\": \"ParticipantId\","]
#[doc = "      \"description\": \"System that owns this transport link definition. Often implicit from context.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"description\": \"Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this transport link within its owning system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...).\","]
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
pub struct TransportLinkDefinition {
    #[doc = "When this definition record was created."]
    #[serde(rename = "creationDate")]
    pub creation_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "End of the transport link as a WGS84 point. Optional here although the v1 XSD requires it, matching transport_route."]
    #[serde(
        rename = "endPoint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub end_point: ::std::option::Option<Point>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Start of the transport link as a WGS84 point. Optional."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Longer description (context, commissioning notes, etc.)."]
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<TransportLinkDefinitionLongDescription>,
    #[doc = "Identifier of this object on the originating (upstream) system. Enables cross-system traceability."]
    #[serde(
        rename = "originatingObjectId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub originating_object_id: ::std::option::Option<ObjectId>,
    #[doc = "Identifier of the upstream system that originally created this object (comparable to the UML DataSourceTypeId)."]
    #[serde(
        rename = "originatingParticipantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub originating_participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "System that owns this transport link definition. Often implicit from context."]
    #[serde(
        rename = "participantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6')."]
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<TransportLinkDefinitionShortDescription>,
    #[doc = "Unique identifier of this transport link within its owning system."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
    #[doc = "Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...)."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl TransportLinkDefinition {
    pub fn builder() -> builder::TransportLinkDefinition {
        Default::default()
    }
}
#[doc = "Longer description (context, commissioning notes, etc.)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Longer description (context, commissioning notes, etc.).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 2000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportLinkDefinitionLongDescription(::std::string::String);
impl ::std::ops::Deref for TransportLinkDefinitionLongDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportLinkDefinitionLongDescription> for ::std::string::String {
    fn from(value: TransportLinkDefinitionLongDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportLinkDefinitionLongDescription {
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
impl ::std::convert::TryFrom<&str> for TransportLinkDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportLinkDefinitionLongDescription {
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
#[doc = "Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportLinkDefinitionShortDescription(::std::string::String);
impl ::std::ops::Deref for TransportLinkDefinitionShortDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportLinkDefinitionShortDescription> for ::std::string::String {
    fn from(value: TransportLinkDefinitionShortDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportLinkDefinitionShortDescription {
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
impl ::std::convert::TryFrom<&str> for TransportLinkDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportLinkDefinitionShortDescription {
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
#[doc = "Transport link fault reports."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Fault\","]
#[doc = "  \"description\": \"Transport link fault reports.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"faultId\","]
#[doc = "    \"faultTypeId\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"acknowledgementDate\": {"]
#[doc = "      \"description\": \"When the fault was acknowledged.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"acknowledgementState\": {"]
#[doc = "      \"description\": \"Current acknowledgement state.\","]
#[doc = "      \"$ref\": \"#/$defs/AcknowledgementState\""]
#[doc = "    },"]
#[doc = "    \"clearedDate\": {"]
#[doc = "      \"description\": \"When the fault was cleared. If present, the fault is historical.\","]
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
#[doc = "      \"description\": \"Free-text description of the fault.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"equipmentFault\": {"]
#[doc = "      \"description\": \"Whether this is an equipment fault. The fault type's catalogue category classifies the type; this flag classifies this fault.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"faultId\": {"]
#[doc = "      \"description\": \"Unique identifier of this fault report within the owning system.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"faultTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Reference to a FaultTypeDefinition entry in the shared fault catalogue.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"lastUpdated\": {"]
#[doc = "      \"description\": \"When the fault record was last modified.\","]
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
#[doc = "      \"description\": \"Fault severity (higher = more severe).\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Whether this fault is still active or has been cleared.\","]
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
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this fault is about.\","]
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
pub struct TransportLinkFault {
    #[doc = "When the fault was acknowledged."]
    #[serde(
        rename = "acknowledgementDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Current acknowledgement state."]
    #[serde(
        rename = "acknowledgementState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_state: ::std::option::Option<AcknowledgementState>,
    #[doc = "When the fault was cleared. If present, the fault is historical."]
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
    #[doc = "Free-text description of the fault."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Whether this is an equipment fault. The fault type's catalogue category classifies the type; this flag classifies this fault."]
    #[serde(
        rename = "equipmentFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub equipment_fault: ::std::option::Option<bool>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Unique identifier of this fault report within the owning system."]
    #[serde(rename = "faultId")]
    pub fault_id: u64,
    #[doc = "Reference to a FaultTypeDefinition entry in the shared fault catalogue."]
    #[serde(rename = "faultTypeId")]
    pub fault_type_id: TypeId,
    #[doc = "When the fault record was last modified."]
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_updated: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub parameters: ::std::vec::Vec<::std::string::String>,
    #[doc = "Fault severity (higher = more severe)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<u64>,
    #[doc = "Whether this fault is still active or has been cleared."]
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
    #[doc = "Identifier of the transport link this fault is about."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkFault {
    pub fn builder() -> builder::TransportLinkFault {
        Default::default()
    }
}
#[doc = "Operational configuration for a GPS-monitored transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Operational configuration for a GPS-monitored transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"kind\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capacityVehiclesPerHour\": {"]
#[doc = "      \"description\": \"Maximum flow in vehicles per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration record became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"fullLengthMetres\": {"]
#[doc = "      \"description\": \"Distance from stop line to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"gps\\\" selects the TransportLinkGPSConfiguration payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"gps\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lengthMetres\": {"]
#[doc = "      \"description\": \"Distance from detector to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"maximumSpeedKph\": {"]
#[doc = "      \"description\": \"Maximum speed associated with the link in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberOfLanes\": {"]
#[doc = "      \"description\": \"Number of lanes on the link.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this configuration applies to.\","]
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
pub struct TransportLinkGpsConfiguration {
    #[doc = "Maximum flow in vehicles per hour."]
    #[serde(
        rename = "capacityVehiclesPerHour",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_vehicles_per_hour: ::std::option::Option<f64>,
    #[doc = "When this configuration record became effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance from stop line to stop line in metres."]
    #[serde(
        rename = "fullLengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_length_metres: ::std::option::Option<f64>,
    #[doc = "Discriminator for the tagged union. The fixed value \"gps\" selects the TransportLinkGPSConfiguration payload; it is not free-text command content."]
    pub kind: TransportLinkGpsConfigurationKind,
    #[doc = "Distance from detector to stop line in metres."]
    #[serde(
        rename = "lengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub length_metres: ::std::option::Option<f64>,
    #[doc = "Maximum speed associated with the link in kilometres per hour."]
    #[serde(
        rename = "maximumSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maximum_speed_kph: ::std::option::Option<f64>,
    #[doc = "Number of lanes on the link."]
    #[serde(
        rename = "numberOfLanes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_of_lanes: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Identifier of the transport link this configuration applies to."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkGpsConfiguration {
    pub fn builder() -> builder::TransportLinkGpsConfiguration {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"gps\" selects the TransportLinkGPSConfiguration payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"gps\\\" selects the TransportLinkGPSConfiguration payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"gps\""]
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
pub enum TransportLinkGpsConfigurationKind {
    #[serde(rename = "gps")]
    Gps,
}
impl ::std::fmt::Display for TransportLinkGpsConfigurationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gps => f.write_str("gps"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkGpsConfigurationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gps" => Ok(Self::Gps),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkGpsConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkGpsConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkGpsConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "GPS-probe live measurement for a transport link.\n\nField set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow\nExtension): CurrentJourneyTime, the three travel-time statistics\n(Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-\nlane flow count. Shared base fields (transport_link_id, timestamp,\nquality, average_speed_kph, link_travel_time_seconds) are inherited\nfrom _TransportLinkDynamicBase."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"GPS-probe live measurement for a transport link.\\n\\nField set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow\\nExtension): CurrentJourneyTime, the three travel-time statistics\\n(Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-\\nlane flow count. Shared base fields (transport_link_id, timestamp,\\nquality, average_speed_kph, link_travel_time_seconds) are inherited\\nfrom _TransportLinkDynamicBase.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"averageSpeedKph\": {"]
#[doc = "      \"description\": \"Estimated average speed in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"currentJourneyTimeSeconds\": {"]
#[doc = "      \"description\": \"Current journey time in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"gps\\\" selects the TransportLinkGPSDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"gps\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"linkTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Current link journey time in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"meanTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Mean travel time along the link in the measured interval, in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"medianTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Median travel time along the link in the measured interval, in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberOfVehicles\": {"]
#[doc = "      \"description\": \"Number of vehicles in the measured interval.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"reservedLaneFlowVehicles\": {"]
#[doc = "      \"description\": \"Flow in the reserved lane in the measured interval, in vehicles. A rate, so it may be fractional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"varianceTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Variance of travel times along the link in the measured interval. The unit is seconds squared (s²), as for any variance of times; the field name keeps 'Seconds' for compatibility with v1.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TransportLinkGpsDynamic {
    #[doc = "Estimated average speed in kilometres per hour."]
    #[serde(
        rename = "averageSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub average_speed_kph: ::std::option::Option<f64>,
    #[doc = "Current journey time in seconds."]
    #[serde(
        rename = "currentJourneyTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_journey_time_seconds: ::std::option::Option<f64>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"gps\" selects the TransportLinkGPSDynamic payload; it is not free-text command content."]
    pub kind: TransportLinkGpsDynamicKind,
    #[doc = "Current link journey time in seconds."]
    #[serde(
        rename = "linkTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_travel_time_seconds: ::std::option::Option<f64>,
    #[doc = "Mean travel time along the link in the measured interval, in seconds."]
    #[serde(
        rename = "meanTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mean_travel_time_seconds: ::std::option::Option<f64>,
    #[doc = "Median travel time along the link in the measured interval, in seconds."]
    #[serde(
        rename = "medianTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub median_travel_time_seconds: ::std::option::Option<f64>,
    #[doc = "Number of vehicles in the measured interval."]
    #[serde(
        rename = "numberOfVehicles",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_of_vehicles: ::std::option::Option<u64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Flow in the reserved lane in the measured interval, in vehicles. A rate, so it may be fractional."]
    #[serde(
        rename = "reservedLaneFlowVehicles",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reserved_lane_flow_vehicles: ::std::option::Option<f64>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the transport link this reading is about."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
    #[doc = "Variance of travel times along the link in the measured interval. The unit is seconds squared (s²), as for any variance of times; the field name keeps 'Seconds' for compatibility with v1."]
    #[serde(
        rename = "varianceTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub variance_travel_time_seconds: ::std::option::Option<f64>,
}
impl TransportLinkGpsDynamic {
    pub fn builder() -> builder::TransportLinkGpsDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"gps\" selects the TransportLinkGPSDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"gps\\\" selects the TransportLinkGPSDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"gps\""]
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
pub enum TransportLinkGpsDynamicKind {
    #[serde(rename = "gps")]
    Gps,
}
impl ::std::fmt::Display for TransportLinkGpsDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gps => f.write_str("gps"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkGpsDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gps" => Ok(Self::Gps),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkGpsDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkGpsDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkGpsDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Transport link history entries."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link History\","]
#[doc = "  \"description\": \"Transport link history entries.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"logDate\","]
#[doc = "    \"logEntry\","]
#[doc = "    \"logId\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"eventType\": {"]
#[doc = "      \"description\": \"Category or type of the history event (e.g. 'reconfiguration', 'capacity_update').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"logDate\": {"]
#[doc = "      \"description\": \"When the log entry was recorded.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"logEntry\": {"]
#[doc = "      \"description\": \"Equipment log data (free-form text).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"logId\": {"]
#[doc = "      \"description\": \"Unique identifier of this log entry within the owning system.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"logUser\": {"]
#[doc = "      \"description\": \"User who created the log entry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this log entry is about.\","]
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
pub struct TransportLinkHistoryEntry {
    #[doc = "Category or type of the history event (e.g. 'reconfiguration', 'capacity_update')."]
    #[serde(
        rename = "eventType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub event_type: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "When the log entry was recorded."]
    #[serde(rename = "logDate")]
    pub log_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Equipment log data (free-form text)."]
    #[serde(rename = "logEntry")]
    pub log_entry: ::std::string::String,
    #[doc = "Unique identifier of this log entry within the owning system."]
    #[serde(rename = "logId")]
    pub log_id: u64,
    #[doc = "User who created the log entry."]
    #[serde(
        rename = "logUser",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub log_user: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the transport link this log entry is about."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkHistoryEntry {
    pub fn builder() -> builder::TransportLinkHistoryEntry {
        Default::default()
    }
}
#[doc = "Transport link dynamic measurement data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Measurement\","]
#[doc = "  \"description\": \"Transport link dynamic measurement data.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkSCOOTDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkANPRDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkGPSDynamic\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"anpr\": \"#/$defs/TransportLinkANPRDynamic\","]
#[doc = "      \"gps\": \"#/$defs/TransportLinkGPSDynamic\","]
#[doc = "      \"scoot\": \"#/$defs/TransportLinkSCOOTDynamic\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransportLinkMeasurement {
    ScootDynamic(TransportLinkScootDynamic),
    AnprDynamic(TransportLinkAnprDynamic),
    GpsDynamic(TransportLinkGpsDynamic),
}
impl ::std::convert::From<TransportLinkScootDynamic> for TransportLinkMeasurement {
    fn from(value: TransportLinkScootDynamic) -> Self {
        Self::ScootDynamic(value)
    }
}
impl ::std::convert::From<TransportLinkAnprDynamic> for TransportLinkMeasurement {
    fn from(value: TransportLinkAnprDynamic) -> Self {
        Self::AnprDynamic(value)
    }
}
impl ::std::convert::From<TransportLinkGpsDynamic> for TransportLinkMeasurement {
    fn from(value: TransportLinkGpsDynamic) -> Self {
        Self::GpsDynamic(value)
    }
}
#[doc = "Operational configuration for a SCOOT-modelled transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Operational configuration for a SCOOT-modelled transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"kind\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capacityVehiclesPerHour\": {"]
#[doc = "      \"description\": \"Maximum flow in vehicles per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration record became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"fullLengthMetres\": {"]
#[doc = "      \"description\": \"Distance from stop line to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"scoot\\\" selects the TransportLinkSCOOTConfiguration payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"scoot\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lengthMetres\": {"]
#[doc = "      \"description\": \"Distance from detector to stop line in metres.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"maximumSpeedKph\": {"]
#[doc = "      \"description\": \"Maximum speed associated with the link in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"numberOfLanes\": {"]
#[doc = "      \"description\": \"Number of lanes on the link.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this configuration applies to.\","]
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
pub struct TransportLinkScootConfiguration {
    #[doc = "Maximum flow in vehicles per hour."]
    #[serde(
        rename = "capacityVehiclesPerHour",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_vehicles_per_hour: ::std::option::Option<f64>,
    #[doc = "When this configuration record became effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance from stop line to stop line in metres."]
    #[serde(
        rename = "fullLengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub full_length_metres: ::std::option::Option<f64>,
    #[doc = "Discriminator for the tagged union. The fixed value \"scoot\" selects the TransportLinkSCOOTConfiguration payload; it is not free-text command content."]
    pub kind: TransportLinkScootConfigurationKind,
    #[doc = "Distance from detector to stop line in metres."]
    #[serde(
        rename = "lengthMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub length_metres: ::std::option::Option<f64>,
    #[doc = "Maximum speed associated with the link in kilometres per hour."]
    #[serde(
        rename = "maximumSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maximum_speed_kph: ::std::option::Option<f64>,
    #[doc = "Number of lanes on the link."]
    #[serde(
        rename = "numberOfLanes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_of_lanes: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Identifier of the transport link this configuration applies to."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkScootConfiguration {
    pub fn builder() -> builder::TransportLinkScootConfiguration {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"scoot\" selects the TransportLinkSCOOTConfiguration payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"scoot\\\" selects the TransportLinkSCOOTConfiguration payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"scoot\""]
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
pub enum TransportLinkScootConfigurationKind {
    #[serde(rename = "scoot")]
    Scoot,
}
impl ::std::fmt::Display for TransportLinkScootConfigurationKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Scoot => f.write_str("scoot"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkScootConfigurationKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "scoot" => Ok(Self::Scoot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkScootConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkScootConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkScootConfigurationKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "SCOOT-reported live measurement for a transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"SCOOT-reported live measurement for a transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"transportLinkId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"averageSpeedKph\": {"]
#[doc = "      \"description\": \"Estimated average speed in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"congestionPercent\": {"]
#[doc = "      \"description\": \"Current percentage congestion as reported by SCOOT.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"currentFlowVehicles\": {"]
#[doc = "      \"description\": \"Current flow in vehicles over the SCOOT measurement interval. A rate, so it may be fractional (e.g. a smoothed or averaged flow).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"scoot\\\" selects the TransportLinkSCOOTDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"scoot\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"linkTravelTimeSeconds\": {"]
#[doc = "      \"description\": \"Current link journey time in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"occupancyPercent\": {"]
#[doc = "      \"description\": \"Average link occupancy percentage across all detectors on the link.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the transport link this reading is about.\","]
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
pub struct TransportLinkScootDynamic {
    #[doc = "Estimated average speed in kilometres per hour."]
    #[serde(
        rename = "averageSpeedKph",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub average_speed_kph: ::std::option::Option<f64>,
    #[doc = "Current percentage congestion as reported by SCOOT."]
    #[serde(
        rename = "congestionPercent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub congestion_percent: ::std::option::Option<f64>,
    #[doc = "Current flow in vehicles over the SCOOT measurement interval. A rate, so it may be fractional (e.g. a smoothed or averaged flow)."]
    #[serde(
        rename = "currentFlowVehicles",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_flow_vehicles: ::std::option::Option<f64>,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"scoot\" selects the TransportLinkSCOOTDynamic payload; it is not free-text command content."]
    pub kind: TransportLinkScootDynamicKind,
    #[doc = "Current link journey time in seconds."]
    #[serde(
        rename = "linkTravelTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_travel_time_seconds: ::std::option::Option<f64>,
    #[doc = "Average link occupancy percentage across all detectors on the link."]
    #[serde(
        rename = "occupancyPercent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub occupancy_percent: ::std::option::Option<f64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the transport link this reading is about."]
    #[serde(rename = "transportLinkId")]
    pub transport_link_id: ObjectId,
}
impl TransportLinkScootDynamic {
    pub fn builder() -> builder::TransportLinkScootDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"scoot\" selects the TransportLinkSCOOTDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"scoot\\\" selects the TransportLinkSCOOTDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"scoot\""]
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
pub enum TransportLinkScootDynamicKind {
    #[serde(rename = "scoot")]
    Scoot,
}
impl ::std::fmt::Display for TransportLinkScootDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Scoot => f.write_str("scoot"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkScootDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "scoot" => Ok(Self::Scoot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkScootDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkScootDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkScootDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Catalogue entry for one kind of transport link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Catalogue entry for one kind of transport link.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"kind\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description of the transport link kind.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"transportLink\\\" selects the TransportLinkType payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"transportLink\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"description\": \"Additional notes, e.g. deployment details.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"specificationLink\": {"]
#[doc = "      \"description\": \"URL to an external specification document for this transport link type.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this transport link type.\","]
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
pub struct TransportLinkType {
    #[doc = "Human-readable description of the transport link kind."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"transportLink\" selects the TransportLinkType payload; it is not free-text command content."]
    pub kind: TransportLinkTypeKind,
    #[doc = "Additional notes, e.g. deployment details."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "URL to an external specification document for this transport link type."]
    #[serde(
        rename = "specificationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_link: ::std::option::Option<TransportLinkTypeSpecificationLink>,
    #[doc = "Identifier of this transport link type."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl TransportLinkType {
    pub fn builder() -> builder::TransportLinkType {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"transportLink\" selects the TransportLinkType payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"transportLink\\\" selects the TransportLinkType payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"transportLink\""]
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
pub enum TransportLinkTypeKind {
    #[serde(rename = "transportLink")]
    TransportLink,
}
impl ::std::fmt::Display for TransportLinkTypeKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TransportLink => f.write_str("transportLink"),
        }
    }
}
impl ::std::str::FromStr for TransportLinkTypeKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transportLink" => Ok(Self::TransportLink),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkTypeKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "URL to an external specification document for this transport link type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL to an external specification document for this transport link type.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TransportLinkTypeSpecificationLink(::std::string::String);
impl ::std::ops::Deref for TransportLinkTypeSpecificationLink {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TransportLinkTypeSpecificationLink> for ::std::string::String {
    fn from(value: TransportLinkTypeSpecificationLink) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TransportLinkTypeSpecificationLink {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TransportLinkTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportLinkTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportLinkTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TransportLinkTypeSpecificationLink {
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
#[doc = "Transport link type definitions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Transport Link Type\","]
#[doc = "  \"description\": \"Transport link type definitions.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/TransportLinkType\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/LinkStatusType\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"linkStatus\": \"#/$defs/LinkStatusType\","]
#[doc = "      \"transportLink\": \"#/$defs/TransportLinkType\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransportLinkTypeUnion {
    TransportLinkType(TransportLinkType),
    LinkStatusType(LinkStatusType),
}
impl ::std::convert::From<TransportLinkType> for TransportLinkTypeUnion {
    fn from(value: TransportLinkType) -> Self {
        Self::TransportLinkType(value)
    }
}
impl ::std::convert::From<LinkStatusType> for TransportLinkTypeUnion {
    fn from(value: LinkStatusType) -> Self {
        Self::LinkStatusType(value)
    }
}
#[doc = "Identifier of this link status type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeId\","]
#[doc = "  \"description\": \"Identifier of this link status type.\","]
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
        link_status_type: ::std::result::Result<
            ::std::option::Option<super::LinkStatusType>,
            ::std::string::String,
        >,
        point: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        recalibrate_transport_link_command: ::std::result::Result<
            ::std::option::Option<super::RecalibrateTransportLinkCommand>,
            ::std::string::String,
        >,
        request_transport_link_status_command: ::std::result::Result<
            ::std::option::Option<super::RequestTransportLinkStatusCommand>,
            ::std::string::String,
        >,
        set_stoc_command: ::std::result::Result<
            ::std::option::Option<super::SetStocCommand>,
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
        transport_link_anpr_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportLinkAnprConfiguration>,
            ::std::string::String,
        >,
        transport_link_anpr_dynamic: ::std::result::Result<
            ::std::option::Option<super::TransportLinkAnprDynamic>,
            ::std::string::String,
        >,
        transport_link_command: ::std::result::Result<
            ::std::option::Option<super::TransportLinkCommand>,
            ::std::string::String,
        >,
        transport_link_command_response: ::std::result::Result<
            ::std::option::Option<super::TransportLinkCommandResponse>,
            ::std::string::String,
        >,
        transport_link_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportLinkConfiguration>,
            ::std::string::String,
        >,
        transport_link_definition: ::std::result::Result<
            ::std::option::Option<super::TransportLinkDefinition>,
            ::std::string::String,
        >,
        transport_link_fault: ::std::result::Result<
            ::std::option::Option<super::TransportLinkFault>,
            ::std::string::String,
        >,
        transport_link_gps_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportLinkGpsConfiguration>,
            ::std::string::String,
        >,
        transport_link_gps_dynamic: ::std::result::Result<
            ::std::option::Option<super::TransportLinkGpsDynamic>,
            ::std::string::String,
        >,
        transport_link_history_entry: ::std::result::Result<
            ::std::option::Option<super::TransportLinkHistoryEntry>,
            ::std::string::String,
        >,
        transport_link_measurement: ::std::result::Result<
            ::std::option::Option<super::TransportLinkMeasurement>,
            ::std::string::String,
        >,
        transport_link_scoot_configuration: ::std::result::Result<
            ::std::option::Option<super::TransportLinkScootConfiguration>,
            ::std::string::String,
        >,
        transport_link_scoot_dynamic: ::std::result::Result<
            ::std::option::Option<super::TransportLinkScootDynamic>,
            ::std::string::String,
        >,
        transport_link_type: ::std::result::Result<
            ::std::option::Option<super::TransportLinkType>,
            ::std::string::String,
        >,
        transport_link_type_union: ::std::result::Result<
            ::std::option::Option<super::TransportLinkTypeUnion>,
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
                link_status_type: Ok(Default::default()),
                point: Ok(Default::default()),
                quality: Ok(Default::default()),
                recalibrate_transport_link_command: Ok(Default::default()),
                request_transport_link_status_command: Ok(Default::default()),
                set_stoc_command: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
                transport_link_anpr_configuration: Ok(Default::default()),
                transport_link_anpr_dynamic: Ok(Default::default()),
                transport_link_command: Ok(Default::default()),
                transport_link_command_response: Ok(Default::default()),
                transport_link_configuration: Ok(Default::default()),
                transport_link_definition: Ok(Default::default()),
                transport_link_fault: Ok(Default::default()),
                transport_link_gps_configuration: Ok(Default::default()),
                transport_link_gps_dynamic: Ok(Default::default()),
                transport_link_history_entry: Ok(Default::default()),
                transport_link_measurement: Ok(Default::default()),
                transport_link_scoot_configuration: Ok(Default::default()),
                transport_link_scoot_dynamic: Ok(Default::default()),
                transport_link_type: Ok(Default::default()),
                transport_link_type_union: Ok(Default::default()),
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
        pub fn link_status_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LinkStatusType>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_status_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_status_type: {e}"));
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
        pub fn recalibrate_transport_link_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RecalibrateTransportLinkCommand>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.recalibrate_transport_link_command = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for recalibrate_transport_link_command: {e}"
                )
            });
            self
        }
        pub fn request_transport_link_status_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RequestTransportLinkStatusCommand>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self . request_transport_link_status_command = value . try_into () . map_err (| e | format ! ("error converting supplied value for request_transport_link_status_command: {e}")) ;
            self
        }
        pub fn set_stoc_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SetStocCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.set_stoc_command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for set_stoc_command: {e}"));
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
        pub fn transport_link_anpr_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportLinkAnprConfiguration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_anpr_configuration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transport_link_anpr_configuration: {e}"
                )
            });
            self
        }
        pub fn transport_link_anpr_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkAnprDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_anpr_dynamic = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_anpr_dynamic: {e}")
            });
            self
        }
        pub fn transport_link_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_command: {e}")
            });
            self
        }
        pub fn transport_link_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_command_response: {e}")
            });
            self
        }
        pub fn transport_link_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_configuration: {e}")
            });
            self
        }
        pub fn transport_link_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_definition: {e}")
            });
            self
        }
        pub fn transport_link_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkFault>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_fault = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_fault: {e}")
            });
            self
        }
        pub fn transport_link_gps_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkGpsConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_gps_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_gps_configuration: {e}")
            });
            self
        }
        pub fn transport_link_gps_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkGpsDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_gps_dynamic = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_gps_dynamic: {e}")
            });
            self
        }
        pub fn transport_link_history_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkHistoryEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_history_entry = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_history_entry: {e}")
            });
            self
        }
        pub fn transport_link_measurement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkMeasurement>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_measurement = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_measurement: {e}")
            });
            self
        }
        pub fn transport_link_scoot_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TransportLinkScootConfiguration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_scoot_configuration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for transport_link_scoot_configuration: {e}"
                )
            });
            self
        }
        pub fn transport_link_scoot_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkScootDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_scoot_dynamic = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_scoot_dynamic: {e}")
            });
            self
        }
        pub fn transport_link_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkType>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_type: {e}")
            });
            self
        }
        pub fn transport_link_type_union<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TransportLinkTypeUnion>>,
            T::Error: ::std::fmt::Display,
        {
            self.transport_link_type_union = value.try_into().map_err(|e| {
                format!("error converting supplied value for transport_link_type_union: {e}")
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
                link_status_type: value.link_status_type?,
                point: value.point?,
                quality: value.quality?,
                recalibrate_transport_link_command: value.recalibrate_transport_link_command?,
                request_transport_link_status_command: value
                    .request_transport_link_status_command?,
                set_stoc_command: value.set_stoc_command?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
                transport_link_anpr_configuration: value.transport_link_anpr_configuration?,
                transport_link_anpr_dynamic: value.transport_link_anpr_dynamic?,
                transport_link_command: value.transport_link_command?,
                transport_link_command_response: value.transport_link_command_response?,
                transport_link_configuration: value.transport_link_configuration?,
                transport_link_definition: value.transport_link_definition?,
                transport_link_fault: value.transport_link_fault?,
                transport_link_gps_configuration: value.transport_link_gps_configuration?,
                transport_link_gps_dynamic: value.transport_link_gps_dynamic?,
                transport_link_history_entry: value.transport_link_history_entry?,
                transport_link_measurement: value.transport_link_measurement?,
                transport_link_scoot_configuration: value.transport_link_scoot_configuration?,
                transport_link_scoot_dynamic: value.transport_link_scoot_dynamic?,
                transport_link_type: value.transport_link_type?,
                transport_link_type_union: value.transport_link_type_union?,
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
                link_status_type: Ok(value.link_status_type),
                point: Ok(value.point),
                quality: Ok(value.quality),
                recalibrate_transport_link_command: Ok(value.recalibrate_transport_link_command),
                request_transport_link_status_command: Ok(
                    value.request_transport_link_status_command
                ),
                set_stoc_command: Ok(value.set_stoc_command),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
                transport_link_anpr_configuration: Ok(value.transport_link_anpr_configuration),
                transport_link_anpr_dynamic: Ok(value.transport_link_anpr_dynamic),
                transport_link_command: Ok(value.transport_link_command),
                transport_link_command_response: Ok(value.transport_link_command_response),
                transport_link_configuration: Ok(value.transport_link_configuration),
                transport_link_definition: Ok(value.transport_link_definition),
                transport_link_fault: Ok(value.transport_link_fault),
                transport_link_gps_configuration: Ok(value.transport_link_gps_configuration),
                transport_link_gps_dynamic: Ok(value.transport_link_gps_dynamic),
                transport_link_history_entry: Ok(value.transport_link_history_entry),
                transport_link_measurement: Ok(value.transport_link_measurement),
                transport_link_scoot_configuration: Ok(value.transport_link_scoot_configuration),
                transport_link_scoot_dynamic: Ok(value.transport_link_scoot_dynamic),
                transport_link_type: Ok(value.transport_link_type),
                transport_link_type_union: Ok(value.transport_link_type_union),
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
    pub struct LinkStatusType {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::LinkStatusTypeKind, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for LinkStatusType {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                notes: Ok(Default::default()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl LinkStatusType {
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LinkStatusTypeKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
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
    impl ::std::convert::TryFrom<LinkStatusType> for super::LinkStatusType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LinkStatusType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                kind: value.kind?,
                notes: value.notes?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::LinkStatusType> for LinkStatusType {
        fn from(value: super::LinkStatusType) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                notes: Ok(value.notes),
                type_id: Ok(value.type_id),
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
    pub struct RecalibrateTransportLinkCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<
            super::RecalibrateTransportLinkCommandKind,
            ::std::string::String,
        >,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for RecalibrateTransportLinkCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl RecalibrateTransportLinkCommand {
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
            T: ::std::convert::TryInto<super::RecalibrateTransportLinkCommandKind>,
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
    }
    impl ::std::convert::TryFrom<RecalibrateTransportLinkCommand>
        for super::RecalibrateTransportLinkCommand
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RecalibrateTransportLinkCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::RecalibrateTransportLinkCommand>
        for RecalibrateTransportLinkCommand
    {
        fn from(value: super::RecalibrateTransportLinkCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestTransportLinkStatusCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<
            super::RequestTransportLinkStatusCommandKind,
            ::std::string::String,
        >,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for RequestTransportLinkStatusCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl RequestTransportLinkStatusCommand {
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
            T: ::std::convert::TryInto<super::RequestTransportLinkStatusCommandKind>,
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
    }
    impl ::std::convert::TryFrom<RequestTransportLinkStatusCommand>
        for super::RequestTransportLinkStatusCommand
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestTransportLinkStatusCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::RequestTransportLinkStatusCommand>
        for RequestTransportLinkStatusCommand
    {
        fn from(value: super::RequestTransportLinkStatusCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetStocCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SetStocCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        stoc_value: ::std::result::Result<::std::string::String, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for SetStocCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                stoc_value: Err("no value supplied for stoc_value".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl SetStocCommand {
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
            T: ::std::convert::TryInto<super::SetStocCommandKind>,
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
        pub fn stoc_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.stoc_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stoc_value: {e}"));
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
    }
    impl ::std::convert::TryFrom<SetStocCommand> for super::SetStocCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetStocCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                stoc_value: value.stoc_value?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::SetStocCommand> for SetStocCommand {
        fn from(value: super::SetStocCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                stoc_value: Ok(value.stoc_value),
                transport_link_id: Ok(value.transport_link_id),
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
    pub struct TransportLinkAnprConfiguration {
        capacity_vehicles_per_hour:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        full_length_metres:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        kind:
            ::std::result::Result<super::TransportLinkAnprConfigurationKind, ::std::string::String>,
        length_metres: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        maximum_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        number_of_lanes: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkAnprConfiguration {
        fn default() -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(Default::default()),
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                extensions: Ok(Default::default()),
                full_length_metres: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                length_metres: Ok(Default::default()),
                maximum_speed_kph: Ok(Default::default()),
                number_of_lanes: Ok(Default::default()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkAnprConfiguration {
        pub fn capacity_vehicles_per_hour<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_vehicles_per_hour = value.try_into().map_err(|e| {
                format!("error converting supplied value for capacity_vehicles_per_hour: {e}")
            });
            self
        }
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
        pub fn full_length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_length_metres = value.try_into().map_err(|e| {
                format!("error converting supplied value for full_length_metres: {e}")
            });
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TransportLinkAnprConfigurationKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.length_metres = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for length_metres: {e}"));
            self
        }
        pub fn maximum_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum_speed_kph: {e}"));
            self
        }
        pub fn number_of_lanes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_lanes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_of_lanes: {e}"));
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
    }
    impl ::std::convert::TryFrom<TransportLinkAnprConfiguration>
        for super::TransportLinkAnprConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkAnprConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity_vehicles_per_hour: value.capacity_vehicles_per_hour?,
                configuration_date: value.configuration_date?,
                extensions: value.extensions?,
                full_length_metres: value.full_length_metres?,
                kind: value.kind?,
                length_metres: value.length_metres?,
                maximum_speed_kph: value.maximum_speed_kph?,
                number_of_lanes: value.number_of_lanes?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkAnprConfiguration>
        for TransportLinkAnprConfiguration
    {
        fn from(value: super::TransportLinkAnprConfiguration) -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(value.capacity_vehicles_per_hour),
                configuration_date: Ok(value.configuration_date),
                extensions: Ok(value.extensions),
                full_length_metres: Ok(value.full_length_metres),
                kind: Ok(value.kind),
                length_metres: Ok(value.length_metres),
                maximum_speed_kph: Ok(value.maximum_speed_kph),
                number_of_lanes: Ok(value.number_of_lanes),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkAnprDynamic {
        average_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::TransportLinkAnprDynamicKind, ::std::string::String>,
        link_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        number_plates_in: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        number_plates_matches:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        number_plates_matches_used:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        number_plates_out: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkAnprDynamic {
        fn default() -> Self {
            Self {
                average_speed_kph: Ok(Default::default()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                link_travel_time_seconds: Ok(Default::default()),
                number_plates_in: Ok(Default::default()),
                number_plates_matches: Ok(Default::default()),
                number_plates_matches_used: Ok(Default::default()),
                number_plates_out: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkAnprDynamic {
        pub fn average_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.average_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for average_speed_kph: {e}"));
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
            T: ::std::convert::TryInto<super::TransportLinkAnprDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn link_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for link_travel_time_seconds: {e}")
            });
            self
        }
        pub fn number_plates_in<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_plates_in = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_plates_in: {e}"));
            self
        }
        pub fn number_plates_matches<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_plates_matches = value.try_into().map_err(|e| {
                format!("error converting supplied value for number_plates_matches: {e}")
            });
            self
        }
        pub fn number_plates_matches_used<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_plates_matches_used = value.try_into().map_err(|e| {
                format!("error converting supplied value for number_plates_matches_used: {e}")
            });
            self
        }
        pub fn number_plates_out<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_plates_out = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_plates_out: {e}"));
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
    }
    impl ::std::convert::TryFrom<TransportLinkAnprDynamic> for super::TransportLinkAnprDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkAnprDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                average_speed_kph: value.average_speed_kph?,
                extensions: value.extensions?,
                kind: value.kind?,
                link_travel_time_seconds: value.link_travel_time_seconds?,
                number_plates_in: value.number_plates_in?,
                number_plates_matches: value.number_plates_matches?,
                number_plates_matches_used: value.number_plates_matches_used?,
                number_plates_out: value.number_plates_out?,
                quality: value.quality?,
                timestamp: value.timestamp?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkAnprDynamic> for TransportLinkAnprDynamic {
        fn from(value: super::TransportLinkAnprDynamic) -> Self {
            Self {
                average_speed_kph: Ok(value.average_speed_kph),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                link_travel_time_seconds: Ok(value.link_travel_time_seconds),
                number_plates_in: Ok(value.number_plates_in),
                number_plates_matches: Ok(value.number_plates_matches),
                number_plates_matches_used: Ok(value.number_plates_matches_used),
                number_plates_out: Ok(value.number_plates_out),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkCommandResponse {
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
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkCommandResponse {
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
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkCommandResponse {
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
    }
    impl ::std::convert::TryFrom<TransportLinkCommandResponse> for super::TransportLinkCommandResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                status: value.status?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkCommandResponse> for TransportLinkCommandResponse {
        fn from(value: super::TransportLinkCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                status: Ok(value.status),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkDefinition {
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
            ::std::option::Option<super::TransportLinkDefinitionLongDescription>,
            ::std::string::String,
        >,
        originating_object_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        originating_participant_id: ::std::result::Result<
            ::std::option::Option<super::ParticipantId>,
            ::std::string::String,
        >,
        participant_id: ::std::result::Result<
            ::std::option::Option<super::ParticipantId>,
            ::std::string::String,
        >,
        short_description: ::std::result::Result<
            ::std::option::Option<super::TransportLinkDefinitionShortDescription>,
            ::std::string::String,
        >,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkDefinition {
        fn default() -> Self {
            Self {
                creation_date: Err("no value supplied for creation_date".to_string()),
                end_point: Ok(Default::default()),
                extensions: Ok(Default::default()),
                location: Ok(Default::default()),
                long_description: Ok(Default::default()),
                originating_object_id: Ok(Default::default()),
                originating_participant_id: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                short_description: Ok(Default::default()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl TransportLinkDefinition {
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
                ::std::option::Option<super::TransportLinkDefinitionLongDescription>,
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
                ::std::option::Option<super::TransportLinkDefinitionShortDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.short_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_description: {e}"));
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
    impl ::std::convert::TryFrom<TransportLinkDefinition> for super::TransportLinkDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                creation_date: value.creation_date?,
                end_point: value.end_point?,
                extensions: value.extensions?,
                location: value.location?,
                long_description: value.long_description?,
                originating_object_id: value.originating_object_id?,
                originating_participant_id: value.originating_participant_id?,
                participant_id: value.participant_id?,
                short_description: value.short_description?,
                transport_link_id: value.transport_link_id?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkDefinition> for TransportLinkDefinition {
        fn from(value: super::TransportLinkDefinition) -> Self {
            Self {
                creation_date: Ok(value.creation_date),
                end_point: Ok(value.end_point),
                extensions: Ok(value.extensions),
                location: Ok(value.location),
                long_description: Ok(value.long_description),
                originating_object_id: Ok(value.originating_object_id),
                originating_participant_id: Ok(value.originating_participant_id),
                participant_id: Ok(value.participant_id),
                short_description: Ok(value.short_description),
                transport_link_id: Ok(value.transport_link_id),
                type_id: Ok(value.type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkFault {
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
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkFault {
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
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkFault {
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
    }
    impl ::std::convert::TryFrom<TransportLinkFault> for super::TransportLinkFault {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkFault,
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
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkFault> for TransportLinkFault {
        fn from(value: super::TransportLinkFault) -> Self {
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
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkGpsConfiguration {
        capacity_vehicles_per_hour:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        full_length_metres:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        kind:
            ::std::result::Result<super::TransportLinkGpsConfigurationKind, ::std::string::String>,
        length_metres: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        maximum_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        number_of_lanes: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkGpsConfiguration {
        fn default() -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(Default::default()),
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                extensions: Ok(Default::default()),
                full_length_metres: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                length_metres: Ok(Default::default()),
                maximum_speed_kph: Ok(Default::default()),
                number_of_lanes: Ok(Default::default()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkGpsConfiguration {
        pub fn capacity_vehicles_per_hour<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_vehicles_per_hour = value.try_into().map_err(|e| {
                format!("error converting supplied value for capacity_vehicles_per_hour: {e}")
            });
            self
        }
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
        pub fn full_length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_length_metres = value.try_into().map_err(|e| {
                format!("error converting supplied value for full_length_metres: {e}")
            });
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TransportLinkGpsConfigurationKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.length_metres = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for length_metres: {e}"));
            self
        }
        pub fn maximum_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum_speed_kph: {e}"));
            self
        }
        pub fn number_of_lanes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_lanes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_of_lanes: {e}"));
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
    }
    impl ::std::convert::TryFrom<TransportLinkGpsConfiguration>
        for super::TransportLinkGpsConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkGpsConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity_vehicles_per_hour: value.capacity_vehicles_per_hour?,
                configuration_date: value.configuration_date?,
                extensions: value.extensions?,
                full_length_metres: value.full_length_metres?,
                kind: value.kind?,
                length_metres: value.length_metres?,
                maximum_speed_kph: value.maximum_speed_kph?,
                number_of_lanes: value.number_of_lanes?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkGpsConfiguration> for TransportLinkGpsConfiguration {
        fn from(value: super::TransportLinkGpsConfiguration) -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(value.capacity_vehicles_per_hour),
                configuration_date: Ok(value.configuration_date),
                extensions: Ok(value.extensions),
                full_length_metres: Ok(value.full_length_metres),
                kind: Ok(value.kind),
                length_metres: Ok(value.length_metres),
                maximum_speed_kph: Ok(value.maximum_speed_kph),
                number_of_lanes: Ok(value.number_of_lanes),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkGpsDynamic {
        average_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        current_journey_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::TransportLinkGpsDynamicKind, ::std::string::String>,
        link_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        mean_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        median_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        number_of_vehicles:
            ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        reserved_lane_flow_vehicles:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        variance_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkGpsDynamic {
        fn default() -> Self {
            Self {
                average_speed_kph: Ok(Default::default()),
                current_journey_time_seconds: Ok(Default::default()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                link_travel_time_seconds: Ok(Default::default()),
                mean_travel_time_seconds: Ok(Default::default()),
                median_travel_time_seconds: Ok(Default::default()),
                number_of_vehicles: Ok(Default::default()),
                quality: Ok(Default::default()),
                reserved_lane_flow_vehicles: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
                variance_travel_time_seconds: Ok(Default::default()),
            }
        }
    }
    impl TransportLinkGpsDynamic {
        pub fn average_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.average_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for average_speed_kph: {e}"));
            self
        }
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
            T: ::std::convert::TryInto<super::TransportLinkGpsDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn link_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for link_travel_time_seconds: {e}")
            });
            self
        }
        pub fn mean_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mean_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for mean_travel_time_seconds: {e}")
            });
            self
        }
        pub fn median_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.median_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for median_travel_time_seconds: {e}")
            });
            self
        }
        pub fn number_of_vehicles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_vehicles = value.try_into().map_err(|e| {
                format!("error converting supplied value for number_of_vehicles: {e}")
            });
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
        pub fn reserved_lane_flow_vehicles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.reserved_lane_flow_vehicles = value.try_into().map_err(|e| {
                format!("error converting supplied value for reserved_lane_flow_vehicles: {e}")
            });
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
        pub fn variance_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.variance_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for variance_travel_time_seconds: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TransportLinkGpsDynamic> for super::TransportLinkGpsDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkGpsDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                average_speed_kph: value.average_speed_kph?,
                current_journey_time_seconds: value.current_journey_time_seconds?,
                extensions: value.extensions?,
                kind: value.kind?,
                link_travel_time_seconds: value.link_travel_time_seconds?,
                mean_travel_time_seconds: value.mean_travel_time_seconds?,
                median_travel_time_seconds: value.median_travel_time_seconds?,
                number_of_vehicles: value.number_of_vehicles?,
                quality: value.quality?,
                reserved_lane_flow_vehicles: value.reserved_lane_flow_vehicles?,
                timestamp: value.timestamp?,
                transport_link_id: value.transport_link_id?,
                variance_travel_time_seconds: value.variance_travel_time_seconds?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkGpsDynamic> for TransportLinkGpsDynamic {
        fn from(value: super::TransportLinkGpsDynamic) -> Self {
            Self {
                average_speed_kph: Ok(value.average_speed_kph),
                current_journey_time_seconds: Ok(value.current_journey_time_seconds),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                link_travel_time_seconds: Ok(value.link_travel_time_seconds),
                mean_travel_time_seconds: Ok(value.mean_travel_time_seconds),
                median_travel_time_seconds: Ok(value.median_travel_time_seconds),
                number_of_vehicles: Ok(value.number_of_vehicles),
                quality: Ok(value.quality),
                reserved_lane_flow_vehicles: Ok(value.reserved_lane_flow_vehicles),
                timestamp: Ok(value.timestamp),
                transport_link_id: Ok(value.transport_link_id),
                variance_travel_time_seconds: Ok(value.variance_travel_time_seconds),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkHistoryEntry {
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
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkHistoryEntry {
        fn default() -> Self {
            Self {
                event_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                log_date: Err("no value supplied for log_date".to_string()),
                log_entry: Err("no value supplied for log_entry".to_string()),
                log_id: Err("no value supplied for log_id".to_string()),
                log_user: Ok(Default::default()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkHistoryEntry {
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
    }
    impl ::std::convert::TryFrom<TransportLinkHistoryEntry> for super::TransportLinkHistoryEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkHistoryEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
                extensions: value.extensions?,
                log_date: value.log_date?,
                log_entry: value.log_entry?,
                log_id: value.log_id?,
                log_user: value.log_user?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkHistoryEntry> for TransportLinkHistoryEntry {
        fn from(value: super::TransportLinkHistoryEntry) -> Self {
            Self {
                event_type: Ok(value.event_type),
                extensions: Ok(value.extensions),
                log_date: Ok(value.log_date),
                log_entry: Ok(value.log_entry),
                log_id: Ok(value.log_id),
                log_user: Ok(value.log_user),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkScootConfiguration {
        capacity_vehicles_per_hour:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        full_length_metres:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        kind: ::std::result::Result<
            super::TransportLinkScootConfigurationKind,
            ::std::string::String,
        >,
        length_metres: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        maximum_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        number_of_lanes: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkScootConfiguration {
        fn default() -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(Default::default()),
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                extensions: Ok(Default::default()),
                full_length_metres: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                length_metres: Ok(Default::default()),
                maximum_speed_kph: Ok(Default::default()),
                number_of_lanes: Ok(Default::default()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkScootConfiguration {
        pub fn capacity_vehicles_per_hour<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_vehicles_per_hour = value.try_into().map_err(|e| {
                format!("error converting supplied value for capacity_vehicles_per_hour: {e}")
            });
            self
        }
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
        pub fn full_length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.full_length_metres = value.try_into().map_err(|e| {
                format!("error converting supplied value for full_length_metres: {e}")
            });
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TransportLinkScootConfigurationKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn length_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.length_metres = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for length_metres: {e}"));
            self
        }
        pub fn maximum_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum_speed_kph: {e}"));
            self
        }
        pub fn number_of_lanes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_lanes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_of_lanes: {e}"));
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
    }
    impl ::std::convert::TryFrom<TransportLinkScootConfiguration>
        for super::TransportLinkScootConfiguration
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkScootConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity_vehicles_per_hour: value.capacity_vehicles_per_hour?,
                configuration_date: value.configuration_date?,
                extensions: value.extensions?,
                full_length_metres: value.full_length_metres?,
                kind: value.kind?,
                length_metres: value.length_metres?,
                maximum_speed_kph: value.maximum_speed_kph?,
                number_of_lanes: value.number_of_lanes?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkScootConfiguration>
        for TransportLinkScootConfiguration
    {
        fn from(value: super::TransportLinkScootConfiguration) -> Self {
            Self {
                capacity_vehicles_per_hour: Ok(value.capacity_vehicles_per_hour),
                configuration_date: Ok(value.configuration_date),
                extensions: Ok(value.extensions),
                full_length_metres: Ok(value.full_length_metres),
                kind: Ok(value.kind),
                length_metres: Ok(value.length_metres),
                maximum_speed_kph: Ok(value.maximum_speed_kph),
                number_of_lanes: Ok(value.number_of_lanes),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkScootDynamic {
        average_speed_kph: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        congestion_percent:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        current_flow_vehicles:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::TransportLinkScootDynamicKind, ::std::string::String>,
        link_travel_time_seconds:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        occupancy_percent: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        transport_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkScootDynamic {
        fn default() -> Self {
            Self {
                average_speed_kph: Ok(Default::default()),
                congestion_percent: Ok(Default::default()),
                current_flow_vehicles: Ok(Default::default()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                link_travel_time_seconds: Ok(Default::default()),
                occupancy_percent: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                transport_link_id: Err("no value supplied for transport_link_id".to_string()),
            }
        }
    }
    impl TransportLinkScootDynamic {
        pub fn average_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.average_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for average_speed_kph: {e}"));
            self
        }
        pub fn congestion_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.congestion_percent = value.try_into().map_err(|e| {
                format!("error converting supplied value for congestion_percent: {e}")
            });
            self
        }
        pub fn current_flow_vehicles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.current_flow_vehicles = value.try_into().map_err(|e| {
                format!("error converting supplied value for current_flow_vehicles: {e}")
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
            T: ::std::convert::TryInto<super::TransportLinkScootDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn link_travel_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_travel_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for link_travel_time_seconds: {e}")
            });
            self
        }
        pub fn occupancy_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.occupancy_percent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occupancy_percent: {e}"));
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
    }
    impl ::std::convert::TryFrom<TransportLinkScootDynamic> for super::TransportLinkScootDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkScootDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                average_speed_kph: value.average_speed_kph?,
                congestion_percent: value.congestion_percent?,
                current_flow_vehicles: value.current_flow_vehicles?,
                extensions: value.extensions?,
                kind: value.kind?,
                link_travel_time_seconds: value.link_travel_time_seconds?,
                occupancy_percent: value.occupancy_percent?,
                quality: value.quality?,
                timestamp: value.timestamp?,
                transport_link_id: value.transport_link_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkScootDynamic> for TransportLinkScootDynamic {
        fn from(value: super::TransportLinkScootDynamic) -> Self {
            Self {
                average_speed_kph: Ok(value.average_speed_kph),
                congestion_percent: Ok(value.congestion_percent),
                current_flow_vehicles: Ok(value.current_flow_vehicles),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                link_travel_time_seconds: Ok(value.link_travel_time_seconds),
                occupancy_percent: Ok(value.occupancy_percent),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
                transport_link_id: Ok(value.transport_link_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TransportLinkType {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::TransportLinkTypeKind, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        specification_link: ::std::result::Result<
            ::std::option::Option<super::TransportLinkTypeSpecificationLink>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for TransportLinkType {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                notes: Ok(Default::default()),
                specification_link: Ok(Default::default()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl TransportLinkType {
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TransportLinkTypeKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
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
                ::std::option::Option<super::TransportLinkTypeSpecificationLink>,
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
    impl ::std::convert::TryFrom<TransportLinkType> for super::TransportLinkType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TransportLinkType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                kind: value.kind?,
                notes: value.notes?,
                specification_link: value.specification_link?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::TransportLinkType> for TransportLinkType {
        fn from(value: super::TransportLinkType) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                notes: Ok(value.notes),
                specification_link: Ok(value.specification_link),
                type_id: Ok(value.type_id),
            }
        }
    }
}
