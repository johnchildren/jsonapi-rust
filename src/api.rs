//! Defines custom types and structs primarily that composite the JSON:API
//! document
use crate::errors::*;
use serde_json;
use std;
use std::collections::HashMap;
use std::str::FromStr;

/// Permitted JSON-API values (all JSON Values)
pub type JsonApiValue = serde_json::Value;

/// Vector of `Resource`
pub type Resources = Vec<Resource>;
/// Vector of `PartialResource`
pub type PartialResources = Vec<PartialResource>;
/// Vector of `ResourceIdentifiers`
pub type ResourceIdentifiers = Vec<ResourceIdentifier>;
pub type Links = HashMap<String, JsonApiValue>;
/// Meta-data object, can contain any data
pub type Meta = HashMap<String, JsonApiValue>;
/// Resource Attributes, can be any JSON value
pub type ResourceAttributes = HashMap<String, JsonApiValue>;
/// Map of relationships with other objects
pub type Relationships = HashMap<String, Relationship>;
/// Side-loaded Resources
pub type Included = Vec<Resource>;
/// Data-related errors
pub type JsonApiErrors = Vec<JsonApiError>;

pub type JsonApiId = String;
pub type JsonApiIds<'a> = Vec<&'a JsonApiId>;

/// Resource Identifier
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: JsonApiId,
}

/// Representation of a JSON:API resource. This is a struct that contains
/// attributes that map to the JSON:API specification of `id`, `type`,
/// `attributes`, `relationships`, `links`, and `meta`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resource {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: JsonApiId,
    #[serde(default)]
    pub attributes: ResourceAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Representation of a JSON:API resource that lacks an id.
/// This is a struct that contains attributes that map to the
/// JSON:API specification of `id`, `type`,
/// `attributes`, `relationships`, `links`, and `meta`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct PartialResource {
    pub _type: String,
    pub attributes: ResourceAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
}

/// Relationship with another object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IdentifierData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Valid data Resource (can be None)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PrimaryData {
    None,
    Single(Box<Resource>),
    Multiple(Resources),
    SinglePartial(Box<PartialResource>),
    MultiplePartial(PartialResources),
}

/// Valid Resource Identifier (can be None)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum IdentifierData {
    None,
    Single(ResourceIdentifier),
    Multiple(ResourceIdentifiers),
}

/// A struct that defines an error state for a JSON:API document
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DocumentError {
    pub errors: JsonApiErrors,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApiInfo>,
}

/// A struct that defines properties for a JSON:API document that contains no errors
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DocumentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PrimaryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Resources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApiInfo>,
}

/// An enum that defines the possible composition of a JSON:API document - one which contains
/// `error`, `data` or `partial_data` - but not multiple.
/// Rely on Rust's type system to handle this basic validation instead of
/// running validators on parsed documents
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum JsonApiDocument {
    Error(DocumentError),
    Data(DocumentData),
}

/// Error location
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

/// Retpresentation of a JSON:API error (all fields are optional)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct JsonApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Optional `JsonApiDocument` payload identifying the JSON-API version the
/// server implements
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonApiInfo {
    pub version: Option<String>,
    pub meta: Option<Meta>,
}

/// Pagination links
#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(Debug)]
pub struct Patch {
    pub patch_type: PatchType,
    pub subject: String,
    pub previous: JsonApiValue,
    pub next: JsonApiValue,
}

#[derive(Debug)]
pub struct PatchSet {
    pub resource_type: String,
    pub resource_id: String,
    pub patches: Vec<Patch>,
}

impl PatchSet {
    pub fn new_for(resource: &Resource) -> Self {
        PatchSet {
            resource_type: resource._type.clone(),
            resource_id: resource.id.clone(),
            patches: Vec::<Patch>::new(),
        }
    }

    pub fn push(&mut self, patch: Patch) {
        self.patches.push(patch);
    }
}

