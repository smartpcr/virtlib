// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.Security.MicrosoftTpm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Namespace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Namespace {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl __Namespace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            name: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

