/*
 * File:        output/rust/sdk/src/detector.rs
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
#[doc = "    \"CalibrateDetectorCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/CalibrateDetectorCommand\""]
#[doc = "    },"]
#[doc = "    \"CommandResponseStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/CommandResponseStatus\""]
#[doc = "    },"]
#[doc = "    \"Coordinates\": {"]
#[doc = "      \"$ref\": \"#/$defs/Coordinates\""]
#[doc = "    },"]
#[doc = "    \"DetectorCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorCommand\""]
#[doc = "    },"]
#[doc = "    \"DetectorCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"DetectorConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorConfiguration\""]
#[doc = "    },"]
#[doc = "    \"DetectorDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorDefinition\""]
#[doc = "    },"]
#[doc = "    \"DetectorFault\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorFault\""]
#[doc = "    },"]
#[doc = "    \"DetectorHistoryEntry\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorHistoryEntry\""]
#[doc = "    },"]
#[doc = "    \"DetectorMeasurement\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorMeasurement\""]
#[doc = "    },"]
#[doc = "    \"DetectorType\": {"]
#[doc = "      \"$ref\": \"#/$defs/DetectorType\""]
#[doc = "    },"]
#[doc = "    \"FaultStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultStatus\""]
#[doc = "    },"]
#[doc = "    \"FlowDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/FlowDynamic\""]
#[doc = "    },"]
#[doc = "    \"HeadwayDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/HeadwayDynamic\""]
#[doc = "    },"]
#[doc = "    \"OccupancyDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/OccupancyDynamic\""]
#[doc = "    },"]
#[doc = "    \"Point\": {"]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"Quality\": {"]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"QueueDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/QueueDynamic\""]
#[doc = "    },"]
#[doc = "    \"ResetDetectorCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/ResetDetectorCommand\""]
#[doc = "    },"]
#[doc = "    \"SetThresholdDetectorCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/SetThresholdDetectorCommand\""]
#[doc = "    },"]
#[doc = "    \"SpeedDynamic\": {"]
#[doc = "      \"$ref\": \"#/$defs/SpeedDynamic\""]
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
        rename = "AcknowledgementState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub acknowledgement_state: ::std::option::Option<AcknowledgementState>,
    #[serde(
        rename = "CalibrateDetectorCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub calibrate_detector_command: ::std::option::Option<CalibrateDetectorCommand>,
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
        rename = "DetectorCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_command: ::std::option::Option<DetectorCommand>,
    #[serde(
        rename = "DetectorCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_command_response: ::std::option::Option<DetectorCommandResponse>,
    #[serde(
        rename = "DetectorConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_configuration: ::std::option::Option<DetectorConfiguration>,
    #[serde(
        rename = "DetectorDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_definition: ::std::option::Option<DetectorDefinition>,
    #[serde(
        rename = "DetectorFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_fault: ::std::option::Option<DetectorFault>,
    #[serde(
        rename = "DetectorHistoryEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_history_entry: ::std::option::Option<DetectorHistoryEntry>,
    #[serde(
        rename = "DetectorMeasurement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_measurement: ::std::option::Option<DetectorMeasurement>,
    #[serde(
        rename = "DetectorType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub detector_type: ::std::option::Option<DetectorType>,
    #[serde(
        rename = "FaultStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_status: ::std::option::Option<FaultStatus>,
    #[serde(
        rename = "FlowDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub flow_dynamic: ::std::option::Option<FlowDynamic>,
    #[serde(
        rename = "HeadwayDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub headway_dynamic: ::std::option::Option<HeadwayDynamic>,
    #[serde(
        rename = "OccupancyDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub occupancy_dynamic: ::std::option::Option<OccupancyDynamic>,
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
        rename = "QueueDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub queue_dynamic: ::std::option::Option<QueueDynamic>,
    #[serde(
        rename = "ResetDetectorCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reset_detector_command: ::std::option::Option<ResetDetectorCommand>,
    #[serde(
        rename = "SetThresholdDetectorCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub set_threshold_detector_command: ::std::option::Option<SetThresholdDetectorCommand>,
    #[serde(
        rename = "SpeedDynamic",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_dynamic: ::std::option::Option<SpeedDynamic>,
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
            acknowledgement_state: Default::default(),
            calibrate_detector_command: Default::default(),
            command_response_status: Default::default(),
            coordinates: Default::default(),
            detector_command: Default::default(),
            detector_command_response: Default::default(),
            detector_configuration: Default::default(),
            detector_definition: Default::default(),
            detector_fault: Default::default(),
            detector_history_entry: Default::default(),
            detector_measurement: Default::default(),
            detector_type: Default::default(),
            fault_status: Default::default(),
            flow_dynamic: Default::default(),
            headway_dynamic: Default::default(),
            occupancy_dynamic: Default::default(),
            point: Default::default(),
            quality: Default::default(),
            queue_dynamic: Default::default(),
            reset_detector_command: Default::default(),
            set_threshold_detector_command: Default::default(),
            speed_dynamic: Default::default(),
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
#[doc = "Trigger a calibration cycle on the detector."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Trigger a calibration cycle on the detector.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector receiving the command.\","]
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
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"calibrate\\\" selects the CalibrateDetectorCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"calibrate\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CalibrateDetectorCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Identifier of the detector receiving the command."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"calibrate\" selects the CalibrateDetectorCommand payload; it is not free-text command content."]
    pub kind: CalibrateDetectorCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl CalibrateDetectorCommand {
    pub fn builder() -> builder::CalibrateDetectorCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"calibrate\" selects the CalibrateDetectorCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"calibrate\\\" selects the CalibrateDetectorCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"calibrate\""]
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
pub enum CalibrateDetectorCommandKind {
    #[serde(rename = "calibrate")]
    Calibrate,
}
impl ::std::fmt::Display for CalibrateDetectorCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Calibrate => f.write_str("calibrate"),
        }
    }
}
impl ::std::str::FromStr for CalibrateDetectorCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "calibrate" => Ok(Self::Calibrate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CalibrateDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CalibrateDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CalibrateDetectorCommandKind {
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
#[doc = "Detector command payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Command\","]
#[doc = "  \"description\": \"Detector command payloads.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ResetDetectorCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/CalibrateDetectorCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SetThresholdDetectorCommand\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"calibrate\": \"#/$defs/CalibrateDetectorCommand\","]
#[doc = "      \"reset\": \"#/$defs/ResetDetectorCommand\","]
#[doc = "      \"setThreshold\": \"#/$defs/SetThresholdDetectorCommand\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DetectorCommand {
    ResetDetectorCommand(ResetDetectorCommand),
    CalibrateDetectorCommand(CalibrateDetectorCommand),
    SetThresholdDetectorCommand(SetThresholdDetectorCommand),
}
impl ::std::convert::From<ResetDetectorCommand> for DetectorCommand {
    fn from(value: ResetDetectorCommand) -> Self {
        Self::ResetDetectorCommand(value)
    }
}
impl ::std::convert::From<CalibrateDetectorCommand> for DetectorCommand {
    fn from(value: CalibrateDetectorCommand) -> Self {
        Self::CalibrateDetectorCommand(value)
    }
}
impl ::std::convert::From<SetThresholdDetectorCommand> for DetectorCommand {
    fn from(value: SetThresholdDetectorCommand) -> Self {
        Self::SetThresholdDetectorCommand(value)
    }
}
#[doc = "Detector command response payloads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Command Response\","]
#[doc = "  \"description\": \"Detector command response payloads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\""]
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
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector the command targeted.\","]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetectorCommandResponse {
    #[doc = "Machine-readable response code for programmatic handling."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    #[doc = "Correlation identifier copied verbatim from the originating command request. Not an identifier of the command itself."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Identifier of the detector the command targeted."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
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
}
impl DetectorCommandResponse {
    pub fn builder() -> builder::DetectorCommandResponse {
        Default::default()
    }
}
#[doc = "Detector operational configuration."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Configuration\","]
#[doc = "  \"description\": \"Detector operational configuration.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"configurationDate\","]
#[doc = "    \"detectorId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration record became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this configuration applies to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"thresholdDown\": {"]
#[doc = "      \"description\": \"Downward threshold for alerting.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"thresholdUp\": {"]
#[doc = "      \"description\": \"Upward threshold for alerting (e.g. occupancy threshold).\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetectorConfiguration {
    #[doc = "When this configuration record became effective."]
    #[serde(rename = "configurationDate")]
    pub configuration_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the detector this configuration applies to."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Downward threshold for alerting."]
    #[serde(
        rename = "thresholdDown",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub threshold_down: ::std::option::Option<u64>,
    #[doc = "Upward threshold for alerting (e.g. occupancy threshold)."]
    #[serde(
        rename = "thresholdUp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub threshold_up: ::std::option::Option<u64>,
}
impl DetectorConfiguration {
    pub fn builder() -> builder::DetectorConfiguration {
        Default::default()
    }
}
#[doc = "Detector equipment details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Definition\","]
#[doc = "  \"description\": \"Detector equipment details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When this definition record was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this detector within its owning system.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"linkDistanceMetres\": {"]
#[doc = "      \"description\": \"Distance along that transport link, in metres. Optional.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Physical location of the detector. Optional.\","]
#[doc = "      \"$ref\": \"#/$defs/Point\""]
#[doc = "    },"]
#[doc = "    \"longDescription\": {"]
#[doc = "      \"description\": \"Longer description (context, installation notes, etc.).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 2000,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"networkLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Network link the detector stands on. At most one of this and networkPathId. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"networkPathId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Network path the detector stands on. At most one of this and networkLinkId. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
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
#[doc = "      \"description\": \"System that owns this detector definition. Often implicit from context.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"description\": \"Short human-readable name (e.g. 'A1 southbound').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"transportLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Transport link this detector monitors or sits on. Optional.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Unique identifier for a detector type (loop, radar, ANPR, ...).\","]
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
pub struct DetectorDefinition {
    #[doc = "When this definition record was created."]
    #[serde(rename = "creationDate")]
    pub creation_date: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Unique identifier of this detector within its owning system."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance along that transport link, in metres. Optional."]
    #[serde(
        rename = "linkDistanceMetres",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_distance_metres: ::std::option::Option<f64>,
    #[doc = "Physical location of the detector. Optional."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<Point>,
    #[doc = "Longer description (context, installation notes, etc.)."]
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<DetectorDefinitionLongDescription>,
    #[doc = "Network link the detector stands on. At most one of this and networkPathId. Optional."]
    #[serde(
        rename = "networkLinkId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_link_id: ::std::option::Option<ObjectId>,
    #[doc = "Network path the detector stands on. At most one of this and networkLinkId. Optional."]
    #[serde(
        rename = "networkPathId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_path_id: ::std::option::Option<ObjectId>,
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
    #[doc = "System that owns this detector definition. Often implicit from context."]
    #[serde(
        rename = "participantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Short human-readable name (e.g. 'A1 southbound')."]
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<DetectorDefinitionShortDescription>,
    #[doc = "Transport link this detector monitors or sits on. Optional."]
    #[serde(
        rename = "transportLinkId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub transport_link_id: ::std::option::Option<ObjectId>,
    #[doc = "Unique identifier for a detector type (loop, radar, ANPR, ...)."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl DetectorDefinition {
    pub fn builder() -> builder::DetectorDefinition {
        Default::default()
    }
}
#[doc = "Longer description (context, installation notes, etc.)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Longer description (context, installation notes, etc.).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 2000,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DetectorDefinitionLongDescription(::std::string::String);
impl ::std::ops::Deref for DetectorDefinitionLongDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DetectorDefinitionLongDescription> for ::std::string::String {
    fn from(value: DetectorDefinitionLongDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DetectorDefinitionLongDescription {
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
impl ::std::convert::TryFrom<&str> for DetectorDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DetectorDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DetectorDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DetectorDefinitionLongDescription {
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
#[doc = "Short human-readable name (e.g. 'A1 southbound')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short human-readable name (e.g. 'A1 southbound').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DetectorDefinitionShortDescription(::std::string::String);
impl ::std::ops::Deref for DetectorDefinitionShortDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DetectorDefinitionShortDescription> for ::std::string::String {
    fn from(value: DetectorDefinitionShortDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DetectorDefinitionShortDescription {
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
impl ::std::convert::TryFrom<&str> for DetectorDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DetectorDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DetectorDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DetectorDefinitionShortDescription {
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
#[doc = "Detector fault reports."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Fault\","]
#[doc = "  \"description\": \"Detector fault reports.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"creationDate\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"faultId\","]
#[doc = "    \"faultTypeId\""]
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
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this fault is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
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
#[doc = "      \"description\": \"Reference to a DetectorFaultType entry in the fault catalogue.\","]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetectorFault {
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
    #[doc = "Identifier of the detector this fault is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
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
    #[doc = "Reference to a DetectorFaultType entry in the fault catalogue."]
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
}
impl DetectorFault {
    pub fn builder() -> builder::DetectorFault {
        Default::default()
    }
}
#[doc = "Detector history entries."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector History\","]
#[doc = "  \"description\": \"Detector history entries.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"logDate\","]
#[doc = "    \"logEntry\","]
#[doc = "    \"logId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this log entry is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"eventType\": {"]
#[doc = "      \"description\": \"Category or type of the history event (e.g. 'recalibration', 'threshold_change').\","]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetectorHistoryEntry {
    #[doc = "Identifier of the detector this log entry is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Category or type of the history event (e.g. 'recalibration', 'threshold_change')."]
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
}
impl DetectorHistoryEntry {
    pub fn builder() -> builder::DetectorHistoryEntry {
        Default::default()
    }
}
#[doc = "Detector dynamic measurement data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Measurement\","]
#[doc = "  \"description\": \"Detector dynamic measurement data.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/FlowDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/OccupancyDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/QueueDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SpeedDynamic\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/HeadwayDynamic\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"flow\": \"#/$defs/FlowDynamic\","]
#[doc = "      \"headway\": \"#/$defs/HeadwayDynamic\","]
#[doc = "      \"occupancy\": \"#/$defs/OccupancyDynamic\","]
#[doc = "      \"queue\": \"#/$defs/QueueDynamic\","]
#[doc = "      \"speed\": \"#/$defs/SpeedDynamic\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DetectorMeasurement {
    FlowDynamic(FlowDynamic),
    OccupancyDynamic(OccupancyDynamic),
    QueueDynamic(QueueDynamic),
    SpeedDynamic(SpeedDynamic),
    HeadwayDynamic(HeadwayDynamic),
}
impl ::std::convert::From<FlowDynamic> for DetectorMeasurement {
    fn from(value: FlowDynamic) -> Self {
        Self::FlowDynamic(value)
    }
}
impl ::std::convert::From<OccupancyDynamic> for DetectorMeasurement {
    fn from(value: OccupancyDynamic) -> Self {
        Self::OccupancyDynamic(value)
    }
}
impl ::std::convert::From<QueueDynamic> for DetectorMeasurement {
    fn from(value: QueueDynamic) -> Self {
        Self::QueueDynamic(value)
    }
}
impl ::std::convert::From<SpeedDynamic> for DetectorMeasurement {
    fn from(value: SpeedDynamic) -> Self {
        Self::SpeedDynamic(value)
    }
}
impl ::std::convert::From<HeadwayDynamic> for DetectorMeasurement {
    fn from(value: HeadwayDynamic) -> Self {
        Self::HeadwayDynamic(value)
    }
}
#[doc = "Detector type definitions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Type\","]
#[doc = "  \"description\": \"Detector type definitions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description of the detector kind.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"notes\": {"]
#[doc = "      \"description\": \"Additional notes, e.g. deployment details.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"specificationLink\": {"]
#[doc = "      \"description\": \"URL to an external specification document for this detector type.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this detector type.\","]
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
pub struct DetectorType {
    #[doc = "Human-readable description of the detector kind."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Additional notes, e.g. deployment details."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "URL to an external specification document for this detector type."]
    #[serde(
        rename = "specificationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_link: ::std::option::Option<DetectorTypeSpecificationLink>,
    #[doc = "Identifier of this detector type."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl DetectorType {
    pub fn builder() -> builder::DetectorType {
        Default::default()
    }
}
#[doc = "URL to an external specification document for this detector type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL to an external specification document for this detector type.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DetectorTypeSpecificationLink(::std::string::String);
impl ::std::ops::Deref for DetectorTypeSpecificationLink {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DetectorTypeSpecificationLink> for ::std::string::String {
    fn from(value: DetectorTypeSpecificationLink) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DetectorTypeSpecificationLink {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DetectorTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DetectorTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DetectorTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DetectorTypeSpecificationLink {
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
#[doc = "Vehicle flow count over a measurement period."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vehicle flow count over a measurement period.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"classCounts\": {"]
#[doc = "      \"description\": \"Per-class vehicle counts in positional order — index 0 is Class1, index 7 is Class8. May be shorter if trailing classes are absent. A null entry means that class was not reported, as distinct from a count of zero.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"type\": \"integer\","]
#[doc = "            \"minimum\": 0.0"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"null\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 8"]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"flow\\\" selects the FlowDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"flow\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"periodSeconds\": {"]
#[doc = "      \"description\": \"Duration of the measurement period in seconds.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
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
#[doc = "    \"totalFlow\": {"]
#[doc = "      \"description\": \"Total vehicle count over the period. Omitted when only class counts are reported.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FlowDynamic {
    #[doc = "Per-class vehicle counts in positional order — index 0 is Class1, index 7 is Class8. May be shorter if trailing classes are absent. A null entry means that class was not reported, as distinct from a count of zero."]
    #[serde(
        rename = "classCounts",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub class_counts: ::std::vec::Vec<::std::option::Option<u64>>,
    #[doc = "Identifier of the detector this reading is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"flow\" selects the FlowDynamic payload; it is not free-text command content."]
    pub kind: FlowDynamicKind,
    #[doc = "Duration of the measurement period in seconds."]
    #[serde(
        rename = "periodSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub period_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Total vehicle count over the period. Omitted when only class counts are reported."]
    #[serde(
        rename = "totalFlow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub total_flow: ::std::option::Option<u64>,
}
impl FlowDynamic {
    pub fn builder() -> builder::FlowDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"flow\" selects the FlowDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"flow\\\" selects the FlowDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"flow\""]
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
pub enum FlowDynamicKind {
    #[serde(rename = "flow")]
    Flow,
}
impl ::std::fmt::Display for FlowDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Flow => f.write_str("flow"),
        }
    }
}
impl ::std::str::FromStr for FlowDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "flow" => Ok(Self::Flow),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FlowDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FlowDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FlowDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mean inter-vehicle headway over a measurement period."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Mean inter-vehicle headway over a measurement period.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"meanHeadwaySeconds\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"headway\\\" selects the HeadwayDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"headway\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"meanHeadwaySeconds\": {"]
#[doc = "      \"description\": \"Mean inter-vehicle headway in seconds.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"periodSeconds\": {"]
#[doc = "      \"description\": \"Duration of the measurement period in seconds.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct HeadwayDynamic {
    #[doc = "Identifier of the detector this reading is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"headway\" selects the HeadwayDynamic payload; it is not free-text command content."]
    pub kind: HeadwayDynamicKind,
    #[doc = "Mean inter-vehicle headway in seconds."]
    #[serde(rename = "meanHeadwaySeconds")]
    pub mean_headway_seconds: f64,
    #[doc = "Duration of the measurement period in seconds."]
    #[serde(
        rename = "periodSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub period_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl HeadwayDynamic {
    pub fn builder() -> builder::HeadwayDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"headway\" selects the HeadwayDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"headway\\\" selects the HeadwayDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"headway\""]
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
pub enum HeadwayDynamicKind {
    #[serde(rename = "headway")]
    Headway,
}
impl ::std::fmt::Display for HeadwayDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Headway => f.write_str("headway"),
        }
    }
}
impl ::std::str::FromStr for HeadwayDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "headway" => Ok(Self::Headway),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HeadwayDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HeadwayDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HeadwayDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the detector receiving the command."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Identifier of the detector receiving the command.\","]
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
#[doc = "Detector occupancy (percentage of period occupied)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Detector occupancy (percentage of period occupied).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"occupancyPercent\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"occupancy\\\" selects the OccupancyDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"occupancy\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"occupancyPercent\": {"]
#[doc = "      \"description\": \"Percentage occupancy over the period.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"periodSeconds\": {"]
#[doc = "      \"description\": \"Duration of the measurement period in seconds.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OccupancyDynamic {
    #[doc = "Identifier of the detector this reading is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"occupancy\" selects the OccupancyDynamic payload; it is not free-text command content."]
    pub kind: OccupancyDynamicKind,
    #[doc = "Percentage occupancy over the period."]
    #[serde(rename = "occupancyPercent")]
    pub occupancy_percent: f64,
    #[doc = "Duration of the measurement period in seconds."]
    #[serde(
        rename = "periodSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub period_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl OccupancyDynamic {
    pub fn builder() -> builder::OccupancyDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"occupancy\" selects the OccupancyDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"occupancy\\\" selects the OccupancyDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"occupancy\""]
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
pub enum OccupancyDynamicKind {
    #[serde(rename = "occupancy")]
    Occupancy,
}
impl ::std::fmt::Display for OccupancyDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Occupancy => f.write_str("occupancy"),
        }
    }
}
impl ::std::str::FromStr for OccupancyDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "occupancy" => Ok(Self::Occupancy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OccupancyDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OccupancyDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OccupancyDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the upstream system that originally created this object (comparable to the UML DataSourceTypeId)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ParticipantId\","]
#[doc = "  \"description\": \"Identifier of the upstream system that originally created this object (comparable to the UML DataSourceTypeId).\","]
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
#[doc = "Queue presence and severity reported by the detector."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Queue presence and severity reported by the detector.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"queuePresent\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"queue\\\" selects the QueueDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"queue\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"queuePresent\": {"]
#[doc = "      \"description\": \"Whether a queue is currently detected.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Queue severity when a queue is present.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"normal\","]
#[doc = "        \"dense\","]
#[doc = "        \"delayed\","]
#[doc = "        \"congested\","]
#[doc = "        \"stopAndGo\","]
#[doc = "        \"severe\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueueDynamic {
    #[doc = "Identifier of the detector this reading is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"queue\" selects the QueueDynamic payload; it is not free-text command content."]
    pub kind: QueueDynamicKind,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Whether a queue is currently detected."]
    #[serde(rename = "queuePresent")]
    pub queue_present: bool,
    #[doc = "Queue severity when a queue is present."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<QueueDynamicSeverity>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl QueueDynamic {
    pub fn builder() -> builder::QueueDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"queue\" selects the QueueDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"queue\\\" selects the QueueDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"queue\""]
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
pub enum QueueDynamicKind {
    #[serde(rename = "queue")]
    Queue,
}
impl ::std::fmt::Display for QueueDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Queue => f.write_str("queue"),
        }
    }
}
impl ::std::str::FromStr for QueueDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "queue" => Ok(Self::Queue),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for QueueDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for QueueDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QueueDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Queue severity when a queue is present."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Queue severity when a queue is present.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"normal\","]
#[doc = "    \"dense\","]
#[doc = "    \"delayed\","]
#[doc = "    \"congested\","]
#[doc = "    \"stopAndGo\","]
#[doc = "    \"severe\""]
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
pub enum QueueDynamicSeverity {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "dense")]
    Dense,
    #[serde(rename = "delayed")]
    Delayed,
    #[serde(rename = "congested")]
    Congested,
    #[serde(rename = "stopAndGo")]
    StopAndGo,
    #[serde(rename = "severe")]
    Severe,
}
impl ::std::fmt::Display for QueueDynamicSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Normal => f.write_str("normal"),
            Self::Dense => f.write_str("dense"),
            Self::Delayed => f.write_str("delayed"),
            Self::Congested => f.write_str("congested"),
            Self::StopAndGo => f.write_str("stopAndGo"),
            Self::Severe => f.write_str("severe"),
        }
    }
}
impl ::std::str::FromStr for QueueDynamicSeverity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "normal" => Ok(Self::Normal),
            "dense" => Ok(Self::Dense),
            "delayed" => Ok(Self::Delayed),
            "congested" => Ok(Self::Congested),
            "stopAndGo" => Ok(Self::StopAndGo),
            "severe" => Ok(Self::Severe),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for QueueDynamicSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for QueueDynamicSeverity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for QueueDynamicSeverity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Reset the detector to its default state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Reset the detector to its default state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector receiving the command.\","]
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
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"reset\\\" selects the ResetDetectorCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"reset\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ResetDetectorCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Identifier of the detector receiving the command."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"reset\" selects the ResetDetectorCommand payload; it is not free-text command content."]
    pub kind: ResetDetectorCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ResetDetectorCommand {
    pub fn builder() -> builder::ResetDetectorCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"reset\" selects the ResetDetectorCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"reset\\\" selects the ResetDetectorCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"reset\""]
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
pub enum ResetDetectorCommandKind {
    #[serde(rename = "reset")]
    Reset,
}
impl ::std::fmt::Display for ResetDetectorCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reset => f.write_str("reset"),
        }
    }
}
impl ::std::str::FromStr for ResetDetectorCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reset" => Ok(Self::Reset),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResetDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResetDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResetDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Set occupancy/flow threshold values on the detector."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Set occupancy/flow threshold values on the detector.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"thresholdDown\","]
#[doc = "    \"thresholdUp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"commandCorrelationId\": {"]
#[doc = "      \"description\": \"Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 9007199254740991.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector receiving the command.\","]
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
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"setThreshold\\\" selects the SetThresholdDetectorCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"setThreshold\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"thresholdDown\": {"]
#[doc = "      \"description\": \"Downward threshold value to set.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"thresholdUp\": {"]
#[doc = "      \"description\": \"Upward threshold value to set.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetThresholdDetectorCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Identifier of the detector receiving the command."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"setThreshold\" selects the SetThresholdDetectorCommand payload; it is not free-text command content."]
    pub kind: SetThresholdDetectorCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Downward threshold value to set."]
    #[serde(rename = "thresholdDown")]
    pub threshold_down: u64,
    #[doc = "Upward threshold value to set."]
    #[serde(rename = "thresholdUp")]
    pub threshold_up: u64,
}
impl SetThresholdDetectorCommand {
    pub fn builder() -> builder::SetThresholdDetectorCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"setThreshold\" selects the SetThresholdDetectorCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"setThreshold\\\" selects the SetThresholdDetectorCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"setThreshold\""]
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
pub enum SetThresholdDetectorCommandKind {
    #[serde(rename = "setThreshold")]
    SetThreshold,
}
impl ::std::fmt::Display for SetThresholdDetectorCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetThreshold => f.write_str("setThreshold"),
        }
    }
}
impl ::std::str::FromStr for SetThresholdDetectorCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "setThreshold" => Ok(Self::SetThreshold),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetThresholdDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetThresholdDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetThresholdDetectorCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mean vehicle speed over a measurement period."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Mean vehicle speed over a measurement period.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detectorId\","]
#[doc = "    \"kind\","]
#[doc = "    \"meanSpeedKph\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detectorId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the detector this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"speed\\\" selects the SpeedDynamic payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"speed\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"meanSpeedKph\": {"]
#[doc = "      \"description\": \"Mean vehicle speed in kilometres per hour.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"periodSeconds\": {"]
#[doc = "      \"description\": \"Duration of the measurement period in seconds.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"quality\": {"]
#[doc = "      \"description\": \"Optional quality statement for this measurement.\","]
#[doc = "      \"$ref\": \"#/$defs/Quality\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the measurement was taken.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SpeedDynamic {
    #[doc = "Identifier of the detector this reading is about."]
    #[serde(rename = "detectorId")]
    pub detector_id: ObjectId,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"speed\" selects the SpeedDynamic payload; it is not free-text command content."]
    pub kind: SpeedDynamicKind,
    #[doc = "Mean vehicle speed in kilometres per hour."]
    #[serde(rename = "meanSpeedKph")]
    pub mean_speed_kph: f64,
    #[doc = "Duration of the measurement period in seconds."]
    #[serde(
        rename = "periodSeconds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub period_seconds: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Optional quality statement for this measurement."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quality: ::std::option::Option<Quality>,
    #[doc = "Time the measurement was taken."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl SpeedDynamic {
    pub fn builder() -> builder::SpeedDynamic {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"speed\" selects the SpeedDynamic payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"speed\\\" selects the SpeedDynamic payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"speed\""]
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
pub enum SpeedDynamicKind {
    #[serde(rename = "speed")]
    Speed,
}
impl ::std::fmt::Display for SpeedDynamicKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Speed => f.write_str("speed"),
        }
    }
}
impl ::std::str::FromStr for SpeedDynamicKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "speed" => Ok(Self::Speed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SpeedDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SpeedDynamicKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SpeedDynamicKind {
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
#[doc = "Terminal message ending a detector sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a detector sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of detector objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Detector Sync Request\","]
#[doc = "  \"description\": \"Request the current state of detector objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Unique identifier for a detector type (loop, radar, ANPR, ...)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeId\","]
#[doc = "  \"description\": \"Unique identifier for a detector type (loop, radar, ANPR, ...).\","]
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
        calibrate_detector_command: ::std::result::Result<
            ::std::option::Option<super::CalibrateDetectorCommand>,
            ::std::string::String,
        >,
        command_response_status: ::std::result::Result<
            ::std::option::Option<super::CommandResponseStatus>,
            ::std::string::String,
        >,
        coordinates:
            ::std::result::Result<::std::option::Option<super::Coordinates>, ::std::string::String>,
        detector_command: ::std::result::Result<
            ::std::option::Option<super::DetectorCommand>,
            ::std::string::String,
        >,
        detector_command_response: ::std::result::Result<
            ::std::option::Option<super::DetectorCommandResponse>,
            ::std::string::String,
        >,
        detector_configuration: ::std::result::Result<
            ::std::option::Option<super::DetectorConfiguration>,
            ::std::string::String,
        >,
        detector_definition: ::std::result::Result<
            ::std::option::Option<super::DetectorDefinition>,
            ::std::string::String,
        >,
        detector_fault: ::std::result::Result<
            ::std::option::Option<super::DetectorFault>,
            ::std::string::String,
        >,
        detector_history_entry: ::std::result::Result<
            ::std::option::Option<super::DetectorHistoryEntry>,
            ::std::string::String,
        >,
        detector_measurement: ::std::result::Result<
            ::std::option::Option<super::DetectorMeasurement>,
            ::std::string::String,
        >,
        detector_type: ::std::result::Result<
            ::std::option::Option<super::DetectorType>,
            ::std::string::String,
        >,
        fault_status:
            ::std::result::Result<::std::option::Option<super::FaultStatus>, ::std::string::String>,
        flow_dynamic:
            ::std::result::Result<::std::option::Option<super::FlowDynamic>, ::std::string::String>,
        headway_dynamic: ::std::result::Result<
            ::std::option::Option<super::HeadwayDynamic>,
            ::std::string::String,
        >,
        occupancy_dynamic: ::std::result::Result<
            ::std::option::Option<super::OccupancyDynamic>,
            ::std::string::String,
        >,
        point: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        queue_dynamic: ::std::result::Result<
            ::std::option::Option<super::QueueDynamic>,
            ::std::string::String,
        >,
        reset_detector_command: ::std::result::Result<
            ::std::option::Option<super::ResetDetectorCommand>,
            ::std::string::String,
        >,
        set_threshold_detector_command: ::std::result::Result<
            ::std::option::Option<super::SetThresholdDetectorCommand>,
            ::std::string::String,
        >,
        speed_dynamic: ::std::result::Result<
            ::std::option::Option<super::SpeedDynamic>,
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
                acknowledgement_state: Ok(Default::default()),
                calibrate_detector_command: Ok(Default::default()),
                command_response_status: Ok(Default::default()),
                coordinates: Ok(Default::default()),
                detector_command: Ok(Default::default()),
                detector_command_response: Ok(Default::default()),
                detector_configuration: Ok(Default::default()),
                detector_definition: Ok(Default::default()),
                detector_fault: Ok(Default::default()),
                detector_history_entry: Ok(Default::default()),
                detector_measurement: Ok(Default::default()),
                detector_type: Ok(Default::default()),
                fault_status: Ok(Default::default()),
                flow_dynamic: Ok(Default::default()),
                headway_dynamic: Ok(Default::default()),
                occupancy_dynamic: Ok(Default::default()),
                point: Ok(Default::default()),
                quality: Ok(Default::default()),
                queue_dynamic: Ok(Default::default()),
                reset_detector_command: Ok(Default::default()),
                set_threshold_detector_command: Ok(Default::default()),
                speed_dynamic: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
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
        pub fn calibrate_detector_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CalibrateDetectorCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.calibrate_detector_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for calibrate_detector_command: {e}")
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
        pub fn detector_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_command: {e}"));
            self
        }
        pub fn detector_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for detector_command_response: {e}")
            });
            self
        }
        pub fn detector_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for detector_configuration: {e}")
            });
            self
        }
        pub fn detector_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for detector_definition: {e}")
            });
            self
        }
        pub fn detector_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorFault>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_fault = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_fault: {e}"));
            self
        }
        pub fn detector_history_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorHistoryEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_history_entry = value.try_into().map_err(|e| {
                format!("error converting supplied value for detector_history_entry: {e}")
            });
            self
        }
        pub fn detector_measurement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorMeasurement>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_measurement = value.try_into().map_err(|e| {
                format!("error converting supplied value for detector_measurement: {e}")
            });
            self
        }
        pub fn detector_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorType>>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_type: {e}"));
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
        pub fn flow_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FlowDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.flow_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flow_dynamic: {e}"));
            self
        }
        pub fn headway_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HeadwayDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.headway_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for headway_dynamic: {e}"));
            self
        }
        pub fn occupancy_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OccupancyDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.occupancy_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occupancy_dynamic: {e}"));
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
        pub fn queue_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::QueueDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.queue_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for queue_dynamic: {e}"));
            self
        }
        pub fn reset_detector_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ResetDetectorCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.reset_detector_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for reset_detector_command: {e}")
            });
            self
        }
        pub fn set_threshold_detector_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SetThresholdDetectorCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.set_threshold_detector_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for set_threshold_detector_command: {e}")
            });
            self
        }
        pub fn speed_dynamic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SpeedDynamic>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed_dynamic = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed_dynamic: {e}"));
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
                acknowledgement_state: value.acknowledgement_state?,
                calibrate_detector_command: value.calibrate_detector_command?,
                command_response_status: value.command_response_status?,
                coordinates: value.coordinates?,
                detector_command: value.detector_command?,
                detector_command_response: value.detector_command_response?,
                detector_configuration: value.detector_configuration?,
                detector_definition: value.detector_definition?,
                detector_fault: value.detector_fault?,
                detector_history_entry: value.detector_history_entry?,
                detector_measurement: value.detector_measurement?,
                detector_type: value.detector_type?,
                fault_status: value.fault_status?,
                flow_dynamic: value.flow_dynamic?,
                headway_dynamic: value.headway_dynamic?,
                occupancy_dynamic: value.occupancy_dynamic?,
                point: value.point?,
                quality: value.quality?,
                queue_dynamic: value.queue_dynamic?,
                reset_detector_command: value.reset_detector_command?,
                set_threshold_detector_command: value.set_threshold_detector_command?,
                speed_dynamic: value.speed_dynamic?,
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
                acknowledgement_state: Ok(value.acknowledgement_state),
                calibrate_detector_command: Ok(value.calibrate_detector_command),
                command_response_status: Ok(value.command_response_status),
                coordinates: Ok(value.coordinates),
                detector_command: Ok(value.detector_command),
                detector_command_response: Ok(value.detector_command_response),
                detector_configuration: Ok(value.detector_configuration),
                detector_definition: Ok(value.detector_definition),
                detector_fault: Ok(value.detector_fault),
                detector_history_entry: Ok(value.detector_history_entry),
                detector_measurement: Ok(value.detector_measurement),
                detector_type: Ok(value.detector_type),
                fault_status: Ok(value.fault_status),
                flow_dynamic: Ok(value.flow_dynamic),
                headway_dynamic: Ok(value.headway_dynamic),
                occupancy_dynamic: Ok(value.occupancy_dynamic),
                point: Ok(value.point),
                quality: Ok(value.quality),
                queue_dynamic: Ok(value.queue_dynamic),
                reset_detector_command: Ok(value.reset_detector_command),
                set_threshold_detector_command: Ok(value.set_threshold_detector_command),
                speed_dynamic: Ok(value.speed_dynamic),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CalibrateDetectorCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::CalibrateDetectorCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for CalibrateDetectorCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
            }
        }
    }
    impl CalibrateDetectorCommand {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::CalibrateDetectorCommandKind>,
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
    }
    impl ::std::convert::TryFrom<CalibrateDetectorCommand> for super::CalibrateDetectorCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CalibrateDetectorCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
            })
        }
    }
    impl ::std::convert::From<super::CalibrateDetectorCommand> for CalibrateDetectorCommand {
        fn from(value: super::CalibrateDetectorCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
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
    pub struct DetectorCommandResponse {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
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
    }
    impl ::std::default::Default for DetectorCommandResponse {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                message: Ok(Default::default()),
                response_time: Err("no value supplied for response_time".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl DetectorCommandResponse {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<DetectorCommandResponse> for super::DetectorCommandResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::DetectorCommandResponse> for DetectorCommandResponse {
        fn from(value: super::DetectorCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DetectorConfiguration {
        configuration_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        threshold_down: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        threshold_up: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
    }
    impl ::std::default::Default for DetectorConfiguration {
        fn default() -> Self {
            Self {
                configuration_date: Err("no value supplied for configuration_date".to_string()),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                threshold_down: Ok(Default::default()),
                threshold_up: Ok(Default::default()),
            }
        }
    }
    impl DetectorConfiguration {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
        pub fn threshold_down<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_down = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_down: {e}"));
            self
        }
        pub fn threshold_up<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_up = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_up: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DetectorConfiguration> for super::DetectorConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration_date: value.configuration_date?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                threshold_down: value.threshold_down?,
                threshold_up: value.threshold_up?,
            })
        }
    }
    impl ::std::convert::From<super::DetectorConfiguration> for DetectorConfiguration {
        fn from(value: super::DetectorConfiguration) -> Self {
            Self {
                configuration_date: Ok(value.configuration_date),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                threshold_down: Ok(value.threshold_down),
                threshold_up: Ok(value.threshold_up),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DetectorDefinition {
        creation_date:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        link_distance_metres:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        location: ::std::result::Result<::std::option::Option<super::Point>, ::std::string::String>,
        long_description: ::std::result::Result<
            ::std::option::Option<super::DetectorDefinitionLongDescription>,
            ::std::string::String,
        >,
        network_link_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        network_path_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
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
            ::std::option::Option<super::DetectorDefinitionShortDescription>,
            ::std::string::String,
        >,
        transport_link_id:
            ::std::result::Result<::std::option::Option<super::ObjectId>, ::std::string::String>,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for DetectorDefinition {
        fn default() -> Self {
            Self {
                creation_date: Err("no value supplied for creation_date".to_string()),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                link_distance_metres: Ok(Default::default()),
                location: Ok(Default::default()),
                long_description: Ok(Default::default()),
                network_link_id: Ok(Default::default()),
                network_path_id: Ok(Default::default()),
                originating_object_id: Ok(Default::default()),
                originating_participant_id: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                short_description: Ok(Default::default()),
                transport_link_id: Ok(Default::default()),
                type_id: Err("no value supplied for type_id".to_string()),
            }
        }
    }
    impl DetectorDefinition {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DetectorDefinitionLongDescription>,
            >,
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
                ::std::option::Option<super::DetectorDefinitionShortDescription>,
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
    }
    impl ::std::convert::TryFrom<DetectorDefinition> for super::DetectorDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                creation_date: value.creation_date?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                link_distance_metres: value.link_distance_metres?,
                location: value.location?,
                long_description: value.long_description?,
                network_link_id: value.network_link_id?,
                network_path_id: value.network_path_id?,
                originating_object_id: value.originating_object_id?,
                originating_participant_id: value.originating_participant_id?,
                participant_id: value.participant_id?,
                short_description: value.short_description?,
                transport_link_id: value.transport_link_id?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::DetectorDefinition> for DetectorDefinition {
        fn from(value: super::DetectorDefinition) -> Self {
            Self {
                creation_date: Ok(value.creation_date),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                link_distance_metres: Ok(value.link_distance_metres),
                location: Ok(value.location),
                long_description: Ok(value.long_description),
                network_link_id: Ok(value.network_link_id),
                network_path_id: Ok(value.network_path_id),
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
    pub struct DetectorFault {
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
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
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
    }
    impl ::std::default::Default for DetectorFault {
        fn default() -> Self {
            Self {
                acknowledgement_date: Ok(Default::default()),
                acknowledgement_state: Ok(Default::default()),
                cleared_date: Ok(Default::default()),
                communications_fault: Ok(Default::default()),
                creation_date: Err("no value supplied for creation_date".to_string()),
                description: Ok(Default::default()),
                detector_id: Err("no value supplied for detector_id".to_string()),
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
            }
        }
    }
    impl DetectorFault {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
    }
    impl ::std::convert::TryFrom<DetectorFault> for super::DetectorFault {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorFault,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_date: value.acknowledgement_date?,
                acknowledgement_state: value.acknowledgement_state?,
                cleared_date: value.cleared_date?,
                communications_fault: value.communications_fault?,
                creation_date: value.creation_date?,
                description: value.description?,
                detector_id: value.detector_id?,
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
            })
        }
    }
    impl ::std::convert::From<super::DetectorFault> for DetectorFault {
        fn from(value: super::DetectorFault) -> Self {
            Self {
                acknowledgement_date: Ok(value.acknowledgement_date),
                acknowledgement_state: Ok(value.acknowledgement_state),
                cleared_date: Ok(value.cleared_date),
                communications_fault: Ok(value.communications_fault),
                creation_date: Ok(value.creation_date),
                description: Ok(value.description),
                detector_id: Ok(value.detector_id),
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
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DetectorHistoryEntry {
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
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
    }
    impl ::std::default::Default for DetectorHistoryEntry {
        fn default() -> Self {
            Self {
                detector_id: Err("no value supplied for detector_id".to_string()),
                event_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                log_date: Err("no value supplied for log_date".to_string()),
                log_entry: Err("no value supplied for log_entry".to_string()),
                log_id: Err("no value supplied for log_id".to_string()),
                log_user: Ok(Default::default()),
            }
        }
    }
    impl DetectorHistoryEntry {
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
            self
        }
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
    }
    impl ::std::convert::TryFrom<DetectorHistoryEntry> for super::DetectorHistoryEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorHistoryEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detector_id: value.detector_id?,
                event_type: value.event_type?,
                extensions: value.extensions?,
                log_date: value.log_date?,
                log_entry: value.log_entry?,
                log_id: value.log_id?,
                log_user: value.log_user?,
            })
        }
    }
    impl ::std::convert::From<super::DetectorHistoryEntry> for DetectorHistoryEntry {
        fn from(value: super::DetectorHistoryEntry) -> Self {
            Self {
                detector_id: Ok(value.detector_id),
                event_type: Ok(value.event_type),
                extensions: Ok(value.extensions),
                log_date: Ok(value.log_date),
                log_entry: Ok(value.log_entry),
                log_id: Ok(value.log_id),
                log_user: Ok(value.log_user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DetectorType {
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
            ::std::option::Option<super::DetectorTypeSpecificationLink>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for DetectorType {
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
    impl DetectorType {
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
            T: ::std::convert::TryInto<::std::option::Option<super::DetectorTypeSpecificationLink>>,
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
    impl ::std::convert::TryFrom<DetectorType> for super::DetectorType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DetectorType,
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
    impl ::std::convert::From<super::DetectorType> for DetectorType {
        fn from(value: super::DetectorType) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                notes: Ok(value.notes),
                specification_link: Ok(value.specification_link),
                type_id: Ok(value.type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FlowDynamic {
        class_counts: ::std::result::Result<
            ::std::vec::Vec<::std::option::Option<u64>>,
            ::std::string::String,
        >,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::FlowDynamicKind, ::std::string::String>,
        period_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        total_flow: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
    }
    impl ::std::default::Default for FlowDynamic {
        fn default() -> Self {
            Self {
                class_counts: Ok(Default::default()),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                period_seconds: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                total_flow: Ok(Default::default()),
            }
        }
    }
    impl FlowDynamic {
        pub fn class_counts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::option::Option<u64>>>,
            T::Error: ::std::fmt::Display,
        {
            self.class_counts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class_counts: {e}"));
            self
        }
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::FlowDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn period_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.period_seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period_seconds: {e}"));
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
        pub fn total_flow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.total_flow = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_flow: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FlowDynamic> for super::FlowDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FlowDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                class_counts: value.class_counts?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                period_seconds: value.period_seconds?,
                quality: value.quality?,
                timestamp: value.timestamp?,
                total_flow: value.total_flow?,
            })
        }
    }
    impl ::std::convert::From<super::FlowDynamic> for FlowDynamic {
        fn from(value: super::FlowDynamic) -> Self {
            Self {
                class_counts: Ok(value.class_counts),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                period_seconds: Ok(value.period_seconds),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
                total_flow: Ok(value.total_flow),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HeadwayDynamic {
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::HeadwayDynamicKind, ::std::string::String>,
        mean_headway_seconds: ::std::result::Result<f64, ::std::string::String>,
        period_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for HeadwayDynamic {
        fn default() -> Self {
            Self {
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                mean_headway_seconds: Err("no value supplied for mean_headway_seconds".to_string()),
                period_seconds: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl HeadwayDynamic {
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::HeadwayDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn mean_headway_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.mean_headway_seconds = value.try_into().map_err(|e| {
                format!("error converting supplied value for mean_headway_seconds: {e}")
            });
            self
        }
        pub fn period_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.period_seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period_seconds: {e}"));
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
    }
    impl ::std::convert::TryFrom<HeadwayDynamic> for super::HeadwayDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HeadwayDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                mean_headway_seconds: value.mean_headway_seconds?,
                period_seconds: value.period_seconds?,
                quality: value.quality?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::HeadwayDynamic> for HeadwayDynamic {
        fn from(value: super::HeadwayDynamic) -> Self {
            Self {
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                mean_headway_seconds: Ok(value.mean_headway_seconds),
                period_seconds: Ok(value.period_seconds),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OccupancyDynamic {
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::OccupancyDynamicKind, ::std::string::String>,
        occupancy_percent: ::std::result::Result<f64, ::std::string::String>,
        period_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for OccupancyDynamic {
        fn default() -> Self {
            Self {
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                occupancy_percent: Err("no value supplied for occupancy_percent".to_string()),
                period_seconds: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl OccupancyDynamic {
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::OccupancyDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn occupancy_percent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.occupancy_percent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occupancy_percent: {e}"));
            self
        }
        pub fn period_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.period_seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period_seconds: {e}"));
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
    }
    impl ::std::convert::TryFrom<OccupancyDynamic> for super::OccupancyDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OccupancyDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                occupancy_percent: value.occupancy_percent?,
                period_seconds: value.period_seconds?,
                quality: value.quality?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::OccupancyDynamic> for OccupancyDynamic {
        fn from(value: super::OccupancyDynamic) -> Self {
            Self {
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                occupancy_percent: Ok(value.occupancy_percent),
                period_seconds: Ok(value.period_seconds),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
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
    pub struct QueueDynamic {
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::QueueDynamicKind, ::std::string::String>,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        queue_present: ::std::result::Result<bool, ::std::string::String>,
        severity: ::std::result::Result<
            ::std::option::Option<super::QueueDynamicSeverity>,
            ::std::string::String,
        >,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for QueueDynamic {
        fn default() -> Self {
            Self {
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                quality: Ok(Default::default()),
                queue_present: Err("no value supplied for queue_present".to_string()),
                severity: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl QueueDynamic {
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::QueueDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
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
        pub fn queue_present<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.queue_present = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for queue_present: {e}"));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::QueueDynamicSeverity>>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {e}"));
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
    }
    impl ::std::convert::TryFrom<QueueDynamic> for super::QueueDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: QueueDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                quality: value.quality?,
                queue_present: value.queue_present?,
                severity: value.severity?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::QueueDynamic> for QueueDynamic {
        fn from(value: super::QueueDynamic) -> Self {
            Self {
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                quality: Ok(value.quality),
                queue_present: Ok(value.queue_present),
                severity: Ok(value.severity),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ResetDetectorCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::ResetDetectorCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for ResetDetectorCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
            }
        }
    }
    impl ResetDetectorCommand {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::ResetDetectorCommandKind>,
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
    }
    impl ::std::convert::TryFrom<ResetDetectorCommand> for super::ResetDetectorCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ResetDetectorCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
            })
        }
    }
    impl ::std::convert::From<super::ResetDetectorCommand> for ResetDetectorCommand {
        fn from(value: super::ResetDetectorCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetThresholdDetectorCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SetThresholdDetectorCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        threshold_down: ::std::result::Result<u64, ::std::string::String>,
        threshold_up: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for SetThresholdDetectorCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                threshold_down: Err("no value supplied for threshold_down".to_string()),
                threshold_up: Err("no value supplied for threshold_up".to_string()),
            }
        }
    }
    impl SetThresholdDetectorCommand {
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
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::SetThresholdDetectorCommandKind>,
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
        pub fn threshold_down<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_down = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_down: {e}"));
            self
        }
        pub fn threshold_up<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.threshold_up = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for threshold_up: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetThresholdDetectorCommand> for super::SetThresholdDetectorCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetThresholdDetectorCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                threshold_down: value.threshold_down?,
                threshold_up: value.threshold_up?,
            })
        }
    }
    impl ::std::convert::From<super::SetThresholdDetectorCommand> for SetThresholdDetectorCommand {
        fn from(value: super::SetThresholdDetectorCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                threshold_down: Ok(value.threshold_down),
                threshold_up: Ok(value.threshold_up),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeedDynamic {
        detector_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SpeedDynamicKind, ::std::string::String>,
        mean_speed_kph: ::std::result::Result<f64, ::std::string::String>,
        period_seconds: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        quality:
            ::std::result::Result<::std::option::Option<super::Quality>, ::std::string::String>,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for SpeedDynamic {
        fn default() -> Self {
            Self {
                detector_id: Err("no value supplied for detector_id".to_string()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                mean_speed_kph: Err("no value supplied for mean_speed_kph".to_string()),
                period_seconds: Ok(Default::default()),
                quality: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl SpeedDynamic {
        pub fn detector_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.detector_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detector_id: {e}"));
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
            T: ::std::convert::TryInto<super::SpeedDynamicKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn mean_speed_kph<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.mean_speed_kph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mean_speed_kph: {e}"));
            self
        }
        pub fn period_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.period_seconds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for period_seconds: {e}"));
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
    }
    impl ::std::convert::TryFrom<SpeedDynamic> for super::SpeedDynamic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeedDynamic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                detector_id: value.detector_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                mean_speed_kph: value.mean_speed_kph?,
                period_seconds: value.period_seconds?,
                quality: value.quality?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::SpeedDynamic> for SpeedDynamic {
        fn from(value: super::SpeedDynamic) -> Self {
            Self {
                detector_id: Ok(value.detector_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                mean_speed_kph: Ok(value.mean_speed_kph),
                period_seconds: Ok(value.period_seconds),
                quality: Ok(value.quality),
                timestamp: Ok(value.timestamp),
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
