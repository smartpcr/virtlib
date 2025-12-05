// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AllocationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AllocationCapabilities {
    #[serde(flatten)]
    pub base: CIM_Capabilities,

/// A string that describes the resource type when a well defined value is not available and ResourceType has the value "Other".
    #[serde(rename = "OtherResourceType")]
    pub other_resource_type: Option<String>,

/// Indicates whether requesting a specific resource is supported 
/// "Specific" -- request can include a request for specific resource 
/// "General" -- request does not include specific resource 
/// "Both" -- both specific and general requests are supported.
    #[serde(rename = "RequestTypesSupported")]
    pub request_types_supported: Option<AllocationCapabilities_RequestTypesSupported>,

/// A string describing an implementation specific sub-type for this resource. For example, this may be used to distinguish different models of the same resource type.
    #[serde(rename = "ResourceSubType")]
    pub resource_sub_type: Option<String>,

/// The type of resource this allocation setting represents.
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<AllocationCapabilities_ResourceType>,

/// Indicates how access to underlying resource is granted: 
/// "Dedicated" -- exclusive access to underlying resource 
/// "Shared" -- shared use of underlying resource. 
/// Actual quantity is controlled by min, max size, weights, etc.
    #[serde(rename = "SharingMode")]
    pub sharing_mode: Option<AllocationCapabilities_SharingMode>,

/// Indicates the states that the System, to which the resource will be associated via SystemDevice, may be in when a new resource is created.
    #[serde(rename = "SupportedAddStates")]
    pub supported_add_states: Vec<AllocationCapabilities_SupportedAddStates>,

/// Indicates the states that the System, to which the resource is associated via SystemDevice, may be in when a the resource is removed .
    #[serde(rename = "SupportedRemoveStates")]
    pub supported_remove_states: Vec<AllocationCapabilities_SupportedRemoveStates>,
}

impl CIM_AllocationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Capabilities::new(),
            other_resource_type: None,
            request_types_supported: None,
            resource_sub_type: None,
            resource_type: None,
            sharing_mode: None,
            supported_add_states: Vec::new(),
            supported_remove_states: Vec::new(),
        }
    }


    /// Sets the value of OtherResourceType
    pub fn set_other_resource_type(&mut self, value: String) {
        self.other_resource_type = Some(value);
    }

    /// Gets the value of OtherResourceType
    pub fn get_other_resource_type(&self) -> Option<&String> {
        self.other_resource_type.as_ref()
    }

    /// Sets the value of RequestTypesSupported
    pub fn set_request_types_supported(&mut self, value: AllocationCapabilities_RequestTypesSupported) {
        self.request_types_supported = Some(value);
    }

    /// Gets the value of RequestTypesSupported
    pub fn get_request_types_supported(&self) -> Option<&AllocationCapabilities_RequestTypesSupported> {
        self.request_types_supported.as_ref()
    }

    /// Sets the value of ResourceSubType
    pub fn set_resource_sub_type(&mut self, value: String) {
        self.resource_sub_type = Some(value);
    }

    /// Gets the value of ResourceSubType
    pub fn get_resource_sub_type(&self) -> Option<&String> {
        self.resource_sub_type.as_ref()
    }

    /// Sets the value of ResourceType
    pub fn set_resource_type(&mut self, value: AllocationCapabilities_ResourceType) {
        self.resource_type = Some(value);
    }

    /// Gets the value of ResourceType
    pub fn get_resource_type(&self) -> Option<&AllocationCapabilities_ResourceType> {
        self.resource_type.as_ref()
    }

    /// Sets the value of SharingMode
    pub fn set_sharing_mode(&mut self, value: AllocationCapabilities_SharingMode) {
        self.sharing_mode = Some(value);
    }

    /// Gets the value of SharingMode
    pub fn get_sharing_mode(&self) -> Option<&AllocationCapabilities_SharingMode> {
        self.sharing_mode.as_ref()
    }

    /// Sets the value of SupportedAddStates
    pub fn set_supported_add_states(&mut self, value: Vec<AllocationCapabilities_SupportedAddStates>) {
        self.supported_add_states = value;
    }

    /// Gets the value of SupportedAddStates
    pub fn get_supported_add_states(&self) -> &Vec<AllocationCapabilities_SupportedAddStates> {
        &self.supported_add_states
    }

    /// Sets the value of SupportedRemoveStates
    pub fn set_supported_remove_states(&mut self, value: Vec<AllocationCapabilities_SupportedRemoveStates>) {
        self.supported_remove_states = value;
    }

    /// Gets the value of SupportedRemoveStates
    pub fn get_supported_remove_states(&self) -> &Vec<AllocationCapabilities_SupportedRemoveStates> {
        &self.supported_remove_states
    }
}