impl DocumentData {
    fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    fn has_included(&self) -> bool {
        self.included.is_some()
    }
    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

/// Top-level JSON-API Document
/// An "error" document can be valid, just as a "data" document can be valid
impl JsonApiDocument {
    /// This function returns `false` if the `JsonApiDocument` contains any violations of the
    /// specification. See [`DocumentValidationError`](enum.DocumentValidationError.html)
    ///
    /// The spec dictates that the document must have least one of `data`, `errors` or `meta`.
    /// Of these, `data` and `errors` must not co-exist.
    /// The optional field `included` may only be present if the `data` field is present too.
    pub fn is_valid(&self) -> bool {
        self.validate().is_none()
    }

    /// This function returns a `Vec` with identified specification violations enumerated in
    /// `DocumentValidationError`
    ///
    /// ```
    /// // Simulate an error where `included` has data but `data` does not
    /// use jsonapi::api::*;
    /// use std::str::FromStr;
    ///
    /// let serialized = r#"{
    ///   "id":"1",
    ///   "type":"post",
    ///   "attributes":{
    ///     "title": "Rails is Omakase",
    ///     "likes": 250
    ///   },
    ///   "relationships":{},
    ///   "links" :{}
    /// }"#;
    ///
    /// let resource = Resource::from_str(&serialized);
    ///
    /// let data = DocumentData {
    ///     data: None,
    ///     included: Some(vec![resource.unwrap()]),
    ///     ..Default::default()
    /// };
    ///
    /// let doc = JsonApiDocument::Data(data);
    ///
    /// match doc.validate() {
    ///   Some(errors) => {
    ///     assert!(
    ///       errors.contains(
    ///         &DocumentValidationError::IncludedWithoutData
    ///       )
    ///     )
    ///   }
    ///   None => assert!(false)
    /// }
    /// ```
    pub fn validate(&self) -> Option<Vec<DocumentValidationError>> {
        let mut errors = Vec::<DocumentValidationError>::new();

        match self {
            JsonApiDocument::Error(_x) => None,
            JsonApiDocument::Data(doc) => {
                if doc.has_included() && !doc.has_data() {
                    errors.push(DocumentValidationError::IncludedWithoutData);
                }

                if !(doc.has_data() || doc.has_meta()) {
                    errors.push(DocumentValidationError::MissingContent);
                }
                match errors.len() {
                    0 => None,
                    _ => Some(errors),
                }
            }
        }
    }
}

impl FromStr for JsonApiDocument {
    type Err = Error;

    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::JsonApiDocument;
    /// use std::str::FromStr;
    ///
    /// let serialized = r#"{
    ///   "data" : [
    ///     { "id":"1", "type":"post", "attributes":{}, "relationships":{}, "links" :{} },
    ///     { "id":"2", "type":"post", "attributes":{}, "relationships":{}, "links" :{} },
    ///     { "id":"3", "type":"post", "attributes":{}, "relationships":{}, "links" :{} }
    ///   ]
    /// }"#;
    /// let doc = JsonApiDocument::from_str(&serialized);
    /// assert_eq!(doc.is_ok(), true);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).chain_err(|| "Error parsing Document")
    }
}

impl Resource {
    pub fn get_relationship(&self, name: &str) -> Option<&Relationship> {
        match self.relationships {
            None => None,
            Some(ref relationships) => {
                match relationships.get(name) {
                    None => None,
                    Some(rel) => Some(rel),
                }
            }
        }
    }

    /// Get an attribute `JsonApiValue`
    ///
    /// ```
    /// use jsonapi::api::Resource;
    /// use std::str::FromStr;
    ///
    /// let serialized = r#"{
    ///   "id":"1",
    ///   "type":"post",
    ///   "attributes":{
    ///     "title": "Rails is Omakase",
    ///     "likes": 250
    ///   },
    ///   "relationships":{},
    ///   "links" :{}
    /// }"#;
    ///
    /// match Resource::from_str(&serialized) {
    ///   Err(_)=> assert!(false),
    ///   Ok(resource)=> {
    ///     match resource.get_attribute("title") {
    ///       None => assert!(false),
    ///       Some(attr) => {
    ///         match attr.as_str() {
    ///           None => assert!(false),
    ///           Some(s) => {
    ///               assert_eq!(s, "Rails is Omakase");
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    pub fn get_attribute(&self, name: &str) -> Option<&JsonApiValue> {
        match self.attributes.get(name) {
            None => None,
            Some(val) => Some(val),
        }
    }

