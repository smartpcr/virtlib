// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SystemSpecificCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SystemSpecificCollection {
    #[serde(flatten)]
    pub base: CIM_Collection,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,
}

impl CIM_SystemSpecificCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Collection::new(),
            instance_id: None,
        }
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

