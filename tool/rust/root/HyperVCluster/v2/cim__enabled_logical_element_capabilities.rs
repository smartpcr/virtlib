// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_EnabledLogicalElementCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_EnabledLogicalElementCapabilities {
    #[serde(flatten)]
    pub base: CIM_Capabilities,

/// Boolean indicating whether the ElementName can be modified.
    #[serde(rename = "ElementNameEditSupported")]
    pub element_name_edit_supported: Option<bool>,

/// This string expresses the restrictions on ElementName.The mask is expressed as a regular expression.See DMTF standard ABNF with the Management Profile Specification Usage Guide, appendix C for the regular expression syntax permitted. 
/// Since the ElementNameMask can describe the maximum length of the ElementName,any length defined in the regexp is in addition to the restriction defined in MaxElementNameLen (causing the smaller value to be the maximum length) The ElementName value satisfies the restriction, if and only if it matches the regular expression
    #[serde(rename = "ElementNameMask")]
    pub element_name_mask: Option<String>,

/// Maximum supported ElementName length.
    #[serde(rename = "MaxElementNameLen")]
    pub max_element_name_len: Option<u16>,

/// RequestedStatesSupported indicates the possible states that can be requested when using the method RequestStateChange on the EnabledLogicalElement.
    #[serde(rename = "RequestedStatesSupported")]
    pub requested_states_supported: Vec<EnabledLogicalElementCapabilities_RequestedStatesSupported>,
}

impl CIM_EnabledLogicalElementCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Capabilities::new(),
            element_name_edit_supported: None,
            element_name_mask: None,
            max_element_name_len: None,
            requested_states_supported: Vec::new(),
        }
    }


    /// Sets the value of ElementNameEditSupported
    pub fn set_element_name_edit_supported(&mut self, value: bool) {
        self.element_name_edit_supported = Some(value);
    }

    /// Gets the value of ElementNameEditSupported
    pub fn get_element_name_edit_supported(&self) -> Option<&bool> {
        self.element_name_edit_supported.as_ref()
    }

    /// Sets the value of ElementNameMask
    pub fn set_element_name_mask(&mut self, value: String) {
        self.element_name_mask = Some(value);
    }

    /// Gets the value of ElementNameMask
    pub fn get_element_name_mask(&self) -> Option<&String> {
        self.element_name_mask.as_ref()
    }

    /// Sets the value of MaxElementNameLen
    pub fn set_max_element_name_len(&mut self, value: u16) {
        self.max_element_name_len = Some(value);
    }

    /// Gets the value of MaxElementNameLen
    pub fn get_max_element_name_len(&self) -> Option<&u16> {
        self.max_element_name_len.as_ref()
    }

    /// Sets the value of RequestedStatesSupported
    pub fn set_requested_states_supported(&mut self, value: Vec<EnabledLogicalElementCapabilities_RequestedStatesSupported>) {
        self.requested_states_supported = value;
    }

    /// Gets the value of RequestedStatesSupported
    pub fn get_requested_states_supported(&self) -> &Vec<EnabledLogicalElementCapabilities_RequestedStatesSupported> {
        &self.requested_states_supported
    }
}

