// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_OwningJobElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_OwningJobElement {

/// The Job created by the ManagedElement.
    #[serde(rename = "OwnedElement")]
    pub owned_element: Option<CIM_Job>,

/// The ManagedElement responsible for the creation of the Job.
    #[serde(rename = "OwningElement")]
    pub owning_element: Option<CIM_ManagedElement>,
}

impl CIM_OwningJobElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            owned_element: None,
            owning_element: None,
        }
    }


    /// Sets the value of OwnedElement
    pub fn set_owned_element(&mut self, value: CIM_Job) {
        self.owned_element = Some(value);
    }

    /// Gets the value of OwnedElement
    pub fn get_owned_element(&self) -> Option<&CIM_Job> {
        self.owned_element.as_ref()
    }

    /// Sets the value of OwningElement
    pub fn set_owning_element(&mut self, value: CIM_ManagedElement) {
        self.owning_element = Some(value);
    }

    /// Gets the value of OwningElement
    pub fn get_owning_element(&self) -> Option<&CIM_ManagedElement> {
        self.owning_element.as_ref()
    }
}

