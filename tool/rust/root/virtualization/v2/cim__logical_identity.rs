// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalIdentity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalIdentity {

/// SameElement represents an alternate aspect of the ManagedElement.
    #[serde(rename = "SameElement")]
    pub same_element: Option<CIM_ManagedElement>,

/// SystemElement represents one aspect of the Managed Element. The use of 'System' in the role name does not limit the scope of the association. The role name was defined in the original association, where the referenced elements were limited to LogicalElements. Since that time, it has been found valuable to instantiate these types of relationships for ManagedElements, such as Collections. So, the referenced elements of the association were redefined to be ManagedElements. Unfortunately, the role name could not be changed without deprecating the entire association. This was not deemed necessary just to correct the role name.
    #[serde(rename = "SystemElement")]
    pub system_element: Option<CIM_ManagedElement>,
}

impl CIM_LogicalIdentity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            same_element: None,
            system_element: None,
        }
    }


    /// Sets the value of SameElement
    pub fn set_same_element(&mut self, value: CIM_ManagedElement) {
        self.same_element = Some(value);
    }

    /// Gets the value of SameElement
    pub fn get_same_element(&self) -> Option<&CIM_ManagedElement> {
        self.same_element.as_ref()
    }

    /// Sets the value of SystemElement
    pub fn set_system_element(&mut self, value: CIM_ManagedElement) {
        self.system_element = Some(value);
    }

    /// Gets the value of SystemElement
    pub fn get_system_element(&self) -> Option<&CIM_ManagedElement> {
        self.system_element.as_ref()
    }
}

