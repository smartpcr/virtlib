// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Component struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Component {

/// The parent element in the association.
    #[serde(rename = "GroupComponent")]
    pub group_component: Option<CIM_ManagedElement>,

/// The child element in the association.
    #[serde(rename = "PartComponent")]
    pub part_component: Option<CIM_ManagedElement>,
}

impl CIM_Component {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            group_component: None,
            part_component: None,
        }
    }


    /// Sets the value of GroupComponent
    pub fn set_group_component(&mut self, value: CIM_ManagedElement) {
        self.group_component = Some(value);
    }

    /// Gets the value of GroupComponent
    pub fn get_group_component(&self) -> Option<&CIM_ManagedElement> {
        self.group_component.as_ref()
    }

    /// Sets the value of PartComponent
    pub fn set_part_component(&mut self, value: CIM_ManagedElement) {
        self.part_component = Some(value);
    }

    /// Gets the value of PartComponent
    pub fn get_part_component(&self) -> Option<&CIM_ManagedElement> {
        self.part_component.as_ref()
    }
}

