// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProductParentChild struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProductParentChild {

/// 
    #[serde(rename = "Child")]
    pub child: Option<CIM_Product>,

/// 
    #[serde(rename = "Parent")]
    pub parent: Option<CIM_Product>,
}

impl CIM_ProductParentChild {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            child: None,
            parent: None,
        }
    }


    /// Sets the value of Child
    pub fn set_child(&mut self, value: CIM_Product) {
        self.child = Some(value);
    }

    /// Gets the value of Child
    pub fn get_child(&self) -> Option<&CIM_Product> {
        self.child.as_ref()
    }

    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: CIM_Product) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&CIM_Product> {
        self.parent.as_ref()
    }
}

