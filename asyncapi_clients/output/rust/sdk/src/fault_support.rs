/*
 * File:        output/rust/sdk/src/fault_support.rs
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
#[doc = "    \"FaultCategory\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultCategory\""]
#[doc = "    },"]
#[doc = "    \"FaultStatus\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultStatus\""]
#[doc = "    },"]
#[doc = "    \"FaultTypeDefinition\": {"]
#[doc = "      \"$ref\": \"#/$defs/FaultTypeDefinition\""]
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
        rename = "FaultCategory",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_category: ::std::option::Option<FaultCategory>,
    #[serde(
        rename = "FaultStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_status: ::std::option::Option<FaultStatus>,
    #[serde(
        rename = "FaultTypeDefinition",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fault_type_definition: ::std::option::Option<FaultTypeDefinition>,
}
impl ::std::default::Default for AllTypes {
    fn default() -> Self {
        Self {
            acknowledgement_state: Default::default(),
            fault_category: Default::default(),
            fault_status: Default::default(),
            fault_type_definition: Default::default(),
        }
    }
}
impl AllTypes {
    pub fn builder() -> builder::AllTypes {
        Default::default()
    }
}
#[doc = "Coarse grouping for catalogue entries.\n\nDeliberately short. It exists so an operator can filter \"show me the\ncommunications faults\" without the catalogue having to agree on a\ntaxonomy first; the precise meaning of a fault is its catalogue entry,\nnot its category."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Coarse grouping for catalogue entries.\\n\\nDeliberately short. It exists so an operator can filter \\\"show me the\\ncommunications faults\\\" without the catalogue having to agree on a\\ntaxonomy first; the precise meaning of a fault is its catalogue entry,\\nnot its category.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"hardware\","]
#[doc = "    \"communications\","]
#[doc = "    \"signalTimings\","]
#[doc = "    \"environmental\","]
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
pub enum FaultCategory {
    #[serde(rename = "hardware")]
    Hardware,
    #[serde(rename = "communications")]
    Communications,
    #[serde(rename = "signalTimings")]
    SignalTimings,
    #[serde(rename = "environmental")]
    Environmental,
    #[serde(rename = "other")]
    Other,
}
impl ::std::fmt::Display for FaultCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Hardware => f.write_str("hardware"),
            Self::Communications => f.write_str("communications"),
            Self::SignalTimings => f.write_str("signalTimings"),
            Self::Environmental => f.write_str("environmental"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for FaultCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "hardware" => Ok(Self::Hardware),
            "communications" => Ok(Self::Communications),
            "signalTimings" => Ok(Self::SignalTimings),
            "environmental" => Ok(Self::Environmental),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FaultCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FaultCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FaultCategory {
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
#[doc = "Catalogue entry describing one kind of fault that can be reported."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Catalogue entry describing one kind of fault that can be reported.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"faultTypeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"category\": {"]
#[doc = "      \"description\": \"Coarse grouping, for filtering rather than semantics.\","]
#[doc = "      \"$ref\": \"#/$defs/FaultCategory\""]
#[doc = "    },"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The identifier this fault carries in its source document or vendor documentation — 'A1', 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may hold two entries with the same code from different vendors.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description of the fault type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions. Open-content object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"faultTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Identifier of this fault type.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"messageTemplate\": {"]
#[doc = "      \"description\": \"Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its parameters field. This is what lets a catalogue carry faults whose meaning depends on runtime values — the stuck-stage and interstage timing faults that a closed enum cannot express.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"parameterNames\": {"]
#[doc = "      \"description\": \"Names for the positional placeholders in messageTemplate, in order — ['stageNumber', 'actualSeconds', 'minimumSeconds']. Lets a consumer label the values rather than only interpolate them. Length SHOULD match the number of placeholders.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"Numeric severity (higher = more severe). Optional.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"subsystemTypeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Reference to a SubsystemDefinition entry: the subsystem this fault type belongs to.\","]
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
pub struct FaultTypeDefinition {
    #[doc = "Coarse grouping, for filtering rather than semantics."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<FaultCategory>,
    #[doc = "The identifier this fault carries in its source document or vendor documentation — 'A1', 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may hold two entries with the same code from different vendors."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<FaultTypeDefinitionCode>,
    #[doc = "Human-readable description of the fault type."]
    pub description: ::std::string::String,
    #[doc = "Vendor or local extensions. Open-content object."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Identifier of this fault type."]
    #[serde(rename = "faultTypeId")]
    pub fault_type_id: TypeId,
    #[doc = "Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its parameters field. This is what lets a catalogue carry faults whose meaning depends on runtime values — the stuck-stage and interstage timing faults that a closed enum cannot express."]
    #[serde(
        rename = "messageTemplate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub message_template: ::std::option::Option<FaultTypeDefinitionMessageTemplate>,
    #[doc = "Names for the positional placeholders in messageTemplate, in order — ['stageNumber', 'actualSeconds', 'minimumSeconds']. Lets a consumer label the values rather than only interpolate them. Length SHOULD match the number of placeholders."]
    #[serde(
        rename = "parameterNames",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub parameter_names: ::std::vec::Vec<::std::string::String>,
    #[doc = "Numeric severity (higher = more severe). Optional."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<u64>,
    #[doc = "Reference to a SubsystemDefinition entry: the subsystem this fault type belongs to."]
    #[serde(
        rename = "subsystemTypeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subsystem_type_id: ::std::option::Option<TypeId>,
}
impl FaultTypeDefinition {
    pub fn builder() -> builder::FaultTypeDefinition {
        Default::default()
    }
}
#[doc = "The identifier this fault carries in its source document or vendor documentation — 'A1', 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may hold two entries with the same code from different vendors."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier this fault carries in its source document or vendor documentation — 'A1', 'F3', 'W12'. Distinct from faultTypeId, which is this catalogue's key: a deployment may hold two entries with the same code from different vendors.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FaultTypeDefinitionCode(::std::string::String);
impl ::std::ops::Deref for FaultTypeDefinitionCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FaultTypeDefinitionCode> for ::std::string::String {
    fn from(value: FaultTypeDefinitionCode) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FaultTypeDefinitionCode {
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
impl ::std::convert::TryFrom<&str> for FaultTypeDefinitionCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FaultTypeDefinitionCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FaultTypeDefinitionCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FaultTypeDefinitionCode {
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
#[doc = "Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its parameters field. This is what lets a catalogue carry faults whose meaning depends on runtime values — the stuck-stage and interstage timing faults that a closed enum cannot express."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Human-readable message with positional placeholders, e.g. 'Minimum green not met: stage {0} ran {1}s against a minimum of {2}s'. A fault instance supplies the values in its parameters field. This is what lets a catalogue carry faults whose meaning depends on runtime values — the stuck-stage and interstage timing faults that a closed enum cannot express.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FaultTypeDefinitionMessageTemplate(::std::string::String);
impl ::std::ops::Deref for FaultTypeDefinitionMessageTemplate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FaultTypeDefinitionMessageTemplate> for ::std::string::String {
    fn from(value: FaultTypeDefinitionMessageTemplate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FaultTypeDefinitionMessageTemplate {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FaultTypeDefinitionMessageTemplate {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FaultTypeDefinitionMessageTemplate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FaultTypeDefinitionMessageTemplate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FaultTypeDefinitionMessageTemplate {
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
#[doc = "Identifier of this fault type."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeId\","]
#[doc = "  \"description\": \"Identifier of this fault type.\","]
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
        fault_category: ::std::result::Result<
            ::std::option::Option<super::FaultCategory>,
            ::std::string::String,
        >,
        fault_status:
            ::std::result::Result<::std::option::Option<super::FaultStatus>, ::std::string::String>,
        fault_type_definition: ::std::result::Result<
            ::std::option::Option<super::FaultTypeDefinition>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AllTypes {
        fn default() -> Self {
            Self {
                acknowledgement_state: Ok(Default::default()),
                fault_category: Ok(Default::default()),
                fault_status: Ok(Default::default()),
                fault_type_definition: Ok(Default::default()),
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
        pub fn fault_category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultCategory>>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fault_category: {e}"));
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
        pub fn fault_type_definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultTypeDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.fault_type_definition = value.try_into().map_err(|e| {
                format!("error converting supplied value for fault_type_definition: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<AllTypes> for super::AllTypes {
        type Error = super::error::ConversionError;
        fn try_from(value: AllTypes) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                acknowledgement_state: value.acknowledgement_state?,
                fault_category: value.fault_category?,
                fault_status: value.fault_status?,
                fault_type_definition: value.fault_type_definition?,
            })
        }
    }
    impl ::std::convert::From<super::AllTypes> for AllTypes {
        fn from(value: super::AllTypes) -> Self {
            Self {
                acknowledgement_state: Ok(value.acknowledgement_state),
                fault_category: Ok(value.fault_category),
                fault_status: Ok(value.fault_status),
                fault_type_definition: Ok(value.fault_type_definition),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FaultTypeDefinition {
        category: ::std::result::Result<
            ::std::option::Option<super::FaultCategory>,
            ::std::string::String,
        >,
        code: ::std::result::Result<
            ::std::option::Option<super::FaultTypeDefinitionCode>,
            ::std::string::String,
        >,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        fault_type_id: ::std::result::Result<super::TypeId, ::std::string::String>,
        message_template: ::std::result::Result<
            ::std::option::Option<super::FaultTypeDefinitionMessageTemplate>,
            ::std::string::String,
        >,
        parameter_names:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        severity: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        subsystem_type_id:
            ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
    }
    impl ::std::default::Default for FaultTypeDefinition {
        fn default() -> Self {
            Self {
                category: Ok(Default::default()),
                code: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                extensions: Ok(Default::default()),
                fault_type_id: Err("no value supplied for fault_type_id".to_string()),
                message_template: Ok(Default::default()),
                parameter_names: Ok(Default::default()),
                severity: Ok(Default::default()),
                subsystem_type_id: Ok(Default::default()),
            }
        }
    }
    impl FaultTypeDefinition {
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultCategory>>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {e}"));
            self
        }
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FaultTypeDefinitionCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
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
        pub fn message_template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FaultTypeDefinitionMessageTemplate>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_template: {e}"));
            self
        }
        pub fn parameter_names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.parameter_names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parameter_names: {e}"));
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
    }
    impl ::std::convert::TryFrom<FaultTypeDefinition> for super::FaultTypeDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FaultTypeDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                category: value.category?,
                code: value.code?,
                description: value.description?,
                extensions: value.extensions?,
                fault_type_id: value.fault_type_id?,
                message_template: value.message_template?,
                parameter_names: value.parameter_names?,
                severity: value.severity?,
                subsystem_type_id: value.subsystem_type_id?,
            })
        }
    }
    impl ::std::convert::From<super::FaultTypeDefinition> for FaultTypeDefinition {
        fn from(value: super::FaultTypeDefinition) -> Self {
            Self {
                category: Ok(value.category),
                code: Ok(value.code),
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                fault_type_id: Ok(value.fault_type_id),
                message_template: Ok(value.message_template),
                parameter_names: Ok(value.parameter_names),
                severity: Ok(value.severity),
                subsystem_type_id: Ok(value.subsystem_type_id),
            }
        }
    }
}
