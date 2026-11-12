/*
 * File:        output/rust/sdk/src/vms.rs
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
#[doc = "    \"ClearSignCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/ClearSignCommand\""]
#[doc = "    },"]
#[doc = "    \"CommandResponseStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"Coordinates\": {"]
#[doc = "      \"$ref\": \"#/$defs/Coordinates\""]
#[doc = "    },"]
#[doc = "    \"DisplayGraphicCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/DisplayGraphicCommand\""]
#[doc = "    },"]
#[doc = "    \"DisplayMessageCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/DisplayMessageCommand\""]
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
#[doc = "    \"SetBrightnessCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/SetBrightnessCommand\""]
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
#[doc = "    \"VmsColourCapability\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsColourCapability\""]
#[doc = "    },"]
#[doc = "    \"VmsCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsCommand\""]
#[doc = "    },"]
#[doc = "    \"VmsCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"VmsConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsConfiguration\""]
#[doc = "    },"]
#[doc = "    \"VmsDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsDefinition\""]
#[doc = "    },"]
#[doc = "    \"VmsDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsDynamic\""]
#[doc = "    },"]
#[doc = "    \"VmsFault\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsFault\""]
#[doc = "    },"]
#[doc = "    \"VmsHistoryEntry\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsHistoryEntry\""]
#[doc = "    },"]
#[doc = "    \"VmsStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsStatus\""]
#[doc = "    },"]
#[doc = "    \"VmsType\": {"]
#[doc = "      \"$ref\": \"#/$defs/VmsType\""]
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
        rename = "ClearSignCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub clear_sign_command: ::std::option::Option<ClearSignCommand>,
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
        rename = "DisplayGraphicCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_graphic_command: ::std::option::Option<DisplayGraphicCommand>,
    #[serde(
        rename = "DisplayMessageCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_message_command: ::std::option::Option<DisplayMessageCommand>,
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
        rename = "SetBrightnessCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub set_brightness_command: ::std::option::Option<SetBrightnessCommand>,
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
        rename = "VmsColourCapability",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_colour_capability: ::std::option::Option<VmsColourCapability>,
    #[serde(
        rename = "VmsCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_command: ::std::option::Option<VmsCommand>,
    #[serde(
        rename = "VmsCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_command_response: ::std::option::Option<VmsCommandResponse>,
    #[serde(
        rename = "VmsConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_configuration: ::std::option::Option<VmsConfiguration>,
    #[serde(
        rename = "VmsDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_definition: ::std::option::Option<VmsDefinition>,
    #[serde(
        rename = "VmsDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_dynamic: ::std::option::Option<VmsDynamic>,
    #[serde(
        rename = "VmsFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_fault: ::std::option::Option<VmsFault>,
    #[serde(
        rename = "VmsHistoryEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_history_entry: ::std::option::Option<VmsHistoryEntry>,
    #[serde(
        rename = "VmsStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_status: ::std::option::Option<VmsStatus>,
    #[serde(
        rename = "VmsType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub vms_type: ::std::option::Option<VmsType>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            acknowledgement_state: Default::default(),
            clear_sign_command: Default::default(),
            command_response_status: Default::default(),
            coordinates: Default::default(),
            display_graphic_command: Default::default(),
            display_message_command: Default::default(),
            fault_status: Default::default(),
            point: Default::default(),
            quality: Default::default(),
            set_brightness_command: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
            vms_colour_capability: Default::default(),
            vms_command: Default::default(),
            vms_command_response: Default::default(),
            vms_configuration: Default::default(),
            vms_definition: Default::default(),
            vms_dynamic: Default::default(),
            vms_fault: Default::default(),
            vms_history_entry: Default::default(),
            vms_status: Default::default(),
            vms_type: Default::default(),
        }
    }
}
impl AllTypes {
    pub fn builder() -> builder::AllTypes {
        Default::default()
    }
}
#[doc = "Clear the sign (blank the display)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Clear the sign (blank the display).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier.\","]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"clearSign\\\" selects the ClearSignCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"clearSign\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS receiving the command.\","]
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
pub struct ClearSignCommand {
    #[doc = "Caller-assigned identifier."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"clearSign\" selects the ClearSignCommand payload; it is not free-text command content."]
    pub kind: ClearSignCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the VMS receiving the command."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl ClearSignCommand {
    pub fn builder() -> builder::ClearSignCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"clearSign\" selects the ClearSignCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"clearSign\\\" selects the ClearSignCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"clearSign\""]
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
pub enum ClearSignCommandKind {
    #[serde(rename = "clearSign")]
    ClearSign,
}
impl ::std::fmt::Display for ClearSignCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClearSign => f.write_str("clearSign"),
        }
    }
}
impl ::std::str::FromStr for ClearSignCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "clearSign" => Ok(Self::ClearSign),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ClearSignCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClearSignCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClearSignCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "Display a graphic on the sign."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Display a graphic on the sign.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"graphicUrl\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"displayDurationSeconds\": {"]
#[doc = "      \"description\": \"How long to display the graphic.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"graphicUrl\": {"]
#[doc = "      \"description\": \"URL of the graphic to display.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"displayGraphic\\\" selects the DisplayGraphicCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"displayGraphic\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS receiving the command.\","]
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
pub struct DisplayGraphicCommand {
    #[doc = "Caller-assigned identifier."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "How long to display the graphic."]
    #[serde(
        rename = "displayDurationSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_duration_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "URL of the graphic to display."]
    #[serde(rename = "graphicUrl")]
    pub graphic_url: DisplayGraphicCommandGraphicUrl,
    #[doc = "Discriminator for the tagged union. The fixed value \"displayGraphic\" selects the DisplayGraphicCommand payload; it is not free-text command content."]
    pub kind: DisplayGraphicCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the VMS receiving the command."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl DisplayGraphicCommand {
    pub fn builder() -> builder::DisplayGraphicCommand {
        Default::default()
    }
}
#[doc = "URL of the graphic to display."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL of the graphic to display.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DisplayGraphicCommandGraphicUrl(::std::string::String);
impl ::std::ops::Deref for DisplayGraphicCommandGraphicUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DisplayGraphicCommandGraphicUrl> for ::std::string::String {
    fn from(value: DisplayGraphicCommandGraphicUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DisplayGraphicCommandGraphicUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DisplayGraphicCommandGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DisplayGraphicCommandGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DisplayGraphicCommandGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DisplayGraphicCommandGraphicUrl {
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
#[doc = "Discriminator for the tagged union. The fixed value \"displayGraphic\" selects the DisplayGraphicCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"displayGraphic\\\" selects the DisplayGraphicCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"displayGraphic\""]
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
pub enum DisplayGraphicCommandKind {
    #[serde(rename = "displayGraphic")]
    DisplayGraphic,
}
impl ::std::fmt::Display for DisplayGraphicCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DisplayGraphic => f.write_str("displayGraphic"),
        }
    }
}
impl ::std::str::FromStr for DisplayGraphicCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "displayGraphic" => Ok(Self::DisplayGraphic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DisplayGraphicCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DisplayGraphicCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DisplayGraphicCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Display a text message on the sign."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Display a text message on the sign.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"message\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"displayDurationSeconds\": {"]
#[doc = "      \"description\": \"How long to display the message.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"displayMessage\\\" selects the DisplayMessageCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"displayMessage\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Message text to display.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"messageLines\": {"]
#[doc = "      \"description\": \"Message split by lines.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS receiving the command.\","]
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
pub struct DisplayMessageCommand {
    #[doc = "Caller-assigned identifier."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "How long to display the message."]
    #[serde(
        rename = "displayDurationSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_duration_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"displayMessage\" selects the DisplayMessageCommand payload; it is not free-text command content."]
    pub kind: DisplayMessageCommandKind,
    #[doc = "Message text to display."]
    pub message: ::std::string::String,
    #[doc = "Message split by lines."]
    #[serde(
        rename = "messageLines",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub message_lines: ::std::vec::Vec<::std::string::String>,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the VMS receiving the command."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl DisplayMessageCommand {
    pub fn builder() -> builder::DisplayMessageCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"displayMessage\" selects the DisplayMessageCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"displayMessage\\\" selects the DisplayMessageCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"displayMessage\""]
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
pub enum DisplayMessageCommandKind {
    #[serde(rename = "displayMessage")]
    DisplayMessage,
}
impl ::std::fmt::Display for DisplayMessageCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DisplayMessage => f.write_str("displayMessage"),
        }
    }
}
impl ::std::str::FromStr for DisplayMessageCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "displayMessage" => Ok(Self::DisplayMessage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DisplayMessageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DisplayMessageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DisplayMessageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "Identifier of the VMS receiving the command."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Identifier of the VMS receiving the command.\","]
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
#[doc = "Set the brightness of the sign."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Set the brightness of the sign.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"brightnessPercent\","]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"brightnessPercent\": {"]
#[doc = "      \"description\": \"Brightness percentage to set.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier.\","]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"setBrightness\\\" selects the SetBrightnessCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"setBrightness\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS receiving the command.\","]
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
pub struct SetBrightnessCommand {
    #[doc = "Brightness percentage to set."]
    #[serde(rename = "brightnessPercent")]
    pub brightness_percent: f64,
    #[doc = "Caller-assigned identifier."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"setBrightness\" selects the SetBrightnessCommand payload; it is not free-text command content."]
    pub kind: SetBrightnessCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the VMS receiving the command."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl SetBrightnessCommand {
    pub fn builder() -> builder::SetBrightnessCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"setBrightness\" selects the SetBrightnessCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"setBrightness\\\" selects the SetBrightnessCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"setBrightness\""]
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
pub enum SetBrightnessCommandKind {
    #[serde(rename = "setBrightness")]
    SetBrightness,
}
impl ::std::fmt::Display for SetBrightnessCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetBrightness => f.write_str("setBrightness"),
        }
    }
}
impl ::std::str::FromStr for SetBrightnessCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "setBrightness" => Ok(Self::SetBrightness),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetBrightnessCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetBrightnessCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetBrightnessCommandKind {
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
#[doc = "Terminal message ending a vms sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a vms sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of vms objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Sync Request\","]
#[doc = "  \"description\": \"Request the current state of vms objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Colour capability of a variable message sign."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Colour capability of a variable message sign.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"monochrome\","]
#[doc = "    \"amber\","]
#[doc = "    \"multiColour\","]
#[doc = "    \"fullColour\""]
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
pub enum VmsColourCapability {
    #[serde(rename = "monochrome")]
    Monochrome,
    #[serde(rename = "amber")]
    Amber,
    #[serde(rename = "multiColour")]
    MultiColour,
    #[serde(rename = "fullColour")]
    FullColour,
}
impl ::std::fmt::Display for VmsColourCapability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Monochrome => f.write_str("monochrome"),
            Self::Amber => f.write_str("amber"),
            Self::MultiColour => f.write_str("multiColour"),
            Self::FullColour => f.write_str("fullColour"),
        }
    }
}
impl ::std::str::FromStr for VmsColourCapability {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "monochrome" => Ok(Self::Monochrome),
            "amber" => Ok(Self::Amber),
            "multiColour" => Ok(Self::MultiColour),
            "fullColour" => Ok(Self::FullColour),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VmsColourCapability {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsColourCapability {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsColourCapability {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Variable message sign command payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Command\","]
#[doc = "  \"description\": \"Variable message sign command payloads.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/DisplayMessageCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/DisplayGraphicCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SetBrightnessCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ClearSignCommand\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"clearSign\": \"#/$defs/ClearSignCommand\","]
#[doc = "      \"displayGraphic\": \"#/$defs/DisplayGraphicCommand\","]
#[doc = "      \"displayMessage\": \"#/$defs/DisplayMessageCommand\","]
#[doc = "      \"setBrightness\": \"#/$defs/SetBrightnessCommand\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum VmsCommand {
    DisplayMessageCommand(DisplayMessageCommand),
    DisplayGraphicCommand(DisplayGraphicCommand),
    SetBrightnessCommand(SetBrightnessCommand),
    ClearSignCommand(ClearSignCommand),
}
impl ::std::convert::From<DisplayMessageCommand> for VmsCommand {
    fn from(value: DisplayMessageCommand) -> Self {
        Self::DisplayMessageCommand(value)
    }
}
impl ::std::convert::From<DisplayGraphicCommand> for VmsCommand {
    fn from(value: DisplayGraphicCommand) -> Self {
        Self::DisplayGraphicCommand(value)
    }
}
impl ::std::convert::From<SetBrightnessCommand> for VmsCommand {
    fn from(value: SetBrightnessCommand) -> Self {
        Self::SetBrightnessCommand(value)
    }
}
impl ::std::convert::From<ClearSignCommand> for VmsCommand {
    fn from(value: ClearSignCommand) -> Self {
        Self::ClearSignCommand(value)
    }
}
#[doc = "Variable message sign command response payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Command Response\","]
#[doc = "  \"description\": \"Variable message sign command response payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\","]
#[doc = "    \"vmsId\""]
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
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS the command targeted.\","]
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
pub struct VmsCommandResponse {
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
    #[doc = "Identifier of the VMS the command targeted."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsCommandResponse {
    pub fn builder() -> builder::VmsCommandResponse {
        Default::default()
    }
}
#[doc = "Variable message sign operational configuration."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Configuration\","]
#[doc = "  \"description\": \"Variable message sign operational configuration.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"autoBrightness\": {"]
#[doc = "      \"description\": \"Whether auto-brightness is enabled.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"defaultBrightnessPercent\": {"]
#[doc = "      \"description\": \"Default brightness percentage.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"defaultDisplayTimeSeconds\": {"]
#[doc = "      \"description\": \"Default message display time in seconds.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"scrollSpeed\": {"]
#[doc = "      \"description\": \"Text scroll speed.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"VMS this configuration applies to.\","]
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
pub struct VmsConfiguration {
    #[doc = "Whether auto-brightness is enabled."]
    #[serde(
        rename = "autoBrightness",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub auto_brightness: ::std::option::Option<bool>,
    #[doc = "When effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Default brightness percentage."]
    #[serde(
        rename = "defaultBrightnessPercent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_brightness_percent: ::std::option::Option<f64>,
    #[doc = "Default message display time in seconds."]
    #[serde(
        rename = "defaultDisplayTimeSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_display_time_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Text scroll speed."]
    #[serde(
        rename = "scrollSpeed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scroll_speed: ::std::option::Option<u64>,
    #[doc = "VMS this configuration applies to."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsConfiguration {
    pub fn builder() -> builder::VmsConfiguration {
        Default::default()
    }
}
#[doc = "Variable message sign equipment details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Definition\","]
#[doc = "  \"description\": \"Variable message sign equipment details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"typeId\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"charactersPerLine\": {"]
#[doc = "      \"description\": \"Characters per line.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"colourCapability\": {"]
#[doc = "      \"description\": \"Colour capability.\","]
#[doc = "      \"$ref\": \"#/$defs/VmsColourCapability\""]
#[doc = "    },"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When this definition was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"linkDistanceMetres\": {"]
#[doc = "      \"description\": \"Distance along that transport link, in metres. Optional.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Physical location.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"longDescription\": {"]
#[doc = "      \"description\": \"Longer description.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 2000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"networkLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Network link the sign stands on. At most one of this and networkPathId. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"networkPathId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Network path the sign stands on. At most one of this and networkLinkId. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"numberOfLines\": {"]
#[doc = "      \"description\": \"Number of text lines.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
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
#[doc = "    \"signHeight\": {"]
#[doc = "      \"description\": \"Sign height in pixels/units.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"signWidth\": {"]
#[doc = "      \"description\": \"Sign width in pixels/units.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"supportsGraphics\": {"]
#[doc = "      \"description\": \"Whether the sign supports graphics.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Transport link this sign monitors or sits on. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Unique identifier for a VMS type (sign type catalogue entry).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this VMS.\","]
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
pub struct VmsDefinition {
    #[doc = "Characters per line."]
    #[serde(
        rename = "charactersPerLine",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub characters_per_line: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Colour capability."]
    #[serde(
        rename = "colourCapability",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub colour_capability: ::std::option::Option<VmsColourCapability>,
    #[doc = "When this definition was created."]
    #[serde(rename = "creationDate")]
    pub creation_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance along that transport link, in metres. Optional."]
    #[serde(
        rename = "linkDistanceMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_distance_metres: ::std::option::Option<f64>,
    #[doc = "Physical location."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Longer description."]
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<VmsDefinitionLongDescription>,
    #[doc = "Network link the sign stands on. At most one of this and networkPathId. Optional."]
    #[serde(
        rename = "networkLinkId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_link_id: ::std::option::Option<ObjectId>,
    #[doc = "Network path the sign stands on. At most one of this and networkLinkId. Optional."]
    #[serde(
        rename = "networkPathId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_path_id: ::std::option::Option<ObjectId>,
    #[doc = "Number of text lines."]
    #[serde(
        rename = "numberOfLines",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub number_of_lines: ::std::option::Option<::std::num::NonZeroU64>,
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
    pub short_description: ::std::option::Option<VmsDefinitionShortDescription>,
    #[doc = "Sign height in pixels/units."]
    #[serde(
        rename = "signHeight",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sign_height: ::std::option::Option<u64>,
    #[doc = "Sign width in pixels/units."]
    #[serde(
        rename = "signWidth",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sign_width: ::std::option::Option<u64>,
    #[doc = "Whether the sign supports graphics."]
    #[serde(
        rename = "supportsGraphics",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_graphics: ::std::option::Option<bool>,
    #[doc = "Transport link this sign monitors or sits on. Optional."]
    #[serde(
        rename = "transportLinkId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_id: ::std::option::Option<ObjectId>,
    #[doc = "Unique identifier for a VMS type (sign type catalogue entry)."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
    #[doc = "Unique identifier of this VMS."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsDefinition {
    pub fn builder() -> builder::VmsDefinition {
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
pub struct VmsDefinitionLongDescription(::std::string::String);
impl ::std::ops::Deref for VmsDefinitionLongDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VmsDefinitionLongDescription> for ::std::string::String {
    fn from(value: VmsDefinitionLongDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VmsDefinitionLongDescription {
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
impl ::std::convert::TryFrom<&str> for VmsDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VmsDefinitionLongDescription {
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
pub struct VmsDefinitionShortDescription(::std::string::String);
impl ::std::ops::Deref for VmsDefinitionShortDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VmsDefinitionShortDescription> for ::std::string::String {
    fn from(value: VmsDefinitionShortDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VmsDefinitionShortDescription {
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
impl ::std::convert::TryFrom<&str> for VmsDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VmsDefinitionShortDescription {
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
#[doc = "Variable message sign dynamic measurement data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Measurement\","]
#[doc = "  \"description\": \"Variable message sign dynamic measurement data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"timestamp\","]
#[doc = "    \"vmsId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"brightnessPercent\": {"]
#[doc = "      \"description\": \"Current brightness percentage.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"currentMessage\": {"]
#[doc = "      \"description\": \"Currently displayed message text.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"graphicUrl\": {"]
#[doc = "      \"description\": \"URL of the currently displayed graphic.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"messageExpiryTime\": {"]
#[doc = "      \"description\": \"When the message expires.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"messageLines\": {"]
#[doc = "      \"description\": \"Message split by lines.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"messageSetTime\": {"]
#[doc = "      \"description\": \"When the current message was set.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"Operational status.\","]
#[doc = "      \"$ref\": \"#/$defs/VmsStatus\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time of the observation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS.\","]
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
pub struct VmsDynamic {
    #[doc = "Current brightness percentage."]
    #[serde(
        rename = "brightnessPercent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub brightness_percent: ::std::option::Option<f64>,
    #[doc = "Currently displayed message text."]
    #[serde(
        rename = "currentMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_message: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "URL of the currently displayed graphic."]
    #[serde(
        rename = "graphicUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub graphic_url: ::std::option::Option<VmsDynamicGraphicUrl>,
    #[doc = "When the message expires."]
    #[serde(
        rename = "messageExpiryTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_expiry_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Message split by lines."]
    #[serde(
        rename = "messageLines",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub message_lines: ::std::vec::Vec<::std::string::String>,
    #[doc = "When the current message was set."]
    #[serde(
        rename = "messageSetTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_set_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Optional quality statement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Operational status."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<VmsStatus>,
    #[doc = "Time of the observation."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the VMS."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsDynamic {
    pub fn builder() -> builder::VmsDynamic {
        Default::default()
    }
}
#[doc = "URL of the currently displayed graphic."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL of the currently displayed graphic.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VmsDynamicGraphicUrl(::std::string::String);
impl ::std::ops::Deref for VmsDynamicGraphicUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VmsDynamicGraphicUrl> for ::std::string::String {
    fn from(value: VmsDynamicGraphicUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VmsDynamicGraphicUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VmsDynamicGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsDynamicGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsDynamicGraphicUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VmsDynamicGraphicUrl {
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
#[doc = "Variable message sign fault reports."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Fault\","]
#[doc = "  \"description\": \"Variable message sign fault reports.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"faultId\","]
#[doc = "    \"faultTypeId\","]
#[doc = "    \"vmsId\""]
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
#[doc = "      \"description\": \"When the fault was cleared.\","]
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
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS this fault is about.\","]
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
pub struct VmsFault {
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
    #[doc = "When the fault was cleared."]
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
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Unique identifier of this fault report."]
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
    #[doc = "Identifier of the VMS this fault is about."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsFault {
    pub fn builder() -> builder::VmsFault {
        Default::default()
    }
}
#[doc = "Variable message sign history entries."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS History\","]
#[doc = "  \"description\": \"Variable message sign history entries.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"logDate\","]
#[doc = "    \"logEntry\","]
#[doc = "    \"logId\","]
#[doc = "    \"vmsId\""]
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
#[doc = "      \"description\": \"Equipment log data (free-form text).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"logId\": {"]
#[doc = "      \"description\": \"Unique identifier of this log entry.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"logUser\": {"]
#[doc = "      \"description\": \"User who created the log entry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"vmsId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the VMS this log entry is about.\","]
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
pub struct VmsHistoryEntry {
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
    #[doc = "Equipment log data (free-form text)."]
    #[serde(rename = "logEntry")]
    pub log_entry: ::std::string::String,
    #[doc = "Unique identifier of this log entry."]
    #[serde(rename = "logId")]
    pub log_id: u64,
    #[doc = "User who created the log entry."]
    #[serde(
        rename = "logUser",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub log_user: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the VMS this log entry is about."]
    #[serde(rename = "vmsId")]
    pub vms_id: ObjectId,
}
impl VmsHistoryEntry {
    pub fn builder() -> builder::VmsHistoryEntry {
        Default::default()
    }
}
#[doc = "Operational status of a variable message sign."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Operational status of a variable message sign.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"off\","]
#[doc = "    \"on\","]
#[doc = "    \"blank\","]
#[doc = "    \"displaying\","]
#[doc = "    \"fault\","]
#[doc = "    \"unknown\""]
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
pub enum VmsStatus {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "blank")]
    Blank,
    #[serde(rename = "displaying")]
    Displaying,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for VmsStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Off => f.write_str("off"),
            Self::On => f.write_str("on"),
            Self::Blank => f.write_str("blank"),
            Self::Displaying => f.write_str("displaying"),
            Self::Fault => f.write_str("fault"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for VmsStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "blank" => Ok(Self::Blank),
            "displaying" => Ok(Self::Displaying),
            "fault" => Ok(Self::Fault),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VmsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Variable message sign type definitions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC VMS Type\","]
#[doc = "  \"description\": \"Variable message sign type definitions.\","]
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
#[doc = "      \"description\": \"URL to an external specification document.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this VMS type.\","]
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
pub struct VmsType {
    #[doc = "Human-readable description."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Additional notes."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "URL to an external specification document."]
    #[serde(
        rename = "specificationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_link: ::std::option::Option<VmsTypeSpecificationLink>,
    #[doc = "Identifier of this VMS type."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl VmsType {
    pub fn builder() -> builder::VmsType {
        Default::default()
    }
}
#[doc = "URL to an external specification document."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL to an external specification document.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VmsTypeSpecificationLink(::std::string::String);
impl ::std::ops::Deref for VmsTypeSpecificationLink {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VmsTypeSpecificationLink> for ::std::string::String {
    fn from(value: VmsTypeSpecificationLink) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VmsTypeSpecificationLink {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VmsTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmsTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmsTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VmsTypeSpecificationLink {
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
        clear_sign_command: ::std::result::Result<
            ::std::option::Option<super::ClearSignCommand>,
            ::std::string::String,
        >,
        command_response_status: ::std::result::Result<
            ::std::option::Option<super::CommandResponseStatus>,
            ::std::string::String,
        >,
        coordinates:
            ::std::result::Result<::std::option::Option<super::Coordinates>, ::std::string::String>,
        display_graphic_command: ::std::result::Result<
            ::std::option::Option<super::DisplayGraphicCommand>,
            ::std::string::String,
        >,
        display_message_command: ::std::result::Result<
            ::std::option::Option<super::DisplayMessageCommand>,
            ::std::string::String,
        >,
        fault_status:
            ::std::result::Result<::std::option::Option<super::FaultStatus>, ::std::string::String>,
        point: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        set_brightness_command: ::std::result::Result<
            ::std::option::Option<super::SetBrightnessCommand>,
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
        vms_colour_capability: ::std::result::Result<
            ::std::option::Option<super::VmsColourCapability>,
            ::std::string::String,
        >,
        vms_command:
            ::std::result::Result<::std::option::Option<super::VmsCommand>, ::std::string::String>,
        vms_command_response: ::std::result::Result<
            ::std::option::Option<super::VmsCommandResponse>,
            ::std::string::String,
        >,
        vms_configuration: ::std::result::Result<
            ::std::option::Option<super::VmsConfiguration>,
            ::std::string::String,
        >,
        vms_definition: ::std::result::Result<
            ::std::option::Option<super::VmsDefinition>,
            ::std::string::String,
        >,
        vms_dynamic:
            ::std::result::Result<::std::option::Option<super::VmsDynamic>, ::std::string::String>,
        vms_fault:
            ::std::result::Result<::std::option::Option<super::VmsFault>, ::std::string::String>,
        vms_history_entry: ::std::result::Result<
            ::std::option::Option<super::VmsHistoryEntry>,
            ::std::string::String,
        >,
        vms_status:
            ::std::result::Result<::std::option::Option<super::VmsStatus>, ::std::string::String>,
        vms_type:
            ::std::result::Result<::std::option::Option<super::VmsType>, ::std::string::String>,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                acknowledgement_state: Ok(Default::default()),
                clear_sign_command: Ok(Default::default()),
                command_response_status: Ok(Default::default()),
                coordinates: Ok(Default::default()),
                display_graphic_command: Ok(Default::default()),
                display_message_command: Ok(Default::default()),
                fault_status: Ok(Default::default()),
                point: Ok(Default::default()),
                quality: Ok(Default::default()),
                set_brightness_command: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
                vms_colour_capability: Ok(Default::default()),
                vms_command: Ok(Default::default()),
                vms_command_response: Ok(Default::default()),
                vms_configuration: Ok(Default::default()),
                vms_definition: Ok(Default::default()),
                vms_dynamic: Ok(Default::default()),
                vms_fault: Ok(Default::default()),
                vms_history_entry: Ok(Default::default()),
                vms_status: Ok(Default::default()),
                vms_type: Ok(Default::default()),
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
        pub fn clear_sign_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClearSignCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.clear_sign_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for clear_sign_command: {e}")
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
        pub fn display_graphic_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DisplayGraphicCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_graphic_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for display_graphic_command: {e}")
            });
            self
        }
        pub fn display_message_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DisplayMessageCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_message_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for display_message_command: {e}")
            });
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
        pub fn set_brightness_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SetBrightnessCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.set_brightness_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for set_brightness_command: {e}")
            });
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
        pub fn vms_colour_capability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsColourCapability>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_colour_capability = value.try_into().map_err(|e| {
                format!("error converting supplied value for vms_colour_capability: {e}")
            });
            self
        }
        pub fn vms_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_command: {e}"));
            self
        }
        pub fn vms_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for vms_command_response: {e}")
            });
            self
        }
        pub fn vms_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_configuration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_configuration: {e}"));
            self
        }
        pub fn vms_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_definition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_definition: {e}"));
            self
        }
        pub fn vms_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_dynamic: {e}"));
            self
        }
        pub fn vms_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsFault>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_fault = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_fault: {e}"));
            self
        }
        pub fn vms_history_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsHistoryEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_history_entry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_history_entry: {e}"));
            self
        }
        pub fn vms_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_status: {e}"));
            self
        }
        pub fn vms_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsType>>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_type: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_state: value.acknowledgement_state?,
                clear_sign_command: value.clear_sign_command?,
                command_response_status: value.command_response_status?,
                coordinates: value.coordinates?,
                display_graphic_command: value.display_graphic_command?,
                display_message_command: value.display_message_command?,
                fault_status: value.fault_status?,
                point: value.point?,
                quality: value.quality?,
                set_brightness_command: value.set_brightness_command?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
                vms_colour_capability: value.vms_colour_capability?,
                vms_command: value.vms_command?,
                vms_command_response: value.vms_command_response?,
                vms_configuration: value.vms_configuration?,
                vms_definition: value.vms_definition?,
                vms_dynamic: value.vms_dynamic?,
                vms_fault: value.vms_fault?,
                vms_history_entry: value.vms_history_entry?,
                vms_status: value.vms_status?,
                vms_type: value.vms_type?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                acknowledgement_state: Ok(value.acknowledgement_state),
                clear_sign_command: Ok(value.clear_sign_command),
                command_response_status: Ok(value.command_response_status),
                coordinates: Ok(value.coordinates),
                display_graphic_command: Ok(value.display_graphic_command),
                display_message_command: Ok(value.display_message_command),
                fault_status: Ok(value.fault_status),
                point: Ok(value.point),
                quality: Ok(value.quality),
                set_brightness_command: Ok(value.set_brightness_command),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
                vms_colour_capability: Ok(value.vms_colour_capability),
                vms_command: Ok(value.vms_command),
                vms_command_response: Ok(value.vms_command_response),
                vms_configuration: Ok(value.vms_configuration),
                vms_definition: Ok(value.vms_definition),
                vms_dynamic: Ok(value.vms_dynamic),
                vms_fault: Ok(value.vms_fault),
                vms_history_entry: Ok(value.vms_history_entry),
                vms_status: Ok(value.vms_status),
                vms_type: Ok(value.vms_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClearSignCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::ClearSignCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for ClearSignCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl ClearSignCommand {
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
            T: ::std::convert::TryInto<super::ClearSignCommandKind>,
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ClearSignCommand> for super::ClearSignCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClearSignCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::ClearSignCommand> for ClearSignCommand {
        fn from(value: super::ClearSignCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                vms_id: Ok(value.vms_id),
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
    pub struct DisplayGraphicCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        display_duration_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        graphic_url:
            ::std::result::Result<super::DisplayGraphicCommandGraphicUrl, ::std::string::String>,
        kind: ::std::result::Result<super::DisplayGraphicCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for DisplayGraphicCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                display_duration_seconds: Ok(Default::default()),
                extensions: Ok(Default::default()),
                graphic_url: Err("no value supplied for graphic_url".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl DisplayGraphicCommand {
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
        pub fn display_duration_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_duration_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for display_duration_seconds: {e}")
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
        pub fn graphic_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DisplayGraphicCommandGraphicUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.graphic_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for graphic_url: {e}"));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DisplayGraphicCommandKind>,
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DisplayGraphicCommand> for super::DisplayGraphicCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DisplayGraphicCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                display_duration_seconds: value.display_duration_seconds?,
                extensions: value.extensions?,
                graphic_url: value.graphic_url?,
                kind: value.kind?,
                request_time: value.request_time?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::DisplayGraphicCommand> for DisplayGraphicCommand {
        fn from(value: super::DisplayGraphicCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                display_duration_seconds: Ok(value.display_duration_seconds),
                extensions: Ok(value.extensions),
                graphic_url: Ok(value.graphic_url),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DisplayMessageCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        display_duration_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::DisplayMessageCommandKind, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
        message_lines:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for DisplayMessageCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                display_duration_seconds: Ok(Default::default()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                message: Err("no value supplied for message".to_string()),
                message_lines: Ok(Default::default()),
                request_time: Err("no value supplied for request_time".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl DisplayMessageCommand {
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
        pub fn display_duration_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.display_duration_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for display_duration_seconds: {e}")
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
            T: ::std::convert::TryInto<super::DisplayMessageCommandKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn message_lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message_lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_lines: {e}"));
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DisplayMessageCommand> for super::DisplayMessageCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DisplayMessageCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                display_duration_seconds: value.display_duration_seconds?,
                extensions: value.extensions?,
                kind: value.kind?,
                message: value.message?,
                message_lines: value.message_lines?,
                request_time: value.request_time?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::DisplayMessageCommand> for DisplayMessageCommand {
        fn from(value: super::DisplayMessageCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                display_duration_seconds: Ok(value.display_duration_seconds),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                message: Ok(value.message),
                message_lines: Ok(value.message_lines),
                request_time: Ok(value.request_time),
                vms_id: Ok(value.vms_id),
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
    pub struct SetBrightnessCommand {
        brightness_percent: ::std::result::Result<f64, ::std::string::String>,
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SetBrightnessCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for SetBrightnessCommand {
        fn default() -> Self {
            Self {
                brightness_percent: Err("no value supplied for brightness_percent".to_string()),
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl SetBrightnessCommand {
        pub fn brightness_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.brightness_percent = value.try_into().map_err(|e| {
                format!("error converting supplied value for brightness_percent: {e}")
            });
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
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SetBrightnessCommandKind>,
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetBrightnessCommand> for super::SetBrightnessCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetBrightnessCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                brightness_percent: value.brightness_percent?,
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::SetBrightnessCommand> for SetBrightnessCommand {
        fn from(value: super::SetBrightnessCommand) -> Self {
            Self {
                brightness_percent: Ok(value.brightness_percent),
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                vms_id: Ok(value.vms_id),
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
    pub struct VmsCommandResponse {
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
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsCommandResponse {
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
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsCommandResponse {
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsCommandResponse> for super::VmsCommandResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VmsCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                status: value.status?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsCommandResponse> for VmsCommandResponse {
        fn from(value: super::VmsCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                status: Ok(value.status),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsConfiguration {
        auto_brightness: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        default_brightness_percent:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        default_display_time_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        scroll_speed: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsConfiguration {
        fn default() -> Self {
            Self {
                auto_brightness: Ok(Default::default()),
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                default_brightness_percent: Ok(Default::default()),
                default_display_time_seconds: Ok(Default::default()),
                extensions: Ok(Default::default()),
                scroll_speed: Ok(Default::default()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsConfiguration {
        pub fn auto_brightness<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.auto_brightness = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auto_brightness: {e}"));
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
        pub fn default_brightness_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_brightness_percent = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_brightness_percent: {e}")
            });
            self
        }
        pub fn default_display_time_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_display_time_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_display_time_seconds: {e}")
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
        pub fn scroll_speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scroll_speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scroll_speed: {e}"));
            self
        }
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsConfiguration> for super::VmsConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VmsConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                auto_brightness: value.auto_brightness?,
                configuration_date: value.configuration_date?,
                default_brightness_percent: value.default_brightness_percent?,
                default_display_time_seconds: value.default_display_time_seconds?,
                extensions: value.extensions?,
                scroll_speed: value.scroll_speed?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsConfiguration> for VmsConfiguration {
        fn from(value: super::VmsConfiguration) -> Self {
            Self {
                auto_brightness: Ok(value.auto_brightness),
                configuration_date: Ok(value.configuration_date),
                default_brightness_percent: Ok(value.default_brightness_percent),
                default_display_time_seconds: Ok(value.default_display_time_seconds),
                extensions: Ok(value.extensions),
                scroll_speed: Ok(value.scroll_speed),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsDefinition {
        characters_per_line: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        colour_capability: ::std::result::Result<
            ::std::option::Option<super::VmsColourCapability>,
            ::std::string::String,
        >,
        creation_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        link_distance_metres:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        long_description: ::std::result::Result<
            ::std::option::Option<super::VmsDefinitionLongDescription>,
            ::std::string::String,
        >,
        network_link_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        network_path_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        number_of_lines: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
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
            ::std::option::Option<super::VmsDefinitionShortDescription>,
            ::std::string::String,
        >,
        sign_height: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        sign_width: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        supports_graphics:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        transport_link_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsDefinition {
        fn default() -> Self {
            Self {
                characters_per_line: Ok(Default::default()),
                colour_capability: Ok(Default::default()),
                creation_date: Err("no value supplied for creation_date".to_string()),
                extensions: Ok(Default::default()),
                link_distance_metres: Ok(Default::default()),
                location: Ok(Default::default()),
                long_description: Ok(Default::default()),
                network_link_id: Ok(Default::default()),
                network_path_id: Ok(Default::default()),
                number_of_lines: Ok(Default::default()),
                originating_object_id: Ok(Default::default()),
                originating_participant_id: Ok(Default::default()),
                owner_operator: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                short_description: Ok(Default::default()),
                sign_height: Ok(Default::default()),
                sign_width: Ok(Default::default()),
                supports_graphics: Ok(Default::default()),
                transport_link_id: Ok(Default::default()),
                type_id: Err("no value supplied for type_id".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsDefinition {
        pub fn characters_per_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.characters_per_line = value.try_into().map_err(|e| {
                format!("error converting supplied value for characters_per_line: {e}")
            });
            self
        }
        pub fn colour_capability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsColourCapability>>,
            T::Error: ::std::fmt::Display,
        {
            self.colour_capability = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for colour_capability: {e}"));
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
        pub fn link_distance_metres<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_distance_metres = value.try_into().map_err(|e| {
                format!("error converting supplied value for link_distance_metres: {e}")
            });
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
            T: ::std::convert::TryInto<::std::option::Option<super::VmsDefinitionLongDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.long_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for long_description: {e}"));
            self
        }
        pub fn network_link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_link_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_link_id: {e}"));
            self
        }
        pub fn network_path_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_path_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_path_id: {e}"));
            self
        }
        pub fn number_of_lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.number_of_lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_of_lines: {e}"));
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
            T: ::std::convert::TryInto<::std::option::Option<super::VmsDefinitionShortDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.short_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_description: {e}"));
            self
        }
        pub fn sign_height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sign_height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sign_height: {e}"));
            self
        }
        pub fn sign_width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sign_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sign_width: {e}"));
            self
        }
        pub fn supports_graphics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.supports_graphics = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supports_graphics: {e}"));
            self
        }
        pub fn transport_link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ObjectId>>,
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsDefinition> for super::VmsDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VmsDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                characters_per_line: value.characters_per_line?,
                colour_capability: value.colour_capability?,
                creation_date: value.creation_date?,
                extensions: value.extensions?,
                link_distance_metres: value.link_distance_metres?,
                location: value.location?,
                long_description: value.long_description?,
                network_link_id: value.network_link_id?,
                network_path_id: value.network_path_id?,
                number_of_lines: value.number_of_lines?,
                originating_object_id: value.originating_object_id?,
                originating_participant_id: value.originating_participant_id?,
                owner_operator: value.owner_operator?,
                participant_id: value.participant_id?,
                short_description: value.short_description?,
                sign_height: value.sign_height?,
                sign_width: value.sign_width?,
                supports_graphics: value.supports_graphics?,
                transport_link_id: value.transport_link_id?,
                type_id: value.type_id?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsDefinition> for VmsDefinition {
        fn from(value: super::VmsDefinition) -> Self {
            Self {
                characters_per_line: Ok(value.characters_per_line),
                colour_capability: Ok(value.colour_capability),
                creation_date: Ok(value.creation_date),
                extensions: Ok(value.extensions),
                link_distance_metres: Ok(value.link_distance_metres),
                location: Ok(value.location),
                long_description: Ok(value.long_description),
                network_link_id: Ok(value.network_link_id),
                network_path_id: Ok(value.network_path_id),
                number_of_lines: Ok(value.number_of_lines),
                originating_object_id: Ok(value.originating_object_id),
                originating_participant_id: Ok(value.originating_participant_id),
                owner_operator: Ok(value.owner_operator),
                participant_id: Ok(value.participant_id),
                short_description: Ok(value.short_description),
                sign_height: Ok(value.sign_height),
                sign_width: Ok(value.sign_width),
                supports_graphics: Ok(value.supports_graphics),
                transport_link_id: Ok(value.transport_link_id),
                type_id: Ok(value.type_id),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsDynamic {
        brightness_percent:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        current_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        graphic_url: ::std::result::Result<
            ::std::option::Option<super::VmsDynamicGraphicUrl>,
            ::std::string::String,
        >,
        message_expiry_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        message_lines:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        message_set_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        status:
            ::std::result::Result<::std::option::Option<super::VmsStatus>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsDynamic {
        fn default() -> Self {
            Self {
                brightness_percent: Ok(Default::default()),
                current_message: Ok(Default::default()),
                extensions: Ok(Default::default()),
                graphic_url: Ok(Default::default()),
                message_expiry_time: Ok(Default::default()),
                message_lines: Ok(Default::default()),
                message_set_time: Ok(Default::default()),
                quality: Ok(Default::default()),
                status: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsDynamic {
        pub fn brightness_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.brightness_percent = value.try_into().map_err(|e| {
                format!("error converting supplied value for brightness_percent: {e}")
            });
            self
        }
        pub fn current_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.current_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for current_message: {e}"));
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
        pub fn graphic_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsDynamicGraphicUrl>>,
            T::Error: ::std::fmt::Display,
        {
            self.graphic_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for graphic_url: {e}"));
            self
        }
        pub fn message_expiry_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_expiry_time = value.try_into().map_err(|e| {
                format!("error converting supplied value for message_expiry_time: {e}")
            });
            self
        }
        pub fn message_lines<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message_lines = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_lines: {e}"));
            self
        }
        pub fn message_set_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_set_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_set_time: {e}"));
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
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VmsStatus>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsDynamic> for super::VmsDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VmsDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                brightness_percent: value.brightness_percent?,
                current_message: value.current_message?,
                extensions: value.extensions?,
                graphic_url: value.graphic_url?,
                message_expiry_time: value.message_expiry_time?,
                message_lines: value.message_lines?,
                message_set_time: value.message_set_time?,
                quality: value.quality?,
                status: value.status?,
                timestamp: value.timestamp?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsDynamic> for VmsDynamic {
        fn from(value: super::VmsDynamic) -> Self {
            Self {
                brightness_percent: Ok(value.brightness_percent),
                current_message: Ok(value.current_message),
                extensions: Ok(value.extensions),
                graphic_url: Ok(value.graphic_url),
                message_expiry_time: Ok(value.message_expiry_time),
                message_lines: Ok(value.message_lines),
                message_set_time: Ok(value.message_set_time),
                quality: Ok(value.quality),
                status: Ok(value.status),
                timestamp: Ok(value.timestamp),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsFault {
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
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsFault {
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
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsFault {
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsFault> for super::VmsFault {
        type Error = super::error::ConversionError;
        fn try_from(value: VmsFault) -> ::std::result::Result<Self, super::error::ConversionError> {
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
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsFault> for VmsFault {
        fn from(value: super::VmsFault) -> Self {
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
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsHistoryEntry {
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
        vms_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsHistoryEntry {
        fn default() -> Self {
            Self {
                event_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                log_date: Err("no value supplied for log_date".to_string()),
                log_entry: Err("no value supplied for log_entry".to_string()),
                log_id: Err("no value supplied for log_id".to_string()),
                log_user: Ok(Default::default()),
                vms_id: Err("no value supplied for vms_id".to_string()),
            }
        }
    }
    impl VmsHistoryEntry {
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
        pub fn vms_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.vms_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vms_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VmsHistoryEntry> for super::VmsHistoryEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VmsHistoryEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
                extensions: value.extensions?,
                log_date: value.log_date?,
                log_entry: value.log_entry?,
                log_id: value.log_id?,
                log_user: value.log_user?,
                vms_id: value.vms_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsHistoryEntry> for VmsHistoryEntry {
        fn from(value: super::VmsHistoryEntry) -> Self {
            Self {
                event_type: Ok(value.event_type),
                extensions: Ok(value.extensions),
                log_date: Ok(value.log_date),
                log_entry: Ok(value.log_entry),
                log_id: Ok(value.log_id),
                log_user: Ok(value.log_user),
                vms_id: Ok(value.vms_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VmsType {
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
            ::std::option::Option<super::VmsTypeSpecificationLink>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for VmsType {
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
    impl VmsType {
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
            T: ::std::convert::TryInto<::std::option::Option<super::VmsTypeSpecificationLink>>,
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
    impl ::std::convert::TryFrom<VmsType> for super::VmsType {
        type Error = super::error::ConversionError;
        fn try_from(value: VmsType) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                notes: value.notes?,
                specification_link: value.specification_link?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::VmsType> for VmsType {
        fn from(value: super::VmsType) -> Self {
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
