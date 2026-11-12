/*
 * File:        output/rust/sdk/src/network.rs
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
#[doc = "    \"GeoJsonLineString\": {"]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonLineString\""]
#[doc = "    },"]
#[doc = "    \"GeoJsonPoint\": {"]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
#[doc = "    },"]
#[doc = "    \"NetworkLink\": {"]
#[doc = "      \"$ref\": \"#/$defs/NetworkLink\""]
#[doc = "    },"]
#[doc = "    \"NetworkNode\": {"]
#[doc = "      \"$ref\": \"#/$defs/NetworkNode\""]
#[doc = "    },"]
#[doc = "    \"NetworkPath\": {"]
#[doc = "      \"$ref\": \"#/$defs/NetworkPath\""]
#[doc = "    },"]
#[doc = "    \"NetworkTurn\": {"]
#[doc = "      \"$ref\": \"#/$defs/NetworkTurn\""]
#[doc = "    },"]
#[doc = "    \"NetworkZone\": {"]
#[doc = "      \"$ref\": \"#/$defs/NetworkZone\""]
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
        rename = "GeoJsonLineString",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geo_json_line_string: ::std::option::Option<GeoJsonLineString>,
    #[serde(
        rename = "GeoJsonPoint",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub geo_json_point: ::std::option::Option<GeoJsonPoint>,
    #[serde(
        rename = "NetworkLink",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_link: ::std::option::Option<NetworkLink>,
    #[serde(
        rename = "NetworkNode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_node: ::std::option::Option<NetworkNode>,
    #[serde(
        rename = "NetworkPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_path: ::std::option::Option<NetworkPath>,
    #[serde(
        rename = "NetworkTurn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_turn: ::std::option::Option<NetworkTurn>,
    #[serde(
        rename = "NetworkZone",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub network_zone: ::std::option::Option<NetworkZone>,
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
            geo_json_line_string: Default::default(),
            geo_json_point: Default::default(),
            network_link: Default::default(),
            network_node: Default::default(),
            network_path: Default::default(),
            network_turn: Default::default(),
            network_zone: Default::default(),
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
#[doc = "A GeoJSON LineString geometry (RFC 7946).\n\nAn ordered sequence of two or more positions forming a polyline.\nCoordinates are [longitude, latitude] per GeoJSON specification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A GeoJSON LineString geometry (RFC 7946).\\n\\nAn ordered sequence of two or more positions forming a polyline.\\nCoordinates are [longitude, latitude] per GeoJSON specification.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"coordinates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"coordinates\": {"]
#[doc = "      \"description\": \"Ordered [longitude, latitude] pairs forming a polyline.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"maxItems\": 2,"]
#[doc = "        \"minItems\": 2,"]
#[doc = "        \"prefixItems\": ["]
#[doc = "          {"]
#[doc = "            \"maximum\": 180.0,"]
#[doc = "            \"minimum\": -180.0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"maximum\": 90.0,"]
#[doc = "            \"minimum\": -90.0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minItems\": 2"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"default\": \"LineString\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"LineString\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GeoJsonLineString {
    #[doc = "Ordered [longitude, latitude] pairs forming a polyline."]
    pub coordinates: ::std::vec::Vec<[::serde_json::Value; 2usize]>,
    #[serde(rename = "type", default = "defaults::geo_json_line_string_type")]
    pub type_: GeoJsonLineStringType,
}
impl GeoJsonLineString {
    pub fn builder() -> builder::GeoJsonLineString {
        Default::default()
    }
}
#[doc = "`GeoJsonLineStringType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"LineString\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"LineString\""]
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
pub enum GeoJsonLineStringType {
    LineString,
}
impl ::std::fmt::Display for GeoJsonLineStringType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LineString => f.write_str("LineString"),
        }
    }
}
impl ::std::str::FromStr for GeoJsonLineStringType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LineString" => Ok(Self::LineString),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GeoJsonLineStringType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GeoJsonLineStringType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GeoJsonLineStringType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for GeoJsonLineStringType {
    fn default() -> Self {
        GeoJsonLineStringType::LineString
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
#[doc = "Road network link between two nodes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network Link\","]
#[doc = "  \"description\": \"Road network link between two nodes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fromNodeId\","]
#[doc = "    \"linkId\","]
#[doc = "    \"toNodeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"fromNodeId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Start node reference.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"geometry\": {"]
#[doc = "      \"description\": \"Polyline shape of the link as GeoJSON LineString.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonLineString\""]
#[doc = "    },"]
#[doc = "    \"linkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this link.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"toNodeId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"End node reference.\","]
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
pub struct NetworkLink {
    #[doc = "Human-readable name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Start node reference."]
    #[serde(rename = "fromNodeId")]
    pub from_node_id: ObjectId,
    #[doc = "Polyline shape of the link as GeoJSON LineString."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub geometry: ::std::option::Option<GeoJsonLineString>,
    #[doc = "Unique identifier of this link."]
    #[serde(rename = "linkId")]
    pub link_id: ObjectId,
    #[doc = "End node reference."]
    #[serde(rename = "toNodeId")]
    pub to_node_id: ObjectId,
}
impl NetworkLink {
    pub fn builder() -> builder::NetworkLink {
        Default::default()
    }
}
#[doc = "Road network node (junction or endpoint)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network Node\","]
#[doc = "  \"description\": \"Road network node (junction or endpoint).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nodeId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"Geographic position as GeoJSON Point.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonPoint\""]
#[doc = "    },"]
#[doc = "    \"nodeId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this node.\","]
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
pub struct NetworkNode {
    #[doc = "Human-readable name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Geographic position as GeoJSON Point."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<GeoJsonPoint>,
    #[doc = "Unique identifier of this node."]
    #[serde(rename = "nodeId")]
    pub node_id: ObjectId,
}
impl NetworkNode {
    pub fn builder() -> builder::NetworkNode {
        Default::default()
    }
}
#[doc = "Road network path (ordered sequence of links)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network Path\","]
#[doc = "  \"description\": \"Road network path (ordered sequence of links).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pathId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"finalTrimMeters\": {"]
#[doc = "      \"description\": \"Distance to trim from the end of the last link.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"initialTrimMeters\": {"]
#[doc = "      \"description\": \"Distance to trim from the start of the first link.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"linkIds\": {"]
#[doc = "      \"description\": \"Ordered links defining the path. List order is the route sequence. Omit the field rather than send an empty list. A link may appear more than once, since a route can traverse the same link twice.\","]
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
#[doc = "    \"pathId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this path.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"permanent\": {"]
#[doc = "      \"description\": \"Whether this is a permanent or temporary path.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Path type classification (e.g. bus route, diversion).\","]
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
pub struct NetworkPath {
    #[doc = "Human-readable name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Distance to trim from the end of the last link."]
    #[serde(
        rename = "finalTrimMeters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub final_trim_meters: ::std::option::Option<f64>,
    #[doc = "Distance to trim from the start of the first link."]
    #[serde(
        rename = "initialTrimMeters",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub initial_trim_meters: ::std::option::Option<f64>,
    #[doc = "Ordered links defining the path. List order is the route sequence. Omit the field rather than send an empty list. A link may appear more than once, since a route can traverse the same link twice."]
    #[serde(
        rename = "linkIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub link_ids: ::std::vec::Vec<ObjectId>,
    #[doc = "Unique identifier of this path."]
    #[serde(rename = "pathId")]
    pub path_id: ObjectId,
    #[doc = "Whether this is a permanent or temporary path."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub permanent: ::std::option::Option<bool>,
    #[doc = "Path type classification (e.g. bus route, diversion)."]
    #[serde(
        rename = "typeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_id: ::std::option::Option<TypeId>,
}
impl NetworkPath {
    pub fn builder() -> builder::NetworkPath {
        Default::default()
    }
}
#[doc = "Road network turn (movement through a node between links)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network Turn\","]
#[doc = "  \"description\": \"Road network turn (movement through a node between links).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"allowed\","]
#[doc = "    \"fromLinkId\","]
#[doc = "    \"toLinkId\","]
#[doc = "    \"turnId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowed\": {"]
#[doc = "      \"description\": \"Whether this turn is permitted.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable description.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"fromLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Approach link.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"geometry\": {"]
#[doc = "      \"description\": \"Turn path geometry as GeoJSON LineString.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonLineString\""]
#[doc = "    },"]
#[doc = "    \"toLinkId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Departure link.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9._:-]+$\""]
#[doc = "    },"]
#[doc = "    \"turnId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this turn.\","]
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
pub struct NetworkTurn {
    #[doc = "Whether this turn is permitted."]
    pub allowed: bool,
    #[doc = "Human-readable description."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Approach link."]
    #[serde(rename = "fromLinkId")]
    pub from_link_id: ObjectId,
    #[doc = "Turn path geometry as GeoJSON LineString."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub geometry: ::std::option::Option<GeoJsonLineString>,
    #[doc = "Departure link."]
    #[serde(rename = "toLinkId")]
    pub to_link_id: ObjectId,
    #[doc = "Unique identifier of this turn."]
    #[serde(rename = "turnId")]
    pub turn_id: ObjectId,
}
impl NetworkTurn {
    pub fn builder() -> builder::NetworkTurn {
        Default::default()
    }
}
#[doc = "Road network zone (area grouping of links/nodes)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network Zone\","]
#[doc = "  \"description\": \"Road network zone (area grouping of links/nodes).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"zoneId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"Human-readable name.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"Vendor or local extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"geometry\": {"]
#[doc = "      \"description\": \"Zone boundary geometry as GeoJSON LineString.\","]
#[doc = "      \"$ref\": \"#/$defs/GeoJsonLineString\""]
#[doc = "    },"]
#[doc = "    \"linkIds\": {"]
#[doc = "      \"description\": \"The links that make up the zone, as a set: the order of the array carries no meaning (v1 describes a zone as an unordered sequence of links). Omit the field rather than send an empty list. Each link appears at most once.\","]
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
#[doc = "    \"permanent\": {"]
#[doc = "      \"description\": \"Whether this is a permanent or temporary zone. Absent means permanent (the v1 default for a zone).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"typeId\": {"]
#[doc = "      \"title\": \"TypeId\","]
#[doc = "      \"description\": \"Zone type classification (e.g. electoral ward, incident area).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^\\\\S(?:[\\\\s\\\\S]*\\\\S)?$\""]
#[doc = "    },"]
#[doc = "    \"zoneId\": {"]
#[doc = "      \"title\": \"ObjectId\","]
#[doc = "      \"description\": \"Unique identifier of this zone.\","]
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
pub struct NetworkZone {
    #[doc = "Human-readable name."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Vendor or local extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub extensions: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "Zone boundary geometry as GeoJSON LineString."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub geometry: ::std::option::Option<GeoJsonLineString>,
    #[doc = "The links that make up the zone, as a set: the order of the array carries no meaning (v1 describes a zone as an unordered sequence of links). Omit the field rather than send an empty list. Each link appears at most once."]
    #[serde(
        rename = "linkIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub link_ids: ::std::vec::Vec<ObjectId>,
    #[doc = "Whether this is a permanent or temporary zone. Absent means permanent (the v1 default for a zone)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub permanent: ::std::option::Option<bool>,
    #[doc = "Zone type classification (e.g. electoral ward, incident area)."]
    #[serde(
        rename = "typeId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_id: ::std::option::Option<TypeId>,
    #[doc = "Unique identifier of this zone."]
    #[serde(rename = "zoneId")]
    pub zone_id: ObjectId,
}
impl NetworkZone {
    pub fn builder() -> builder::NetworkZone {
        Default::default()
    }
}
#[doc = "Start node reference."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ObjectId\","]
#[doc = "  \"description\": \"Start node reference.\","]
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
#[doc = "Terminal message ending a network topology sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network topology Sync Complete\","]
#[doc = "  \"description\": \"Terminal message ending a network topology sync batch. Sent to the requester's reply node after the last object, or instead of any objects when the filter matched none.\","]
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
#[doc = "Request the current state of network topology objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UTMC Network topology Sync Request\","]
#[doc = "  \"description\": \"Request the current state of network topology objects. The Data Publisher answers to the request's reply-to address, not the domain's data channels: a sync batch (x-message-mode: sync, x-sync-id/-sequence/-total) followed by a SyncComplete.\","]
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
#[doc = "Path type classification (e.g. bus route, diversion)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TypeId\","]
#[doc = "  \"description\": \"Path type classification (e.g. bus route, diversion).\","]
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
        geo_json_line_string: ::std::result::Result<
            ::std::option::Option<super::GeoJsonLineString>,
            ::std::string::String,
        >,
        geo_json_point: ::std::result::Result<
            ::std::option::Option<super::GeoJsonPoint>,
            ::std::string::String,
        >,
        network_link:
            ::std::result::Result<::std::option::Option<super::NetworkLink>, ::std::string::String>,
        network_node:
            ::std::result::Result<::std::option::Option<super::NetworkNode>, ::std::string::String>,
        network_path:
            ::std::result::Result<::std::option::Option<super::NetworkPath>, ::std::string::String>,
        network_turn:
            ::std::result::Result<::std::option::Option<super::NetworkTurn>, ::std::string::String>,
        network_zone:
            ::std::result::Result<::std::option::Option<super::NetworkZone>, ::std::string::String>,
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
                geo_json_line_string: Ok(Default::default()),
                geo_json_point: Ok(Default::default()),
                network_link: Ok(Default::default()),
                network_node: Ok(Default::default()),
                network_path: Ok(Default::default()),
                network_turn: Ok(Default::default()),
                network_zone: Ok(Default::default()),
                sync_category_enum: Ok(Default::default()),
                sync_complete: Ok(Default::default()),
                sync_request: Ok(Default::default()),
                sync_status: Ok(Default::default()),
            }
        }
    }
    impl AllTypes {
        pub fn geo_json_line_string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonLineString>>,
            T::Error: ::std::fmt::Display,
        {
            self.geo_json_line_string = value.try_into().map_err(|e| {
                format!("error converting supplied value for geo_json_line_string: {e}")
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
        pub fn network_link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkLink>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_link: {e}"));
            self
        }
        pub fn network_node<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkNode>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_node = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_node: {e}"));
            self
        }
        pub fn network_path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkPath>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_path: {e}"));
            self
        }
        pub fn network_turn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkTurn>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_turn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_turn: {e}"));
            self
        }
        pub fn network_zone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::NetworkZone>>,
            T::Error: ::std::fmt::Display,
        {
            self.network_zone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for network_zone: {e}"));
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
                geo_json_line_string: value.geo_json_line_string?,
                geo_json_point: value.geo_json_point?,
                network_link: value.network_link?,
                network_node: value.network_node?,
                network_path: value.network_path?,
                network_turn: value.network_turn?,
                network_zone: value.network_zone?,
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
                geo_json_line_string: Ok(value.geo_json_line_string),
                geo_json_point: Ok(value.geo_json_point),
                network_link: Ok(value.network_link),
                network_node: Ok(value.network_node),
                network_path: Ok(value.network_path),
                network_turn: Ok(value.network_turn),
                network_zone: Ok(value.network_zone),
                sync_category_enum: Ok(value.sync_category_enum),
                sync_complete: Ok(value.sync_complete),
                sync_request: Ok(value.sync_request),
                sync_status: Ok(value.sync_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeoJsonLineString {
        coordinates: ::std::result::Result<
            ::std::vec::Vec<[::serde_json::Value; 2usize]>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::GeoJsonLineStringType, ::std::string::String>,
    }
    impl ::std::default::Default for GeoJsonLineString {
        fn default() -> Self {
            Self {
                coordinates: Err("no value supplied for coordinates".to_string()),
                type_: Ok(super::defaults::geo_json_line_string_type()),
            }
        }
    }
    impl GeoJsonLineString {
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<[::serde_json::Value; 2usize]>>,
            T::Error: ::std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GeoJsonLineStringType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GeoJsonLineString> for super::GeoJsonLineString {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GeoJsonLineString,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                coordinates: value.coordinates?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::GeoJsonLineString> for GeoJsonLineString {
        fn from(value: super::GeoJsonLineString) -> Self {
            Self {
                coordinates: Ok(value.coordinates),
                type_: Ok(value.type_),
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
    pub struct NetworkLink {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        from_node_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        geometry: ::std::result::Result<
            ::std::option::Option<super::GeoJsonLineString>,
            ::std::string::String,
        >,
        link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        to_node_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkLink {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                from_node_id: Err("no value supplied for from_node_id".to_string()),
                geometry: Ok(Default::default()),
                link_id: Err("no value supplied for link_id".to_string()),
                to_node_id: Err("no value supplied for to_node_id".to_string()),
            }
        }
    }
    impl NetworkLink {
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
        pub fn from_node_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.from_node_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from_node_id: {e}"));
            self
        }
        pub fn geometry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonLineString>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry: {e}"));
            self
        }
        pub fn link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.link_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_id: {e}"));
            self
        }
        pub fn to_node_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.to_node_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_node_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NetworkLink> for super::NetworkLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                from_node_id: value.from_node_id?,
                geometry: value.geometry?,
                link_id: value.link_id?,
                to_node_id: value.to_node_id?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkLink> for NetworkLink {
        fn from(value: super::NetworkLink) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                from_node_id: Ok(value.from_node_id),
                geometry: Ok(value.geometry),
                link_id: Ok(value.link_id),
                to_node_id: Ok(value.to_node_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NetworkNode {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
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
        node_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkNode {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                location: Ok(Default::default()),
                node_id: Err("no value supplied for node_id".to_string()),
            }
        }
    }
    impl NetworkNode {
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
        pub fn node_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.node_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NetworkNode> for super::NetworkNode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkNode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                location: value.location?,
                node_id: value.node_id?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkNode> for NetworkNode {
        fn from(value: super::NetworkNode) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                location: Ok(value.location),
                node_id: Ok(value.node_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NetworkPath {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        final_trim_meters: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        initial_trim_meters:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        link_ids: ::std::result::Result<::std::vec::Vec<super::ObjectId>, ::std::string::String>,
        path_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        permanent: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_id: ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkPath {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                final_trim_meters: Ok(Default::default()),
                initial_trim_meters: Ok(Default::default()),
                link_ids: Ok(Default::default()),
                path_id: Err("no value supplied for path_id".to_string()),
                permanent: Ok(Default::default()),
                type_id: Ok(Default::default()),
            }
        }
    }
    impl NetworkPath {
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
        pub fn final_trim_meters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.final_trim_meters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for final_trim_meters: {e}"));
            self
        }
        pub fn initial_trim_meters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial_trim_meters = value.try_into().map_err(|e| {
                format!("error converting supplied value for initial_trim_meters: {e}")
            });
            self
        }
        pub fn link_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_ids: {e}"));
            self
        }
        pub fn path_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.path_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path_id: {e}"));
            self
        }
        pub fn permanent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.permanent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permanent: {e}"));
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
    impl ::std::convert::TryFrom<NetworkPath> for super::NetworkPath {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkPath,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                final_trim_meters: value.final_trim_meters?,
                initial_trim_meters: value.initial_trim_meters?,
                link_ids: value.link_ids?,
                path_id: value.path_id?,
                permanent: value.permanent?,
                type_id: value.type_id?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkPath> for NetworkPath {
        fn from(value: super::NetworkPath) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                final_trim_meters: Ok(value.final_trim_meters),
                initial_trim_meters: Ok(value.initial_trim_meters),
                link_ids: Ok(value.link_ids),
                path_id: Ok(value.path_id),
                permanent: Ok(value.permanent),
                type_id: Ok(value.type_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NetworkTurn {
        allowed: ::std::result::Result<bool, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        from_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        geometry: ::std::result::Result<
            ::std::option::Option<super::GeoJsonLineString>,
            ::std::string::String,
        >,
        to_link_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
        turn_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkTurn {
        fn default() -> Self {
            Self {
                allowed: Err("no value supplied for allowed".to_string()),
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                from_link_id: Err("no value supplied for from_link_id".to_string()),
                geometry: Ok(Default::default()),
                to_link_id: Err("no value supplied for to_link_id".to_string()),
                turn_id: Err("no value supplied for turn_id".to_string()),
            }
        }
    }
    impl NetworkTurn {
        pub fn allowed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed: {e}"));
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
        pub fn from_link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.from_link_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for from_link_id: {e}"));
            self
        }
        pub fn geometry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonLineString>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry: {e}"));
            self
        }
        pub fn to_link_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.to_link_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for to_link_id: {e}"));
            self
        }
        pub fn turn_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.turn_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for turn_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NetworkTurn> for super::NetworkTurn {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkTurn,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed: value.allowed?,
                description: value.description?,
                extensions: value.extensions?,
                from_link_id: value.from_link_id?,
                geometry: value.geometry?,
                to_link_id: value.to_link_id?,
                turn_id: value.turn_id?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkTurn> for NetworkTurn {
        fn from(value: super::NetworkTurn) -> Self {
            Self {
                allowed: Ok(value.allowed),
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                from_link_id: Ok(value.from_link_id),
                geometry: Ok(value.geometry),
                to_link_id: Ok(value.to_link_id),
                turn_id: Ok(value.turn_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NetworkZone {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        geometry: ::std::result::Result<
            ::std::option::Option<super::GeoJsonLineString>,
            ::std::string::String,
        >,
        link_ids: ::std::result::Result<::std::vec::Vec<super::ObjectId>, ::std::string::String>,
        permanent: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_id: ::std::result::Result<::std::option::Option<super::TypeId>, ::std::string::String>,
        zone_id: ::std::result::Result<super::ObjectId, ::std::string::String>,
    }
    impl ::std::default::Default for NetworkZone {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                geometry: Ok(Default::default()),
                link_ids: Ok(Default::default()),
                permanent: Ok(Default::default()),
                type_id: Ok(Default::default()),
                zone_id: Err("no value supplied for zone_id".to_string()),
            }
        }
    }
    impl NetworkZone {
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
        pub fn geometry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GeoJsonLineString>>,
            T::Error: ::std::fmt::Display,
        {
            self.geometry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for geometry: {e}"));
            self
        }
        pub fn link_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ObjectId>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_ids: {e}"));
            self
        }
        pub fn permanent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.permanent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permanent: {e}"));
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
        pub fn zone_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ObjectId>,
            T::Error: ::std::fmt::Display,
        {
            self.zone_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zone_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NetworkZone> for super::NetworkZone {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NetworkZone,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extensions: value.extensions?,
                geometry: value.geometry?,
                link_ids: value.link_ids?,
                permanent: value.permanent?,
                type_id: value.type_id?,
                zone_id: value.zone_id?,
            })
        }
    }
    impl ::std::convert::From<super::NetworkZone> for NetworkZone {
        fn from(value: super::NetworkZone) -> Self {
            Self {
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                geometry: Ok(value.geometry),
                link_ids: Ok(value.link_ids),
                permanent: Ok(value.permanent),
                type_id: Ok(value.type_id),
                zone_id: Ok(value.zone_id),
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
    pub(super) fn geo_json_line_string_type() -> super::GeoJsonLineStringType {
        super::GeoJsonLineStringType::LineString
    }
    pub(super) fn geo_json_point_type() -> super::GeoJsonPointType {
        super::GeoJsonPointType::Point
    }
}
