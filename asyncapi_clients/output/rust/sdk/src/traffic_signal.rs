/*
 * File:        output/rust/sdk/src/traffic_signal.rs
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
#[doc = "    \"FaultSeverityEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultSeverityEnum\""]
#[doc = "    },"]
#[doc = "    \"ForceStageCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/ForceStageCommand\""]
#[doc = "    },"]
#[doc = "    \"GeoJsonPoint\": {"]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
#[doc = "    },"]
#[doc = "    \"HoldStageCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/HoldStageCommand\""]
#[doc = "    },"]
#[doc = "    \"PlanLine\": {"]
#[doc = "      \"$ref\": \"#/$defs/PlanLine\""]
#[doc = "    },"]
#[doc = "    \"RequestStatusCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/RequestStatusCommand\""]
#[doc = "    },"]
#[doc = "    \"SetPlanCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/SetPlanCommand\""]
#[doc = "    },"]
#[doc = "    \"SignalGroupTimings\": {"]
#[doc = "      \"$ref\": \"#/$defs/SignalGroupTimings\""]
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
#[doc = "    \"SynchronizeClockCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/SynchronizeClockCommand\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalCommand\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalCommand\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalCommandResponse\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalCommandResponse\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalConfiguration\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalConfiguration\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalDefinition\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalDynamicState\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalDynamicState\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalFault\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalFault\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalFaultEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalFaultEnum\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalHistoryEntry\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalHistoryEntry\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalModeEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalModeEnum\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalSubModeEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalSubModeEnum\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalType\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalType\""]
#[doc = "    },"]
#[doc = "    \"TrafficSignalTypeEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalTypeEnum\""]
#[doc = "    },"]
#[doc = "    \"UrgencyEnum\": {"]
#[doc = "      \"$ref\": \"#/$defs/UrgencyEnum\""]
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
        rename = "FaultSeverityEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_severity_enum: ::std::option::Option<FaultSeverityEnum>,
    #[serde(
        rename = "ForceStageCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub force_stage_command: ::std::option::Option<ForceStageCommand>,
    #[serde(
        rename = "GeoJsonPoint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geo_json_point: ::std::option::Option<GeoJsonPoint>,
    #[serde(
        rename = "HoldStageCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hold_stage_command: ::std::option::Option<HoldStageCommand>,
    #[serde(
        rename = "PlanLine",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_line: ::std::option::Option<PlanLine>,
    #[serde(
        rename = "RequestStatusCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub request_status_command: ::std::option::Option<RequestStatusCommand>,
    #[serde(
        rename = "SetPlanCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub set_plan_command: ::std::option::Option<SetPlanCommand>,
    #[serde(
        rename = "SignalGroupTimings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub signal_group_timings: ::std::option::Option<SignalGroupTimings>,
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
        rename = "SynchronizeClockCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub synchronize_clock_command: ::std::option::Option<SynchronizeClockCommand>,
    #[serde(
        rename = "TrafficSignalCommand",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_command: ::std::option::Option<TrafficSignalCommand>,
    #[serde(
        rename = "TrafficSignalCommandResponse",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_command_response: ::std::option::Option<TrafficSignalCommandResponse>,
    #[serde(
        rename = "TrafficSignalConfiguration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_configuration: ::std::option::Option<TrafficSignalConfiguration>,
    #[serde(
        rename = "TrafficSignalDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_definition: ::std::option::Option<TrafficSignalDefinition>,
    #[serde(
        rename = "TrafficSignalDynamicState",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_dynamic_state: ::std::option::Option<TrafficSignalDynamicState>,
    #[serde(
        rename = "TrafficSignalFault",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_fault: ::std::option::Option<TrafficSignalFault>,
    #[serde(
        rename = "TrafficSignalFaultEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_fault_enum: ::std::option::Option<TrafficSignalFaultEnum>,
    #[serde(
        rename = "TrafficSignalHistoryEntry",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_history_entry: ::std::option::Option<TrafficSignalHistoryEntry>,
    #[serde(
        rename = "TrafficSignalModeEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_mode_enum: ::std::option::Option<TrafficSignalModeEnum>,
    #[serde(
        rename = "TrafficSignalSubModeEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_sub_mode_enum: ::std::option::Option<TrafficSignalSubModeEnum>,
    #[serde(
        rename = "TrafficSignalType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_type: ::std::option::Option<TrafficSignalType>,
    #[serde(
        rename = "TrafficSignalTypeEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_type_enum: ::std::option::Option<TrafficSignalTypeEnum>,
    #[serde(
        rename = "UrgencyEnum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub urgency_enum: ::std::option::Option<UrgencyEnum>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            command_response_status: Default::default(),
            fault_severity_enum: Default::default(),
            force_stage_command: Default::default(),
            geo_json_point: Default::default(),
            hold_stage_command: Default::default(),
            plan_line: Default::default(),
            request_status_command: Default::default(),
            set_plan_command: Default::default(),
            signal_group_timings: Default::default(),
            sync_category_enum: Default::default(),
            sync_complete: Default::default(),
            sync_request: Default::default(),
            sync_status: Default::default(),
            synchronize_clock_command: Default::default(),
            traffic_signal_command: Default::default(),
            traffic_signal_command_response: Default::default(),
            traffic_signal_configuration: Default::default(),
            traffic_signal_definition: Default::default(),
            traffic_signal_dynamic_state: Default::default(),
            traffic_signal_fault: Default::default(),
            traffic_signal_fault_enum: Default::default(),
            traffic_signal_history_entry: Default::default(),
            traffic_signal_mode_enum: Default::default(),
            traffic_signal_sub_mode_enum: Default::default(),
            traffic_signal_type: Default::default(),
            traffic_signal_type_enum: Default::default(),
            urgency_enum: Default::default(),
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
#[doc = "DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"low\","]
#[doc = "    \"medium\","]
#[doc = "    \"high\","]
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
pub enum FaultSeverityEnum {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for FaultSeverityEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for FaultSeverityEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FaultSeverityEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FaultSeverityEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FaultSeverityEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Force a specific stage on a traffic signal controller."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Force a specific stage on a traffic signal controller.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"stageNumber\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"forceStage\\\" selects the ForceStageCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"forceStage\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"stageNumber\": {"]
#[doc = "      \"description\": \"Stage number to force.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
pub struct ForceStageCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"forceStage\" selects the ForceStageCommand payload; it is not free-text command content."]
    pub kind: ForceStageCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Stage number to force."]
    #[serde(rename = "stageNumber")]
    pub stage_number: u64,
    #[doc = "Identifier of the traffic signal receiving the command."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl ForceStageCommand {
    pub fn builder() -> builder::ForceStageCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"forceStage\" selects the ForceStageCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"forceStage\\\" selects the ForceStageCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"forceStage\""]
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
pub enum ForceStageCommandKind {
    #[serde(rename = "forceStage")]
    ForceStage,
}
impl ::std::fmt::Display for ForceStageCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ForceStage => f.write_str("forceStage"),
        }
    }
}
impl ::std::str::FromStr for ForceStageCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "forceStage" => Ok(Self::ForceStage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ForceStageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ForceStageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ForceStageCommandKind {
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
#[doc = "Hold a specific stage on a traffic signal controller."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Hold a specific stage on a traffic signal controller.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"stageNumber\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"holdStage\\\" selects the HoldStageCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"holdStage\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"stageNumber\": {"]
#[doc = "      \"description\": \"Stage number to hold.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
pub struct HoldStageCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"holdStage\" selects the HoldStageCommand payload; it is not free-text command content."]
    pub kind: HoldStageCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Stage number to hold."]
    #[serde(rename = "stageNumber")]
    pub stage_number: u64,
    #[doc = "Identifier of the traffic signal receiving the command."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl HoldStageCommand {
    pub fn builder() -> builder::HoldStageCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"holdStage\" selects the HoldStageCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"holdStage\\\" selects the HoldStageCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"holdStage\""]
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
pub enum HoldStageCommandKind {
    #[serde(rename = "holdStage")]
    HoldStage,
}
impl ::std::fmt::Display for HoldStageCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::HoldStage => f.write_str("holdStage"),
        }
    }
}
impl ::std::str::FromStr for HoldStageCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "holdStage" => Ok(Self::HoldStage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for HoldStageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HoldStageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HoldStageCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Identifier of the traffic signal receiving the command."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
#[doc = "A signal plan definition carried in the dynamic state (§3.5, §3.6)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A signal plan definition carried in the dynamic state (§3.5, §3.6).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"planNumber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cycleTime\": {"]
#[doc = "      \"description\": \"The current cycle time.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 1000.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"offset\": {"]
#[doc = "      \"description\": \"The offset to the first SignalGroupTiming in the list. Must be present if SignalGroupTimingList is present.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"planDescription\": {"]
#[doc = "      \"description\": \"A user entered description of the active plan.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 50,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"planNumber\": {"]
#[doc = "      \"description\": \"The currently active plan.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 99.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"planStartTime\": {"]
#[doc = "      \"description\": \"The time the plan was started. Optional: a publisher emitting on a fixed interval may not know the start time of the running plan.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"signalGroupTimings\": {"]
#[doc = "      \"description\": \"The signal group data for the plan.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SignalGroupTimings\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlanLine {
    #[doc = "The current cycle time."]
    #[serde(
        rename = "cycleTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cycle_time: ::std::option::Option<::std::num::NonZeroU64>,
    #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The offset to the first SignalGroupTiming in the list. Must be present if SignalGroupTimingList is present."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub offset: ::std::option::Option<u64>,
    #[doc = "A user entered description of the active plan."]
    #[serde(
        rename = "planDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_description: ::std::option::Option<PlanLinePlanDescription>,
    #[doc = "The currently active plan."]
    #[serde(rename = "planNumber")]
    pub plan_number: ::std::num::NonZeroU64,
    #[doc = "The time the plan was started. Optional: a publisher emitting on a fixed interval may not know the start time of the running plan."]
    #[serde(
        rename = "planStartTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_start_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The signal group data for the plan."]
    #[serde(
        rename = "signalGroupTimings",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub signal_group_timings: ::std::vec::Vec<SignalGroupTimings>,
}
impl PlanLine {
    pub fn builder() -> builder::PlanLine {
        Default::default()
    }
}
#[doc = "A user entered description of the active plan."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A user entered description of the active plan.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 50,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PlanLinePlanDescription(::std::string::String);
impl ::std::ops::Deref for PlanLinePlanDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PlanLinePlanDescription> for ::std::string::String {
    fn from(value: PlanLinePlanDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PlanLinePlanDescription {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 50usize {
            return Err("longer than 50 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PlanLinePlanDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlanLinePlanDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlanLinePlanDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PlanLinePlanDescription {
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
#[doc = "Request the current status of a traffic signal controller."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Request the current status of a traffic signal controller.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestStatusCommand payload; it is not free-text command content.\","]
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
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
pub struct RequestStatusCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestStatusCommand payload; it is not free-text command content."]
    pub kind: RequestStatusCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the traffic signal receiving the command."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl RequestStatusCommand {
    pub fn builder() -> builder::RequestStatusCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"requestStatus\" selects the RequestStatusCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"requestStatus\\\" selects the RequestStatusCommand payload; it is not free-text command content.\","]
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
pub enum RequestStatusCommandKind {
    #[serde(rename = "requestStatus")]
    RequestStatus,
}
impl ::std::fmt::Display for RequestStatusCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RequestStatus => f.write_str("requestStatus"),
        }
    }
}
impl ::std::str::FromStr for RequestStatusCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requestStatus" => Ok(Self::RequestStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RequestStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RequestStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RequestStatusCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Set the active plan on a traffic signal controller."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Set the active plan on a traffic signal controller.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"planNumber\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"setPlan\\\" selects the SetPlanCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"setPlan\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"planNumber\": {"]
#[doc = "      \"description\": \"Plan number to activate. Same 1 to 99 range as the plan number a controller reports (PlanLine.plan_number), so any plan the controller can be told to run can be reported back.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 99.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
pub struct SetPlanCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"setPlan\" selects the SetPlanCommand payload; it is not free-text command content."]
    pub kind: SetPlanCommandKind,
    #[doc = "Plan number to activate. Same 1 to 99 range as the plan number a controller reports (PlanLine.plan_number), so any plan the controller can be told to run can be reported back."]
    #[serde(rename = "planNumber")]
    pub plan_number: ::std::num::NonZeroU64,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the traffic signal receiving the command."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl SetPlanCommand {
    pub fn builder() -> builder::SetPlanCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"setPlan\" selects the SetPlanCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"setPlan\\\" selects the SetPlanCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"setPlan\""]
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
pub enum SetPlanCommandKind {
    #[serde(rename = "setPlan")]
    SetPlan,
}
impl ::std::fmt::Display for SetPlanCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SetPlan => f.write_str("setPlan"),
        }
    }
}
impl ::std::str::FromStr for SetPlanCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "setPlan" => Ok(Self::SetPlan),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetPlanCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetPlanCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetPlanCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Signal-group timing row within a PlanLine (§3.5, §3.6)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Signal-group timing row within a PlanLine (§3.5, §3.6).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"primarySignalGroup\","]
#[doc = "    \"primarySignalGroupDuration\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternateSignalGroup\": {"]
#[doc = "      \"description\": \"A text representation of the alternatives to the primary signal group.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 20,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"primarySignalGroup\": {"]
#[doc = "      \"description\": \"A text representation of the primary signal group.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 20,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"primarySignalGroupDuration\": {"]
#[doc = "      \"description\": \"The length of the primary signal group. Note that there is no timing data associated with the alternate signal group.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 998.0,"]
#[doc = "      \"minimum\": 2.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SignalGroupTimings {
    #[doc = "A text representation of the alternatives to the primary signal group."]
    #[serde(
        rename = "alternateSignalGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub alternate_signal_group: ::std::option::Option<SignalGroupTimingsAlternateSignalGroup>,
    #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "A text representation of the primary signal group."]
    #[serde(rename = "primarySignalGroup")]
    pub primary_signal_group: SignalGroupTimingsPrimarySignalGroup,
    #[doc = "The length of the primary signal group. Note that there is no timing data associated with the alternate signal group."]
    #[serde(rename = "primarySignalGroupDuration")]
    pub primary_signal_group_duration: i64,
}
impl SignalGroupTimings {
    pub fn builder() -> builder::SignalGroupTimings {
        Default::default()
    }
}
#[doc = "A text representation of the alternatives to the primary signal group."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A text representation of the alternatives to the primary signal group.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 20,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SignalGroupTimingsAlternateSignalGroup(::std::string::String);
impl ::std::ops::Deref for SignalGroupTimingsAlternateSignalGroup {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SignalGroupTimingsAlternateSignalGroup> for ::std::string::String {
    fn from(value: SignalGroupTimingsAlternateSignalGroup) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SignalGroupTimingsAlternateSignalGroup {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 20usize {
            return Err("longer than 20 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SignalGroupTimingsAlternateSignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SignalGroupTimingsAlternateSignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SignalGroupTimingsAlternateSignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SignalGroupTimingsAlternateSignalGroup {
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
#[doc = "A text representation of the primary signal group."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A text representation of the primary signal group.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 20,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SignalGroupTimingsPrimarySignalGroup(::std::string::String);
impl ::std::ops::Deref for SignalGroupTimingsPrimarySignalGroup {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SignalGroupTimingsPrimarySignalGroup> for ::std::string::String {
    fn from(value: SignalGroupTimingsPrimarySignalGroup) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SignalGroupTimingsPrimarySignalGroup {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 20usize {
            return Err("longer than 20 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SignalGroupTimingsPrimarySignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SignalGroupTimingsPrimarySignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SignalGroupTimingsPrimarySignalGroup {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SignalGroupTimingsPrimarySignalGroup {
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
#[doc = "Terminal message ending a traffic signal sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic signal Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a traffic signal sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of traffic signal objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic signal Sync Request\","]
#[doc = "  \"description\": \"Request the current state of traffic signal objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Synchronize the traffic signal controller's clock."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Synchronize the traffic signal controller's clock.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"kind\","]
#[doc = "    \"requestTime\","]
#[doc = "    \"targetTime\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "      \"description\": \"Discriminator for the tagged union. The fixed value \\\"synchronizeClock\\\" selects the SynchronizeClockCommand payload; it is not free-text command content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"synchronizeClock\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requestTime\": {"]
#[doc = "      \"description\": \"When the command was issued.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"targetTime\": {"]
#[doc = "      \"description\": \"Target time to synchronize the controller clock to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal receiving the command.\","]
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
pub struct SynchronizeClockCommand {
    #[doc = "Caller-assigned identifier correlating a command request with its response. Not an identifier of the command itself. Named to avoid collision with the AMQP 1.0 correlation-id message property."]
    #[serde(rename = "commandCorrelationId")]
    pub command_correlation_id: i64,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Discriminator for the tagged union. The fixed value \"synchronizeClock\" selects the SynchronizeClockCommand payload; it is not free-text command content."]
    pub kind: SynchronizeClockCommandKind,
    #[doc = "When the command was issued."]
    #[serde(rename = "requestTime")]
    pub request_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Target time to synchronize the controller clock to."]
    #[serde(rename = "targetTime")]
    pub target_time: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the traffic signal receiving the command."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl SynchronizeClockCommand {
    pub fn builder() -> builder::SynchronizeClockCommand {
        Default::default()
    }
}
#[doc = "Discriminator for the tagged union. The fixed value \"synchronizeClock\" selects the SynchronizeClockCommand payload; it is not free-text command content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Discriminator for the tagged union. The fixed value \\\"synchronizeClock\\\" selects the SynchronizeClockCommand payload; it is not free-text command content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"synchronizeClock\""]
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
pub enum SynchronizeClockCommandKind {
    #[serde(rename = "synchronizeClock")]
    SynchronizeClock,
}
impl ::std::fmt::Display for SynchronizeClockCommandKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SynchronizeClock => f.write_str("synchronizeClock"),
        }
    }
}
impl ::std::str::FromStr for SynchronizeClockCommandKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "synchronizeClock" => Ok(Self::SynchronizeClock),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SynchronizeClockCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SynchronizeClockCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SynchronizeClockCommandKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Traffic signal command payloads (reused from TS004.0061:2010 Annex D)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Command\","]
#[doc = "  \"description\": \"Traffic signal command payloads (reused from TS004.0061:2010 Annex D).\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SetPlanCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ForceStageCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/HoldStageCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/SynchronizeClockCommand\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RequestStatusCommand\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"discriminator\": {"]
#[doc = "    \"mapping\": {"]
#[doc = "      \"forceStage\": \"#/$defs/ForceStageCommand\","]
#[doc = "      \"holdStage\": \"#/$defs/HoldStageCommand\","]
#[doc = "      \"requestStatus\": \"#/$defs/RequestStatusCommand\","]
#[doc = "      \"setPlan\": \"#/$defs/SetPlanCommand\","]
#[doc = "      \"synchronizeClock\": \"#/$defs/SynchronizeClockCommand\""]
#[doc = "    },"]
#[doc = "    \"propertyName\": \"kind\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TrafficSignalCommand {
    SetPlanCommand(SetPlanCommand),
    ForceStageCommand(ForceStageCommand),
    HoldStageCommand(HoldStageCommand),
    SynchronizeClockCommand(SynchronizeClockCommand),
    RequestStatusCommand(RequestStatusCommand),
}
impl ::std::convert::From<SetPlanCommand> for TrafficSignalCommand {
    fn from(value: SetPlanCommand) -> Self {
        Self::SetPlanCommand(value)
    }
}
impl ::std::convert::From<ForceStageCommand> for TrafficSignalCommand {
    fn from(value: ForceStageCommand) -> Self {
        Self::ForceStageCommand(value)
    }
}
impl ::std::convert::From<HoldStageCommand> for TrafficSignalCommand {
    fn from(value: HoldStageCommand) -> Self {
        Self::HoldStageCommand(value)
    }
}
impl ::std::convert::From<SynchronizeClockCommand> for TrafficSignalCommand {
    fn from(value: SynchronizeClockCommand) -> Self {
        Self::SynchronizeClockCommand(value)
    }
}
impl ::std::convert::From<RequestStatusCommand> for TrafficSignalCommand {
    fn from(value: RequestStatusCommand) -> Self {
        Self::RequestStatusCommand(value)
    }
}
#[doc = "Traffic signal command response (reused from TS004.0061:2010 Annex D)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Command Response\","]
#[doc = "  \"description\": \"Traffic signal command response (reused from TS004.0061:2010 Annex D).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"commandCorrelationId\","]
#[doc = "    \"responseTime\","]
#[doc = "    \"status\","]
#[doc = "    \"trafficSignalId\""]
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
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal the command targeted.\","]
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
pub struct TrafficSignalCommandResponse {
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
    #[doc = "Identifier of the traffic signal the command targeted."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl TrafficSignalCommandResponse {
    pub fn builder() -> builder::TrafficSignalCommandResponse {
        Default::default()
    }
}
#[doc = "Traffic signal operational configuration (TS004.0064bis:2017 §3.4)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Configuration\","]
#[doc = "  \"description\": \"Traffic signal operational configuration (TS004.0064bis:2017 §3.4).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"trafficSignalId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configurationDate\": {"]
#[doc = "      \"description\": \"When this configuration record became effective.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"controllerType\": {"]
#[doc = "      \"description\": \"Textual description of controller type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"subArea\": {"]
#[doc = "      \"description\": \"Group of signals to which this traffic signal belongs.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"supplier\": {"]
#[doc = "      \"description\": \"Name of supplier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal this configuration applies to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalType\": {"]
#[doc = "      \"description\": \"Type of the traffic signal.\","]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalTypeEnum\""]
#[doc = "    },"]
#[doc = "    \"userCodeNumber\": {"]
#[doc = "      \"description\": \"Operator's own reference for this traffic signal.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrafficSignalConfiguration {
    #[doc = "When this configuration record became effective."]
    #[serde(
        rename = "configurationDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub configuration_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Textual description of controller type."]
    #[serde(
        rename = "controllerType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub controller_type: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Group of signals to which this traffic signal belongs."]
    #[serde(
        rename = "subArea",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_area: ::std::option::Option<::std::string::String>,
    #[doc = "Name of supplier."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supplier: ::std::option::Option<::std::string::String>,
    #[doc = "Identifier of the traffic signal this configuration applies to."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
    #[doc = "Type of the traffic signal."]
    #[serde(
        rename = "trafficSignalType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_type: ::std::option::Option<TrafficSignalTypeEnum>,
    #[doc = "Operator's own reference for this traffic signal."]
    #[serde(
        rename = "userCodeNumber",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub user_code_number: ::std::option::Option<TrafficSignalConfigurationUserCodeNumber>,
}
impl TrafficSignalConfiguration {
    pub fn builder() -> builder::TrafficSignalConfiguration {
        Default::default()
    }
}
#[doc = "Operator's own reference for this traffic signal."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Operator's own reference for this traffic signal.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TrafficSignalConfigurationUserCodeNumber(::std::string::String);
impl ::std::ops::Deref for TrafficSignalConfigurationUserCodeNumber {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TrafficSignalConfigurationUserCodeNumber> for ::std::string::String {
    fn from(value: TrafficSignalConfigurationUserCodeNumber) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TrafficSignalConfigurationUserCodeNumber {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalConfigurationUserCodeNumber {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalConfigurationUserCodeNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalConfigurationUserCodeNumber {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TrafficSignalConfigurationUserCodeNumber {
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
#[doc = "Traffic signal equipment details."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Definition\","]
#[doc = "  \"description\": \"Traffic signal equipment details.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"trafficSignalId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"controlMethodId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Optional reference into the TrafficSignalType catalogue identifying the operational control method (MOVA, SCOOT, fixed-time, ...). Orthogonal to TrafficSignalConfiguration.trafficSignalType, which classifies the junction kind (intersection, pelican, puffin, ...).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"creationDate\": {"]
#[doc = "      \"description\": \"When this definition record was created.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Location of traffic signal.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
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
#[doc = "      \"description\": \"System that owns this traffic signal definition. Often implicit from context.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"shortDescription\": {"]
#[doc = "      \"description\": \"Short human-readable name (e.g. 'Main St / High St junction').\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Human readable identifier for the Traffic Signal.\","]
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
pub struct TrafficSignalDefinition {
    #[doc = "Optional reference into the TrafficSignalType catalogue identifying the operational control method (MOVA, SCOOT, fixed-time, ...). Orthogonal to TrafficSignalConfiguration.trafficSignalType, which classifies the junction kind (intersection, pelican, puffin, ...)."]
    #[serde(
        rename = "controlMethodId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub control_method_id: ::std::option::Option<TypeId>,
    #[doc = "When this definition record was created."]
    #[serde(
        rename = "creationDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub creation_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Location of traffic signal."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<GeoJsonPoint>,
    #[doc = "Longer description (context, commissioning notes, etc.)."]
    #[serde(
        rename = "longDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub long_description: ::std::option::Option<TrafficSignalDefinitionLongDescription>,
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
    #[doc = "System that owns this traffic signal definition. Often implicit from context."]
    #[serde(
        rename = "participantId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub participant_id: ::std::option::Option<ParticipantId>,
    #[doc = "Short human-readable name (e.g. 'Main St / High St junction')."]
    #[serde(
        rename = "shortDescription",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_description: ::std::option::Option<TrafficSignalDefinitionShortDescription>,
    #[doc = "Human readable identifier for the Traffic Signal."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl TrafficSignalDefinition {
    pub fn builder() -> builder::TrafficSignalDefinition {
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
pub struct TrafficSignalDefinitionLongDescription(::std::string::String);
impl ::std::ops::Deref for TrafficSignalDefinitionLongDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TrafficSignalDefinitionLongDescription> for ::std::string::String {
    fn from(value: TrafficSignalDefinitionLongDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TrafficSignalDefinitionLongDescription {
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
impl ::std::convert::TryFrom<&str> for TrafficSignalDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalDefinitionLongDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TrafficSignalDefinitionLongDescription {
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
#[doc = "Short human-readable name (e.g. 'Main St / High St junction')."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short human-readable name (e.g. 'Main St / High St junction').\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TrafficSignalDefinitionShortDescription(::std::string::String);
impl ::std::ops::Deref for TrafficSignalDefinitionShortDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TrafficSignalDefinitionShortDescription> for ::std::string::String {
    fn from(value: TrafficSignalDefinitionShortDescription) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TrafficSignalDefinitionShortDescription {
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
impl ::std::convert::TryFrom<&str> for TrafficSignalDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalDefinitionShortDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TrafficSignalDefinitionShortDescription {
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
#[doc = "Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Dynamic State\","]
#[doc = "  \"description\": \"Traffic signal dynamic state with PlanLine/SignalGroupTimings (TS004.0064bis:2017 §3.5).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"timestamp\","]
#[doc = "    \"trafficSignalId\","]
#[doc = "    \"trafficSignalMode\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"planLine\": {"]
#[doc = "      \"description\": \"A plan line for this traffic signal. This is intended to provide information allowing the plan data to be displayed to an operator. It contains the current plan data and would be updated when the plan is changed. For adaptive central control, such as SCOOT, the information should be available periodically and contain the plan at that instant.\","]
#[doc = "      \"$ref\": \"#/$defs/PlanLine\""]
#[doc = "    },"]
#[doc = "    \"subArea\": {"]
#[doc = "      \"description\": \"Name for the sub-area that identifies the group of signals to which the associated traffic signal equipment belongs.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"Time the state was observed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal this reading is about.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalMode\": {"]
#[doc = "      \"description\": \"The high level mode of the traffic signal.\","]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalModeEnum\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalSubMode\": {"]
#[doc = "      \"description\": \"The detailed sub-mode of the traffic signal.\","]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalSubModeEnum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TrafficSignalDynamicState {
    #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "A plan line for this traffic signal. This is intended to provide information allowing the plan data to be displayed to an operator. It contains the current plan data and would be updated when the plan is changed. For adaptive central control, such as SCOOT, the information should be available periodically and contain the plan at that instant."]
    #[serde(
        rename = "planLine",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub plan_line: ::std::option::Option<PlanLine>,
    #[doc = "Name for the sub-area that identifies the group of signals to which the associated traffic signal equipment belongs."]
    #[serde(
        rename = "subArea",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sub_area: ::std::option::Option<::std::string::String>,
    #[doc = "Time the state was observed."]
    pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Identifier of the traffic signal this reading is about."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
    #[doc = "The high level mode of the traffic signal."]
    #[serde(rename = "trafficSignalMode")]
    pub traffic_signal_mode: TrafficSignalModeEnum,
    #[doc = "The detailed sub-mode of the traffic signal."]
    #[serde(
        rename = "trafficSignalSubMode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub traffic_signal_sub_mode: ::std::option::Option<TrafficSignalSubModeEnum>,
}
impl TrafficSignalDynamicState {
    pub fn builder() -> builder::TrafficSignalDynamicState {
        Default::default()
    }
}
#[doc = "Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3, F6)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Fault\","]
#[doc = "  \"description\": \"Traffic signal fault reports with flattened DATEX II Fault base (TS004.0064bis:2017 §3.3, F6).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"faultTypeId\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"trafficSignalFaultType\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"faultLastUpdateTime\","]
#[doc = "    \"trafficSignalId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or forward-compatibility extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"faultCreationTime\": {"]
#[doc = "      \"description\": \"Time the fault was first recorded.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"faultDescription\": {"]
#[doc = "      \"description\": \"Human-readable fault description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"faultIdentifier\": {"]
#[doc = "      \"description\": \"Unique identifier for this fault.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"faultLastUpdateTime\": {"]
#[doc = "      \"description\": \"Time fault was last updated.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"faultSeverity\": {"]
#[doc = "      \"description\": \"Severity of the fault.\","]
#[doc = "      \"$ref\": \"#/$defs/FaultSeverityEnum\""]
#[doc = "    },"]
#[doc = "    \"faultTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Reference to a FaultTypeDefinition entry. The catalogue carries the code, category and message template; new fault types are added as catalogue entries, with no schema change. Required unless trafficSignalFaultType is given; when both are present this is the authoritative classification.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"parameters\": {"]
#[doc = "      \"description\": \"Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"trafficSignalFaultClearanceTime\": {"]
#[doc = "      \"description\": \"Time fault was cleared.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalFaultType\": {"]
#[doc = "      \"description\": \"Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is given; when both are present faultTypeId is the authoritative classification.\","]
#[doc = "      \"$ref\": \"#/$defs/TrafficSignalFaultEnum\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalFaultUrgency\": {"]
#[doc = "      \"description\": \"The urgency of this fault.\","]
#[doc = "      \"$ref\": \"#/$defs/UrgencyEnum\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal this fault applies to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"trafficSignalManufacturerFaultCode\": {"]
#[doc = "      \"description\": \"A supplier unique identifier for this specific fault.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum TrafficSignalFault {
    Variant0 {
        #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[doc = "Time the fault was first recorded."]
        #[serde(
            rename = "faultCreationTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_creation_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[doc = "Human-readable fault description."]
        #[serde(
            rename = "faultDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_description: ::std::option::Option<::std::string::String>,
        #[doc = "Unique identifier for this fault."]
        #[serde(
            rename = "faultIdentifier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_identifier: ::std::option::Option<::std::string::String>,
        #[doc = "Time fault was last updated."]
        #[serde(rename = "faultLastUpdateTime")]
        fault_last_update_time: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "Severity of the fault."]
        #[serde(
            rename = "faultSeverity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_severity: ::std::option::Option<FaultSeverityEnum>,
        #[doc = "Reference to a FaultTypeDefinition entry. The catalogue carries the code, category and message template; new fault types are added as catalogue entries, with no schema change. Required unless trafficSignalFaultType is given; when both are present this is the authoritative classification."]
        #[serde(rename = "faultTypeId")]
        fault_type_id: TypeId,
        #[doc = "Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in."]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        parameters: ::std::vec::Vec<::std::string::String>,
        #[doc = "Time fault was cleared."]
        #[serde(
            rename = "trafficSignalFaultClearanceTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_fault_clearance_time:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[doc = "The urgency of this fault."]
        #[serde(
            rename = "trafficSignalFaultUrgency",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_fault_urgency: ::std::option::Option<UrgencyEnum>,
        #[doc = "Identifier of the traffic signal this fault applies to."]
        #[serde(rename = "trafficSignalId")]
        traffic_signal_id: ObjectId,
        #[doc = "A supplier unique identifier for this specific fault."]
        #[serde(
            rename = "trafficSignalManufacturerFaultCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_manufacturer_fault_code: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[doc = "Vendor or forward-compatibility extensions. Open-content object."]
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[doc = "Time the fault was first recorded."]
        #[serde(
            rename = "faultCreationTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_creation_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[doc = "Human-readable fault description."]
        #[serde(
            rename = "faultDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_description: ::std::option::Option<::std::string::String>,
        #[doc = "Unique identifier for this fault."]
        #[serde(
            rename = "faultIdentifier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_identifier: ::std::option::Option<::std::string::String>,
        #[doc = "Time fault was last updated."]
        #[serde(rename = "faultLastUpdateTime")]
        fault_last_update_time: ::chrono::DateTime<::chrono::offset::Utc>,
        #[doc = "Severity of the fault."]
        #[serde(
            rename = "faultSeverity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        fault_severity: ::std::option::Option<FaultSeverityEnum>,
        #[doc = "Values for the positional placeholders in the referenced FaultTypeDefinition's messageTemplate, in order. Rendered as text so a value carries whatever form the fault reports it in."]
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        parameters: ::std::vec::Vec<::std::string::String>,
        #[doc = "Time fault was cleared."]
        #[serde(
            rename = "trafficSignalFaultClearanceTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_fault_clearance_time:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[doc = "Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required unless faultTypeId is given; when both are present faultTypeId is the authoritative classification."]
        #[serde(rename = "trafficSignalFaultType")]
        traffic_signal_fault_type: TrafficSignalFaultEnum,
        #[doc = "The urgency of this fault."]
        #[serde(
            rename = "trafficSignalFaultUrgency",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_fault_urgency: ::std::option::Option<UrgencyEnum>,
        #[doc = "Identifier of the traffic signal this fault applies to."]
        #[serde(rename = "trafficSignalId")]
        traffic_signal_id: ObjectId,
        #[doc = "A supplier unique identifier for this specific fault."]
        #[serde(
            rename = "trafficSignalManufacturerFaultCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        traffic_signal_manufacturer_fault_code: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "PDF §4.1 — traffic-signal fault types."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"PDF §4.1 — traffic-signal fault types.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"detectorFault\","]
#[doc = "    \"electronicsFault\","]
#[doc = "    \"indicatorFault\","]
#[doc = "    \"lampFault\","]
#[doc = "    \"mechanicalFault\","]
#[doc = "    \"utilityFault\","]
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
pub enum TrafficSignalFaultEnum {
    #[serde(rename = "detectorFault")]
    DetectorFault,
    #[serde(rename = "electronicsFault")]
    ElectronicsFault,
    #[serde(rename = "indicatorFault")]
    IndicatorFault,
    #[serde(rename = "lampFault")]
    LampFault,
    #[serde(rename = "mechanicalFault")]
    MechanicalFault,
    #[serde(rename = "utilityFault")]
    UtilityFault,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for TrafficSignalFaultEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DetectorFault => f.write_str("detectorFault"),
            Self::ElectronicsFault => f.write_str("electronicsFault"),
            Self::IndicatorFault => f.write_str("indicatorFault"),
            Self::LampFault => f.write_str("lampFault"),
            Self::MechanicalFault => f.write_str("mechanicalFault"),
            Self::UtilityFault => f.write_str("utilityFault"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for TrafficSignalFaultEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "detectorFault" => Ok(Self::DetectorFault),
            "electronicsFault" => Ok(Self::ElectronicsFault),
            "indicatorFault" => Ok(Self::IndicatorFault),
            "lampFault" => Ok(Self::LampFault),
            "mechanicalFault" => Ok(Self::MechanicalFault),
            "utilityFault" => Ok(Self::UtilityFault),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalFaultEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalFaultEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalFaultEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Traffic signal history entries (reused from TS004.0061:2010 Annex D)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 History\","]
#[doc = "  \"description\": \"Traffic signal history entries (reused from TS004.0061:2010 Annex D).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"logDate\","]
#[doc = "    \"logEntry\","]
#[doc = "    \"logId\","]
#[doc = "    \"trafficSignalId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"eventType\": {"]
#[doc = "      \"description\": \"Category or type of the history event (e.g. 'configuration_change', 'fault_cleared').\","]
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
#[doc = "    \"trafficSignalId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Identifier of the traffic signal this log entry is about.\","]
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
pub struct TrafficSignalHistoryEntry {
    #[doc = "Category or type of the history event (e.g. 'configuration_change', 'fault_cleared')."]
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
    #[doc = "Identifier of the traffic signal this log entry is about."]
    #[serde(rename = "trafficSignalId")]
    pub traffic_signal_id: ObjectId,
}
impl TrafficSignalHistoryEntry {
    pub fn builder() -> builder::TrafficSignalHistoryEntry {
        Default::default()
    }
}
#[doc = "PDF §4.3 — high-level traffic-signal modes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"PDF §4.3 — high-level traffic-signal modes.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"localMode\","]
#[doc = "    \"centralMode\","]
#[doc = "    \"signalsOffMode\","]
#[doc = "    \"startUpMode\","]
#[doc = "    \"unknownMode\""]
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
pub enum TrafficSignalModeEnum {
    #[serde(rename = "localMode")]
    LocalMode,
    #[serde(rename = "centralMode")]
    CentralMode,
    #[serde(rename = "signalsOffMode")]
    SignalsOffMode,
    #[serde(rename = "startUpMode")]
    StartUpMode,
    #[serde(rename = "unknownMode")]
    UnknownMode,
}
impl ::std::fmt::Display for TrafficSignalModeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LocalMode => f.write_str("localMode"),
            Self::CentralMode => f.write_str("centralMode"),
            Self::SignalsOffMode => f.write_str("signalsOffMode"),
            Self::StartUpMode => f.write_str("startUpMode"),
            Self::UnknownMode => f.write_str("unknownMode"),
        }
    }
}
impl ::std::str::FromStr for TrafficSignalModeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "localMode" => Ok(Self::LocalMode),
            "centralMode" => Ok(Self::CentralMode),
            "signalsOffMode" => Ok(Self::SignalsOffMode),
            "startUpMode" => Ok(Self::StartUpMode),
            "unknownMode" => Ok(Self::UnknownMode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "PDF §4.4 — detailed traffic-signal sub-modes.\n\nNote: §5.1's mode/subMode combinations table references additional values\n(hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,\npublicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are\nNOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"PDF §4.4 — detailed traffic-signal sub-modes.\\n\\nNote: §5.1's mode/subMode combinations table references additional values\\n(hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,\\npublicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are\\nNOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cablelessLinking\","]
#[doc = "    \"fixedModeSimpleOrLinked\","]
#[doc = "    \"fixedModeToCurrentMax\","]
#[doc = "    \"manualMode\","]
#[doc = "    \"manualStepOn\","]
#[doc = "    \"movaMode\","]
#[doc = "    \"otherDistributedMode\","]
#[doc = "    \"preEmptLevel1\","]
#[doc = "    \"preEmptLevel2\","]
#[doc = "    \"preEmptLevel3\","]
#[doc = "    \"utcFixedTime\","]
#[doc = "    \"utcOtherCentralControlMethod\","]
#[doc = "    \"utcScoot\","]
#[doc = "    \"vehicleActuated\","]
#[doc = "    \"wardenControl\""]
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
pub enum TrafficSignalSubModeEnum {
    #[serde(rename = "cablelessLinking")]
    CablelessLinking,
    #[serde(rename = "fixedModeSimpleOrLinked")]
    FixedModeSimpleOrLinked,
    #[serde(rename = "fixedModeToCurrentMax")]
    FixedModeToCurrentMax,
    #[serde(rename = "manualMode")]
    ManualMode,
    #[serde(rename = "manualStepOn")]
    ManualStepOn,
    #[serde(rename = "movaMode")]
    MovaMode,
    #[serde(rename = "otherDistributedMode")]
    OtherDistributedMode,
    #[serde(rename = "preEmptLevel1")]
    PreEmptLevel1,
    #[serde(rename = "preEmptLevel2")]
    PreEmptLevel2,
    #[serde(rename = "preEmptLevel3")]
    PreEmptLevel3,
    #[serde(rename = "utcFixedTime")]
    UtcFixedTime,
    #[serde(rename = "utcOtherCentralControlMethod")]
    UtcOtherCentralControlMethod,
    #[serde(rename = "utcScoot")]
    UtcScoot,
    #[serde(rename = "vehicleActuated")]
    VehicleActuated,
    #[serde(rename = "wardenControl")]
    WardenControl,
}
impl ::std::fmt::Display for TrafficSignalSubModeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CablelessLinking => f.write_str("cablelessLinking"),
            Self::FixedModeSimpleOrLinked => f.write_str("fixedModeSimpleOrLinked"),
            Self::FixedModeToCurrentMax => f.write_str("fixedModeToCurrentMax"),
            Self::ManualMode => f.write_str("manualMode"),
            Self::ManualStepOn => f.write_str("manualStepOn"),
            Self::MovaMode => f.write_str("movaMode"),
            Self::OtherDistributedMode => f.write_str("otherDistributedMode"),
            Self::PreEmptLevel1 => f.write_str("preEmptLevel1"),
            Self::PreEmptLevel2 => f.write_str("preEmptLevel2"),
            Self::PreEmptLevel3 => f.write_str("preEmptLevel3"),
            Self::UtcFixedTime => f.write_str("utcFixedTime"),
            Self::UtcOtherCentralControlMethod => f.write_str("utcOtherCentralControlMethod"),
            Self::UtcScoot => f.write_str("utcScoot"),
            Self::VehicleActuated => f.write_str("vehicleActuated"),
            Self::WardenControl => f.write_str("wardenControl"),
        }
    }
}
impl ::std::str::FromStr for TrafficSignalSubModeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cablelessLinking" => Ok(Self::CablelessLinking),
            "fixedModeSimpleOrLinked" => Ok(Self::FixedModeSimpleOrLinked),
            "fixedModeToCurrentMax" => Ok(Self::FixedModeToCurrentMax),
            "manualMode" => Ok(Self::ManualMode),
            "manualStepOn" => Ok(Self::ManualStepOn),
            "movaMode" => Ok(Self::MovaMode),
            "otherDistributedMode" => Ok(Self::OtherDistributedMode),
            "preEmptLevel1" => Ok(Self::PreEmptLevel1),
            "preEmptLevel2" => Ok(Self::PreEmptLevel2),
            "preEmptLevel3" => Ok(Self::PreEmptLevel3),
            "utcFixedTime" => Ok(Self::UtcFixedTime),
            "utcOtherCentralControlMethod" => Ok(Self::UtcOtherCentralControlMethod),
            "utcScoot" => Ok(Self::UtcScoot),
            "vehicleActuated" => Ok(Self::VehicleActuated),
            "wardenControl" => Ok(Self::WardenControl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalSubModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalSubModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalSubModeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Traffic signal type definitions (reused from TS004.0061:2010 Annex D)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Traffic Signal 0064 Type\","]
#[doc = "  \"description\": \"Traffic signal type definitions (reused from TS004.0061:2010 Annex D).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"typeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description of the control method (e.g. 'MOVA', 'SCOOT', 'fixed-time', 'vehicle-actuated').\","]
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
#[doc = "      \"description\": \"URL to an external specification document for this control method.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this control method. Referenced by TrafficSignalDefinition.controlMethodId.\","]
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
pub struct TrafficSignalType {
    #[doc = "Human-readable description of the control method (e.g. 'MOVA', 'SCOOT', 'fixed-time', 'vehicle-actuated')."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Additional notes, e.g. deployment details."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
    #[doc = "URL to an external specification document for this control method."]
    #[serde(
        rename = "specificationLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub specification_link: ::std::option::Option<TrafficSignalTypeSpecificationLink>,
    #[doc = "Identifier of this control method. Referenced by TrafficSignalDefinition.controlMethodId."]
    #[serde(rename = "typeId")]
    pub type_id: TypeId,
}
impl TrafficSignalType {
    pub fn builder() -> builder::TrafficSignalType {
        Default::default()
    }
}
#[doc = "PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"intersection\","]
#[doc = "    \"pegasus\","]
#[doc = "    \"pelican\","]
#[doc = "    \"puffin\","]
#[doc = "    \"toucan\","]
#[doc = "    \"tramCrossing\","]
#[doc = "    \"wigWag\","]
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
pub enum TrafficSignalTypeEnum {
    #[serde(rename = "intersection")]
    Intersection,
    #[serde(rename = "pegasus")]
    Pegasus,
    #[serde(rename = "pelican")]
    Pelican,
    #[serde(rename = "puffin")]
    Puffin,
    #[serde(rename = "toucan")]
    Toucan,
    #[serde(rename = "tramCrossing")]
    TramCrossing,
    #[serde(rename = "wigWag")]
    WigWag,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for TrafficSignalTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Intersection => f.write_str("intersection"),
            Self::Pegasus => f.write_str("pegasus"),
            Self::Pelican => f.write_str("pelican"),
            Self::Puffin => f.write_str("puffin"),
            Self::Toucan => f.write_str("toucan"),
            Self::TramCrossing => f.write_str("tramCrossing"),
            Self::WigWag => f.write_str("wigWag"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for TrafficSignalTypeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "intersection" => Ok(Self::Intersection),
            "pegasus" => Ok(Self::Pegasus),
            "pelican" => Ok(Self::Pelican),
            "puffin" => Ok(Self::Puffin),
            "toucan" => Ok(Self::Toucan),
            "tramCrossing" => Ok(Self::TramCrossing),
            "wigWag" => Ok(Self::WigWag),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalTypeEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "URL to an external specification document for this control method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL to an external specification document for this control method.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TrafficSignalTypeSpecificationLink(::std::string::String);
impl ::std::ops::Deref for TrafficSignalTypeSpecificationLink {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TrafficSignalTypeSpecificationLink> for ::std::string::String {
    fn from(value: TrafficSignalTypeSpecificationLink) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TrafficSignalTypeSpecificationLink {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TrafficSignalTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TrafficSignalTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TrafficSignalTypeSpecificationLink {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TrafficSignalTypeSpecificationLink {
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
#[doc = "DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF §3.3).\n\n``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a\npublisher that cannot determine urgency must be able to say so rather\nthan defaulting to ``normalUrgency`` and understating a fault. This\nmirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF §3.3).\\n\\n``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a\\npublisher that cannot determine urgency must be able to say so rather\\nthan defaulting to ``normalUrgency`` and understating a fault. This\\nmirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"extremelyUrgent\","]
#[doc = "    \"urgent\","]
#[doc = "    \"normalUrgency\","]
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
pub enum UrgencyEnum {
    #[serde(rename = "extremelyUrgent")]
    ExtremelyUrgent,
    #[serde(rename = "urgent")]
    Urgent,
    #[serde(rename = "normalUrgency")]
    NormalUrgency,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for UrgencyEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExtremelyUrgent => f.write_str("extremelyUrgent"),
            Self::Urgent => f.write_str("urgent"),
            Self::NormalUrgency => f.write_str("normalUrgency"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for UrgencyEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "extremelyUrgent" => Ok(Self::ExtremelyUrgent),
            "urgent" => Ok(Self::Urgent),
            "normalUrgency" => Ok(Self::NormalUrgency),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UrgencyEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UrgencyEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UrgencyEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
        fault_severity_enum: ::std::result::Result<
            ::std::option::Option<super::FaultSeverityEnum>,
            ::std::string::String,
        >,
        force_stage_command: ::std::result::Result<
            ::std::option::Option<super::ForceStageCommand>,
            ::std::string::String,
        >,
        geo_json_point: ::std::result::Result<
            ::std::option::Option<super::GeoJsonPoint>,
            ::std::string::String,
        >,
        hold_stage_command: ::std::result::Result<
            ::std::option::Option<super::HoldStageCommand>,
            ::std::string::String,
        >,
        plan_line:
            ::std::result::Result<::std::option::Option<super::PlanLine>, ::std::string::String>,
        request_status_command: ::std::result::Result<
            ::std::option::Option<super::RequestStatusCommand>,
            ::std::string::String,
        >,
        set_plan_command: ::std::result::Result<
            ::std::option::Option<super::SetPlanCommand>,
            ::std::string::String,
        >,
        signal_group_timings: ::std::result::Result<
            ::std::option::Option<super::SignalGroupTimings>,
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
        synchronize_clock_command: ::std::result::Result<
            ::std::option::Option<super::SynchronizeClockCommand>,
            ::std::string::String,
        >,
        traffic_signal_command: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalCommand>,
            ::std::string::String,
        >,
        traffic_signal_command_response: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalCommandResponse>,
            ::std::string::String,
        >,
        traffic_signal_configuration: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalConfiguration>,
            ::std::string::String,
        >,
        traffic_signal_definition: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalDefinition>,
            ::std::string::String,
        >,
        traffic_signal_dynamic_state: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalDynamicState>,
            ::std::string::String,
        >,
        traffic_signal_fault: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalFault>,
            ::std::string::String,
        >,
        traffic_signal_fault_enum: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalFaultEnum>,
            ::std::string::String,
        >,
        traffic_signal_history_entry: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalHistoryEntry>,
            ::std::string::String,
        >,
        traffic_signal_mode_enum: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalModeEnum>,
            ::std::string::String,
        >,
        traffic_signal_sub_mode_enum: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalSubModeEnum>,
            ::std::string::String,
        >,
        traffic_signal_type: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalType>,
            ::std::string::String,
        >,
        traffic_signal_type_enum: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalTypeEnum>,
            ::std::string::String,
        >,
        urgency_enum:
            ::std::result::Result<::std::option::Option<super::UrgencyEnum>, ::std::string::String>,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                command_response_status: Ok(Default::default()),
                fault_severity_enum: Ok(Default::default()),
                force_stage_command: Ok(Default::default()),
                geo_json_point: Ok(Default::default()),
                hold_stage_command: Ok(Default::default()),
                plan_line: Ok(Default::default()),
                request_status_command: Ok(Default::default()),
                set_plan_command: Ok(Default::default()),
                signal_group_timings: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
                synchronize_clock_command: Ok(Default::default()),
                traffic_signal_command: Ok(Default::default()),
                traffic_signal_command_response: Ok(Default::default()),
                traffic_signal_configuration: Ok(Default::default()),
                traffic_signal_definition: Ok(Default::default()),
                traffic_signal_dynamic_state: Ok(Default::default()),
                traffic_signal_fault: Ok(Default::default()),
                traffic_signal_fault_enum: Ok(Default::default()),
                traffic_signal_history_entry: Ok(Default::default()),
                traffic_signal_mode_enum: Ok(Default::default()),
                traffic_signal_sub_mode_enum: Ok(Default::default()),
                traffic_signal_type: Ok(Default::default()),
                traffic_signal_type_enum: Ok(Default::default()),
                urgency_enum: Ok(Default::default()),
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
        pub fn fault_severity_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultSeverityEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_severity_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for fault_severity_enum: {e}")
            });
            self
        }
        pub fn force_stage_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ForceStageCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.force_stage_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for force_stage_command: {e}")
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
        pub fn hold_stage_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HoldStageCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.hold_stage_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for hold_stage_command: {e}")
            });
            self
        }
        pub fn plan_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlanLine>>,
            T::Error: ::std::fmt::Display,
        {
            self.plan_line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_line: {e}"));
            self
        }
        pub fn request_status_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RequestStatusCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_status_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for request_status_command: {e}")
            });
            self
        }
        pub fn set_plan_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SetPlanCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.set_plan_command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for set_plan_command: {e}"));
            self
        }
        pub fn signal_group_timings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SignalGroupTimings>>,
            T::Error: ::std::fmt::Display,
        {
            self.signal_group_timings = value.try_into().map_err(|e| {
                format!("error converting supplied value for signal_group_timings: {e}")
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
        pub fn synchronize_clock_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SynchronizeClockCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.synchronize_clock_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for synchronize_clock_command: {e}")
            });
            self
        }
        pub fn traffic_signal_command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalCommand>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_command = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_command: {e}")
            });
            self
        }
        pub fn traffic_signal_command_response<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalCommandResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_command_response = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_command_response: {e}")
            });
            self
        }
        pub fn traffic_signal_configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_configuration = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_configuration: {e}")
            });
            self
        }
        pub fn traffic_signal_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_definition: {e}")
            });
            self
        }
        pub fn traffic_signal_dynamic_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalDynamicState>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_dynamic_state = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_dynamic_state: {e}")
            });
            self
        }
        pub fn traffic_signal_fault<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalFault>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_fault = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_fault: {e}")
            });
            self
        }
        pub fn traffic_signal_fault_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalFaultEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_fault_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_fault_enum: {e}")
            });
            self
        }
        pub fn traffic_signal_history_entry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalHistoryEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_history_entry = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_history_entry: {e}")
            });
            self
        }
        pub fn traffic_signal_mode_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalModeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_mode_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_mode_enum: {e}")
            });
            self
        }
        pub fn traffic_signal_sub_mode_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalSubModeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_sub_mode_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_sub_mode_enum: {e}")
            });
            self
        }
        pub fn traffic_signal_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalType>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_type: {e}")
            });
            self
        }
        pub fn traffic_signal_type_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalTypeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_type_enum = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_type_enum: {e}")
            });
            self
        }
        pub fn urgency_enum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UrgencyEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.urgency_enum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urgency_enum: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_response_status: value.command_response_status?,
                fault_severity_enum: value.fault_severity_enum?,
                force_stage_command: value.force_stage_command?,
                geo_json_point: value.geo_json_point?,
                hold_stage_command: value.hold_stage_command?,
                plan_line: value.plan_line?,
                request_status_command: value.request_status_command?,
                set_plan_command: value.set_plan_command?,
                signal_group_timings: value.signal_group_timings?,
                sync_category_enum: value.sync_category_enum?,
                sync_complete: value.sync_complete?,
                sync_request: value.sync_request?,
                sync_status: value.sync_status?,
                synchronize_clock_command: value.synchronize_clock_command?,
                traffic_signal_command: value.traffic_signal_command?,
                traffic_signal_command_response: value.traffic_signal_command_response?,
                traffic_signal_configuration: value.traffic_signal_configuration?,
                traffic_signal_definition: value.traffic_signal_definition?,
                traffic_signal_dynamic_state: value.traffic_signal_dynamic_state?,
                traffic_signal_fault: value.traffic_signal_fault?,
                traffic_signal_fault_enum: value.traffic_signal_fault_enum?,
                traffic_signal_history_entry: value.traffic_signal_history_entry?,
                traffic_signal_mode_enum: value.traffic_signal_mode_enum?,
                traffic_signal_sub_mode_enum: value.traffic_signal_sub_mode_enum?,
                traffic_signal_type: value.traffic_signal_type?,
                traffic_signal_type_enum: value.traffic_signal_type_enum?,
                urgency_enum: value.urgency_enum?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                command_response_status: Ok(value.command_response_status),
                fault_severity_enum: Ok(value.fault_severity_enum),
                force_stage_command: Ok(value.force_stage_command),
                geo_json_point: Ok(value.geo_json_point),
                hold_stage_command: Ok(value.hold_stage_command),
                plan_line: Ok(value.plan_line),
                request_status_command: Ok(value.request_status_command),
                set_plan_command: Ok(value.set_plan_command),
                signal_group_timings: Ok(value.signal_group_timings),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
                synchronize_clock_command: Ok(value.synchronize_clock_command),
                traffic_signal_command: Ok(value.traffic_signal_command),
                traffic_signal_command_response: Ok(value.traffic_signal_command_response),
                traffic_signal_configuration: Ok(value.traffic_signal_configuration),
                traffic_signal_definition: Ok(value.traffic_signal_definition),
                traffic_signal_dynamic_state: Ok(value.traffic_signal_dynamic_state),
                traffic_signal_fault: Ok(value.traffic_signal_fault),
                traffic_signal_fault_enum: Ok(value.traffic_signal_fault_enum),
                traffic_signal_history_entry: Ok(value.traffic_signal_history_entry),
                traffic_signal_mode_enum: Ok(value.traffic_signal_mode_enum),
                traffic_signal_sub_mode_enum: Ok(value.traffic_signal_sub_mode_enum),
                traffic_signal_type: Ok(value.traffic_signal_type),
                traffic_signal_type_enum: Ok(value.traffic_signal_type_enum),
                urgency_enum: Ok(value.urgency_enum),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ForceStageCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::ForceStageCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        stage_number: ::std::result::Result<u64, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for ForceStageCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                stage_number: Err("no value supplied for stage_number".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl ForceStageCommand {
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
            T: ::std::convert::TryInto<super::ForceStageCommandKind>,
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
        pub fn stage_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.stage_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stage_number: {e}"));
            self
        }
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ForceStageCommand> for super::ForceStageCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ForceStageCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                stage_number: value.stage_number?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::ForceStageCommand> for ForceStageCommand {
        fn from(value: super::ForceStageCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                stage_number: Ok(value.stage_number),
                traffic_signal_id: Ok(value.traffic_signal_id),
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
    pub struct HoldStageCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::HoldStageCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        stage_number: ::std::result::Result<u64, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for HoldStageCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                stage_number: Err("no value supplied for stage_number".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl HoldStageCommand {
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
            T: ::std::convert::TryInto<super::HoldStageCommandKind>,
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
        pub fn stage_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.stage_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stage_number: {e}"));
            self
        }
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HoldStageCommand> for super::HoldStageCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HoldStageCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                stage_number: value.stage_number?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::HoldStageCommand> for HoldStageCommand {
        fn from(value: super::HoldStageCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                stage_number: Ok(value.stage_number),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PlanLine {
        cycle_time: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        offset: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        plan_description: ::std::result::Result<
            ::std::option::Option<super::PlanLinePlanDescription>,
            ::std::string::String,
        >,
        plan_number: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        plan_start_time: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        signal_group_timings: ::std::result::Result<
            ::std::vec::Vec<super::SignalGroupTimings>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PlanLine {
        fn default() -> Self {
            Self {
                cycle_time: Ok(Default::default()),
                extensions: Ok(Default::default()),
                offset: Ok(Default::default()),
                plan_description: Ok(Default::default()),
                plan_number: Err("no value supplied for plan_number".to_string()),
                plan_start_time: Ok(Default::default()),
                signal_group_timings: Ok(Default::default()),
            }
        }
    }
    impl PlanLine {
        pub fn cycle_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cycle_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cycle_time: {e}"));
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
        pub fn offset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.offset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for offset: {e}"));
            self
        }
        pub fn plan_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlanLinePlanDescription>>,
            T::Error: ::std::fmt::Display,
        {
            self.plan_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_description: {e}"));
            self
        }
        pub fn plan_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.plan_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_number: {e}"));
            self
        }
        pub fn plan_start_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.plan_start_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_start_time: {e}"));
            self
        }
        pub fn signal_group_timings<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SignalGroupTimings>>,
            T::Error: ::std::fmt::Display,
        {
            self.signal_group_timings = value.try_into().map_err(|e| {
                format!("error converting supplied value for signal_group_timings: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PlanLine> for super::PlanLine {
        type Error = super::error::ConversionError;
        fn try_from(value: PlanLine) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cycle_time: value.cycle_time?,
                extensions: value.extensions?,
                offset: value.offset?,
                plan_description: value.plan_description?,
                plan_number: value.plan_number?,
                plan_start_time: value.plan_start_time?,
                signal_group_timings: value.signal_group_timings?,
            })
        }
    }
    impl ::std::convert::From<super::PlanLine> for PlanLine {
        fn from(value: super::PlanLine) -> Self {
            Self {
                cycle_time: Ok(value.cycle_time),
                extensions: Ok(value.extensions),
                offset: Ok(value.offset),
                plan_description: Ok(value.plan_description),
                plan_number: Ok(value.plan_number),
                plan_start_time: Ok(value.plan_start_time),
                signal_group_timings: Ok(value.signal_group_timings),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RequestStatusCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::RequestStatusCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for RequestStatusCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl RequestStatusCommand {
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
            T: ::std::convert::TryInto<super::RequestStatusCommandKind>,
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
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RequestStatusCommand> for super::RequestStatusCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RequestStatusCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::RequestStatusCommand> for RequestStatusCommand {
        fn from(value: super::RequestStatusCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetPlanCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SetPlanCommandKind, ::std::string::String>,
        plan_number: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for SetPlanCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                plan_number: Err("no value supplied for plan_number".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl SetPlanCommand {
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
            T: ::std::convert::TryInto<super::SetPlanCommandKind>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn plan_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.plan_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_number: {e}"));
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
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SetPlanCommand> for super::SetPlanCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetPlanCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                plan_number: value.plan_number?,
                request_time: value.request_time?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::SetPlanCommand> for SetPlanCommand {
        fn from(value: super::SetPlanCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                plan_number: Ok(value.plan_number),
                request_time: Ok(value.request_time),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SignalGroupTimings {
        alternate_signal_group: ::std::result::Result<
            ::std::option::Option<super::SignalGroupTimingsAlternateSignalGroup>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        primary_signal_group: ::std::result::Result<
            super::SignalGroupTimingsPrimarySignalGroup,
            ::std::string::String,
        >,
        primary_signal_group_duration: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for SignalGroupTimings {
        fn default() -> Self {
            Self {
                alternate_signal_group: Ok(Default::default()),
                extensions: Ok(Default::default()),
                primary_signal_group: Err("no value supplied for primary_signal_group".to_string()),
                primary_signal_group_duration: Err(
                    "no value supplied for primary_signal_group_duration".to_string(),
                ),
            }
        }
    }
    impl SignalGroupTimings {
        pub fn alternate_signal_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SignalGroupTimingsAlternateSignalGroup>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.alternate_signal_group = value.try_into().map_err(|e| {
                format!("error converting supplied value for alternate_signal_group: {e}")
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
        pub fn primary_signal_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SignalGroupTimingsPrimarySignalGroup>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_signal_group = value.try_into().map_err(|e| {
                format!("error converting supplied value for primary_signal_group: {e}")
            });
            self
        }
        pub fn primary_signal_group_duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_signal_group_duration = value.try_into().map_err(|e| {
                format!("error converting supplied value for primary_signal_group_duration: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SignalGroupTimings> for super::SignalGroupTimings {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SignalGroupTimings,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternate_signal_group: value.alternate_signal_group?,
                extensions: value.extensions?,
                primary_signal_group: value.primary_signal_group?,
                primary_signal_group_duration: value.primary_signal_group_duration?,
            })
        }
    }
    impl ::std::convert::From<super::SignalGroupTimings> for SignalGroupTimings {
        fn from(value: super::SignalGroupTimings) -> Self {
            Self {
                alternate_signal_group: Ok(value.alternate_signal_group),
                extensions: Ok(value.extensions),
                primary_signal_group: Ok(value.primary_signal_group),
                primary_signal_group_duration: Ok(value.primary_signal_group_duration),
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
    pub struct SynchronizeClockCommand {
        command_correlation_id: ::std::result::Result<i64, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<super::SynchronizeClockCommandKind, ::std::string::String>,
        request_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        target_time:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for SynchronizeClockCommand {
        fn default() -> Self {
            Self {
                command_correlation_id: Err(
                    "no value supplied for command_correlation_id".to_string()
                ),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                request_time: Err("no value supplied for request_time".to_string()),
                target_time: Err("no value supplied for target_time".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl SynchronizeClockCommand {
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
            T: ::std::convert::TryInto<super::SynchronizeClockCommandKind>,
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
        pub fn target_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.target_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_time: {e}"));
            self
        }
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SynchronizeClockCommand> for super::SynchronizeClockCommand {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SynchronizeClockCommand,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                request_time: value.request_time?,
                target_time: value.target_time?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::SynchronizeClockCommand> for SynchronizeClockCommand {
        fn from(value: super::SynchronizeClockCommand) -> Self {
            Self {
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                request_time: Ok(value.request_time),
                target_time: Ok(value.target_time),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalCommandResponse {
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
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TrafficSignalCommandResponse {
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
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl TrafficSignalCommandResponse {
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
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TrafficSignalCommandResponse> for super::TrafficSignalCommandResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalCommandResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                command_correlation_id: value.command_correlation_id?,
                extensions: value.extensions?,
                message: value.message?,
                response_time: value.response_time?,
                status: value.status?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::TrafficSignalCommandResponse> for TrafficSignalCommandResponse {
        fn from(value: super::TrafficSignalCommandResponse) -> Self {
            Self {
                code: Ok(value.code),
                command_correlation_id: Ok(value.command_correlation_id),
                extensions: Ok(value.extensions),
                message: Ok(value.message),
                response_time: Ok(value.response_time),
                status: Ok(value.status),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalConfiguration {
        configuration_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        controller_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        sub_area: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        supplier: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        traffic_signal_type: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalTypeEnum>,
            ::std::string::String,
        >,
        user_code_number: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalConfigurationUserCodeNumber>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TrafficSignalConfiguration {
        fn default() -> Self {
            Self {
                configuration_date: Ok(Default::default()),
                controller_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                sub_area: Ok(Default::default()),
                supplier: Ok(Default::default()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
                traffic_signal_type: Ok(Default::default()),
                user_code_number: Ok(Default::default()),
            }
        }
    }
    impl TrafficSignalConfiguration {
        pub fn configuration_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.configuration_date = value.try_into().map_err(|e| {
                format!("error converting supplied value for configuration_date: {e}")
            });
            self
        }
        pub fn controller_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.controller_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for controller_type: {e}"));
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
        pub fn sub_area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub_area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_area: {e}"));
            self
        }
        pub fn supplier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supplier = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supplier: {e}"));
            self
        }
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
        pub fn traffic_signal_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalTypeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_type = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_type: {e}")
            });
            self
        }
        pub fn user_code_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TrafficSignalConfigurationUserCodeNumber>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.user_code_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_code_number: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TrafficSignalConfiguration> for super::TrafficSignalConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration_date: value.configuration_date?,
                controller_type: value.controller_type?,
                extensions: value.extensions?,
                sub_area: value.sub_area?,
                supplier: value.supplier?,
                traffic_signal_id: value.traffic_signal_id?,
                traffic_signal_type: value.traffic_signal_type?,
                user_code_number: value.user_code_number?,
            })
        }
    }
    impl ::std::convert::From<super::TrafficSignalConfiguration> for TrafficSignalConfiguration {
        fn from(value: super::TrafficSignalConfiguration) -> Self {
            Self {
                configuration_date: Ok(value.configuration_date),
                controller_type: Ok(value.controller_type),
                extensions: Ok(value.extensions),
                sub_area: Ok(value.sub_area),
                supplier: Ok(value.supplier),
                traffic_signal_id: Ok(value.traffic_signal_id),
                traffic_signal_type: Ok(value.traffic_signal_type),
                user_code_number: Ok(value.user_code_number),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalDefinition {
        control_method_id:
            ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
        creation_date: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        location: ::std::result::Result<
            ::std::option::Option<super::GeoJsonPoint>,
            ::std::string::String,
        >,
        long_description: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalDefinitionLongDescription>,
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
            ::std::option::Option<super::TrafficSignalDefinitionShortDescription>,
            ::std::string::String,
        >,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TrafficSignalDefinition {
        fn default() -> Self {
            Self {
                control_method_id: Ok(Default::default()),
                creation_date: Ok(Default::default()),
                extensions: Ok(Default::default()),
                location: Ok(Default::default()),
                long_description: Ok(Default::default()),
                originating_object_id: Ok(Default::default()),
                originating_participant_id: Ok(Default::default()),
                participant_id: Ok(Default::default()),
                short_description: Ok(Default::default()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl TrafficSignalDefinition {
        pub fn control_method_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TypeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.control_method_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for control_method_id: {e}"));
            self
        }
        pub fn creation_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
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
        pub fn long_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::TrafficSignalDefinitionLongDescription>,
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
                ::std::option::Option<super::TrafficSignalDefinitionShortDescription>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.short_description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_description: {e}"));
            self
        }
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TrafficSignalDefinition> for super::TrafficSignalDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                control_method_id: value.control_method_id?,
                creation_date: value.creation_date?,
                extensions: value.extensions?,
                location: value.location?,
                long_description: value.long_description?,
                originating_object_id: value.originating_object_id?,
                originating_participant_id: value.originating_participant_id?,
                participant_id: value.participant_id?,
                short_description: value.short_description?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::TrafficSignalDefinition> for TrafficSignalDefinition {
        fn from(value: super::TrafficSignalDefinition) -> Self {
            Self {
                control_method_id: Ok(value.control_method_id),
                creation_date: Ok(value.creation_date),
                extensions: Ok(value.extensions),
                location: Ok(value.location),
                long_description: Ok(value.long_description),
                originating_object_id: Ok(value.originating_object_id),
                originating_participant_id: Ok(value.originating_participant_id),
                participant_id: Ok(value.participant_id),
                short_description: Ok(value.short_description),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalDynamicState {
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        plan_line:
            ::std::result::Result<::std::option::Option<super::PlanLine>, ::std::string::String>,
        sub_area: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        timestamp:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        traffic_signal_mode:
            ::std::result::Result<super::TrafficSignalModeEnum, ::std::string::String>,
        traffic_signal_sub_mode: ::std::result::Result<
            ::std::option::Option<super::TrafficSignalSubModeEnum>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TrafficSignalDynamicState {
        fn default() -> Self {
            Self {
                extensions: Ok(Default::default()),
                plan_line: Ok(Default::default()),
                sub_area: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
                traffic_signal_mode: Err("no value supplied for traffic_signal_mode".to_string()),
                traffic_signal_sub_mode: Ok(Default::default()),
            }
        }
    }
    impl TrafficSignalDynamicState {
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
        pub fn plan_line<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PlanLine>>,
            T::Error: ::std::fmt::Display,
        {
            self.plan_line = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for plan_line: {e}"));
            self
        }
        pub fn sub_area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.sub_area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_area: {e}"));
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
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
        pub fn traffic_signal_mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TrafficSignalModeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_mode = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_mode: {e}")
            });
            self
        }
        pub fn traffic_signal_sub_mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TrafficSignalSubModeEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_sub_mode = value.try_into().map_err(|e| {
                format!("error converting supplied value for traffic_signal_sub_mode: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TrafficSignalDynamicState> for super::TrafficSignalDynamicState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalDynamicState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extensions: value.extensions?,
                plan_line: value.plan_line?,
                sub_area: value.sub_area?,
                timestamp: value.timestamp?,
                traffic_signal_id: value.traffic_signal_id?,
                traffic_signal_mode: value.traffic_signal_mode?,
                traffic_signal_sub_mode: value.traffic_signal_sub_mode?,
            })
        }
    }
    impl ::std::convert::From<super::TrafficSignalDynamicState> for TrafficSignalDynamicState {
        fn from(value: super::TrafficSignalDynamicState) -> Self {
            Self {
                extensions: Ok(value.extensions),
                plan_line: Ok(value.plan_line),
                sub_area: Ok(value.sub_area),
                timestamp: Ok(value.timestamp),
                traffic_signal_id: Ok(value.traffic_signal_id),
                traffic_signal_mode: Ok(value.traffic_signal_mode),
                traffic_signal_sub_mode: Ok(value.traffic_signal_sub_mode),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalHistoryEntry {
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
        traffic_signal_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for TrafficSignalHistoryEntry {
        fn default() -> Self {
            Self {
                event_type: Ok(Default::default()),
                extensions: Ok(Default::default()),
                log_date: Err("no value supplied for log_date".to_string()),
                log_entry: Err("no value supplied for log_entry".to_string()),
                log_id: Err("no value supplied for log_id".to_string()),
                log_user: Ok(Default::default()),
                traffic_signal_id: Err("no value supplied for traffic_signal_id".to_string()),
            }
        }
    }
    impl TrafficSignalHistoryEntry {
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
        pub fn traffic_signal_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.traffic_signal_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traffic_signal_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TrafficSignalHistoryEntry> for super::TrafficSignalHistoryEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalHistoryEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_type: value.event_type?,
                extensions: value.extensions?,
                log_date: value.log_date?,
                log_entry: value.log_entry?,
                log_id: value.log_id?,
                log_user: value.log_user?,
                traffic_signal_id: value.traffic_signal_id?,
            })
        }
    }
    impl ::std::convert::From<super::TrafficSignalHistoryEntry> for TrafficSignalHistoryEntry {
        fn from(value: super::TrafficSignalHistoryEntry) -> Self {
            Self {
                event_type: Ok(value.event_type),
                extensions: Ok(value.extensions),
                log_date: Ok(value.log_date),
                log_entry: Ok(value.log_entry),
                log_id: Ok(value.log_id),
                log_user: Ok(value.log_user),
                traffic_signal_id: Ok(value.traffic_signal_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrafficSignalType {
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
            ::std::option::Option<super::TrafficSignalTypeSpecificationLink>,
            ::std::string::String,
        >,
        type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
    }
    impl ::std::default::Default for TrafficSignalType {
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
    impl TrafficSignalType {
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
                ::std::option::Option<super::TrafficSignalTypeSpecificationLink>,
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
    impl ::std::convert::TryFrom<TrafficSignalType> for super::TrafficSignalType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TrafficSignalType,
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
    impl ::std::convert::From<super::TrafficSignalType> for TrafficSignalType {
        fn from(value: super::TrafficSignalType) -> Self {
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
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn geo_json_point_type() -> super::GeoJsonPointType {
        super::GeoJsonPointType::Point
    }
}
