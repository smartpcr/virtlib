// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ManagedElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ManagedElement {

/// The Caption property is a short textual description (one- line string) of the object.
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// The Description property provides a textual description of the object.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// A user-friendly name for the object. This property allows each instance to define a user-friendly name in addition to its key properties, identity data, and description information. 
/// Note that the Name property of ManagedSystemElement is also defined as a user-friendly name. But, it is often subclassed to be a Key. It is not reasonable that the same property can convey both identity and a user-friendly name, without inconsistencies. Where Name exists and is not a Key (such as for instances of LogicalDevice), the same information can be present in both the Name and ElementName properties. Note that if there is an associated instance of CIM_EnabledLogicalElementCapabilities, restrictions on this properties may exist as defined in ElementNameMask and MaxElementNameLen properties defined in that class.
    #[serde(rename = "ElementName")]
    pub element_name: Option<String>,

/// InstanceID is an optional property that may be used to opaquely and uniquely identify an instance of this class within the scope of the instantiating Namespace. Various subclasses of this class may override this property to make it required, or a key. Such subclasses may also modify the preferred algorithms for ensuring uniqueness that are defined below.
/// To ensure uniqueness within the NameSpace, the value of InstanceID should be constructed using the following "preferred" algorithm: 
/// <OrgID>:<LocalID> 
/// Where <OrgID> and <LocalID> are separated by a colon (:), and where <OrgID> must include a copyrighted, trademarked, or otherwise unique name that is owned by the business entity that is creating or defining the InstanceID or that is a registered ID assigned to the business entity by a recognized global authority. (This requirement is similar to the <Schema Name>_<Class Name> structure of Schema class names.) In addition, to ensure uniqueness, <OrgID> must not contain a colon (:). When using this algorithm, the first colon to appear in InstanceID must appear between <OrgID> and <LocalID>. 
/// <LocalID> is chosen by the business entity and should not be reused to identify different underlying (real-world) elements. If not null and the above "preferred" algorithm is not used, the defining entity must assure that the resulting InstanceID is not reused across any InstanceIDs produced by this or other providers for the NameSpace of this instance. 
/// If not set to null for DMTF-defined instances, the "preferred" algorithm must be used with the <OrgID> set to CIM.
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,
}

impl CIM_ManagedElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            description: None,
            element_name: None,
            instance_id: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ElementName
    pub fn set_element_name(&mut self, value: String) {
        self.element_name = Some(value);
    }

    /// Gets the value of ElementName
    pub fn get_element_name(&self) -> Option<&String> {
        self.element_name.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }
}