    pub fn diff(&self, other: Resource) -> std::result::Result<PatchSet, DiffPatchError> {
        if self._type != other._type {
            Err(DiffPatchError::IncompatibleTypes(
                self._type.clone(),
                other._type.clone(),
            ))
        } else {

            let mut self_keys: Vec<String> =
                self.attributes.iter().map(|(key, _)| key.clone()).collect();

            self_keys.sort();

            let mut other_keys: Vec<String> = other
                .attributes
                .iter()
                .map(|(key, _)| key.clone())
                .collect();

            other_keys.sort();

            let matching = self_keys
                .iter()
                .zip(other_keys.iter())
                .filter(|&(a, b)| a == b)
                .count();

            if matching != self_keys.len() {
                Err(DiffPatchError::DifferentAttributeKeys)
            } else {
                let mut patchset = PatchSet::new_for(self);

                for (attr, self_value) in &self.attributes {
                    match other.attributes.get(attr) {
                        None => {
                            error!(
                                "Resource::diff unable to find attribute {:?} in {:?}",
                                attr,
                                other
                            );
                        }
                        Some(other_value) => {
                            if self_value != other_value {
                                patchset.push(Patch {
                                    patch_type: PatchType::Attribute,
                                    subject: attr.clone(),
                                    previous: self_value.clone(),
                                    next: other_value.clone(),
                                });
                            }
                        }
                    }

                }

                Ok(patchset)
            }
        }
    }

    pub fn patch(&mut self, patchset: PatchSet) -> Result<Resource> {
        let mut res = self.clone();
        for patch in &patchset.patches {
            res.attributes.insert(
                patch.subject.clone(),
                patch.next.clone(),
            );
        }
        Ok(res)
    }
}

impl FromStr for Resource {
    type Err = Error;

    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::Resource;
    /// use std::str::FromStr;
    ///
    /// let serialized = r#"{
    ///   "id":"1",
    ///   "type":"post",
    ///   "attributes":{
    ///     "title": "Rails is Omakase",
    ///     "likes": 250
    ///   },
    ///   "relationships":{},
    ///   "links" :{}
    /// }"#;
    ///
    /// let data = Resource::from_str(&serialized);
    /// assert_eq!(data.is_ok(), true);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).chain_err(|| "Error parsing resource")
    }
}


impl Relationship {
    pub fn as_id(&self) -> std::result::Result<Option<&JsonApiId>, RelationshipAssumptionError> {
        match self.data {
            Some(IdentifierData::None) => Ok(None),
            Some(IdentifierData::Multiple(_)) => Err(RelationshipAssumptionError::RelationshipIsAList),
            Some(IdentifierData::Single(ref data)) => Ok(Some(&data.id)),
            None => Ok(None),
        }
    }

    pub fn as_ids(&self) -> std::result::Result<Option<JsonApiIds>, RelationshipAssumptionError> {
        match self.data {
            Some(IdentifierData::None) => Ok(None),
            Some(IdentifierData::Single(_)) => Err(RelationshipAssumptionError::RelationshipIsNotAList),
            Some(IdentifierData::Multiple(ref data)) => Ok(Some(data.iter().map(|x| &x.id).collect())),
            None => Ok(None),
        }
    }
}

/// Enum to describe top-level JSON:API specification violations
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DocumentValidationError {
    IncludedWithoutData,
    MissingContent,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum JsonApiDataError {
    AttributeNotFound,
    IncompatibleAttributeType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RelationshipAssumptionError {
    RelationshipIsAList,
    RelationshipIsNotAList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffPatchError {
    IncompatibleTypes(String, String),
    DifferentAttributeKeys,
    NonExistentProperty(String),
    IncorrectPropertyValue(String),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PatchType {
    Relationship,
    Attribute,
}
