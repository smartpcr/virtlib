// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V2_TypeGroup5 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V2_TypeGroup5 {
    #[serde(flatten)]
    pub base: Process_V2,

/// 
    #[serde(rename = "Object")]
    pub object: Option<u32>,
}

impl Process_V2_TypeGroup5 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V2::new(),
            object: None,
        }
    }


    /// Sets the value of Object
    pub fn set_object(&mut self, value: u32) {
        self.object = Some(value);
    }

    /// Gets the value of Object
    pub fn get_object(&self) -> Option<&u32> {
        self.object.as_ref()
    }
}

