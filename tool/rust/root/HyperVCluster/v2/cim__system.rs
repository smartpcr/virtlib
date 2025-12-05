// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_System struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_System {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElement,

/// CreationClassName indicates the name of the class or the subclass used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// An array of free-form strings providing explanations and details behind the entries in the OtherIdentifying Info array. Note, each entry of this array is related to the entry in OtherIdentifyingInfo that is located at the same index.
    #[serde(rename = "IdentifyingDescriptions")]
    pub identifying_descriptions: Vec<String>,

/// The System object and its derivatives are top-level objects of CIM. They provide the scope for numerous components. Having unique System keys is required. A heuristic can be defined in individual System subclasses to attempt to always generate the same System Name Key. The NameFormat property identifies how the System name was generated, using the heuristic of the subclass.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<String>,

/// OtherIdentifyingInfo captures additional data, beyond System Name information, that could be used to identify a ComputerSystem. One example would be to hold the Fibre Channel World-Wide Name (WWN) of a node. Note that if only the Fibre Channel name is available and is unique (able to be used as the System key), then this property would be NULL and the WWN would become the System key, its data placed in the Name property.
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// A string that provides information on how the primary system owner can be reached (for example, phone number, e-mail address, and so on).
    #[serde(rename = "PrimaryOwnerContact")]
    pub primary_owner_contact: Option<String>,

/// The name of the primary system owner. The system owner is the primary user of the system.
    #[serde(rename = "PrimaryOwnerName")]
    pub primary_owner_name: Option<String>,

/// An array (bag) of strings that specifies the administrator -defined roles this System plays in the managed environment. Examples might be 'Building 8 print server' or 'Boise user directories'. A single system may perform multiple roles. 
/// Note that the instrumentation view of the 'roles' of a System is defined by instantiating a specific subclass of System, or by properties in a subclass, or both. For example, the purpose of a ComputerSystem is defined using the Dedicated and OtherDedicatedDescription properties.
    #[serde(rename = "Roles")]
    pub roles: Vec<String>,
}

impl CIM_System {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElement::new(),
            creation_class_name: None,
            identifying_descriptions: Vec::new(),
            name_format: None,
            other_identifying_info: Vec::new(),
            primary_owner_contact: None,
            primary_owner_name: None,
            roles: Vec::new(),
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of IdentifyingDescriptions
    pub fn set_identifying_descriptions(&mut self, value: Vec<String>) {
        self.identifying_descriptions = value;
    }

    /// Gets the value of IdentifyingDescriptions
    pub fn get_identifying_descriptions(&self) -> &Vec<String> {
        &self.identifying_descriptions
    }

    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: String) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&String> {
        self.name_format.as_ref()
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of PrimaryOwnerContact
    pub fn set_primary_owner_contact(&mut self, value: String) {
        self.primary_owner_contact = Some(value);
    }

    /// Gets the value of PrimaryOwnerContact
    pub fn get_primary_owner_contact(&self) -> Option<&String> {
        self.primary_owner_contact.as_ref()
    }

    /// Sets the value of PrimaryOwnerName
    pub fn set_primary_owner_name(&mut self, value: String) {
        self.primary_owner_name = Some(value);
    }

    /// Gets the value of PrimaryOwnerName
    pub fn get_primary_owner_name(&self) -> Option<&String> {
        self.primary_owner_name.as_ref()
    }

    /// Sets the value of Roles
    pub fn set_roles(&mut self, value: Vec<String>) {
        self.roles = value;
    }

    /// Gets the value of Roles
    pub fn get_roles(&self) -> &Vec<String> {
        &self.roles
    }
}

