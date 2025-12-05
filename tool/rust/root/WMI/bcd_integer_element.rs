// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdIntegerElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdIntegerElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the integer value of the element.
    #[serde(rename = "Integer")]
    pub integer: Option<u64>,
}

impl BcdIntegerElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            integer: None,
        }
    }


    /// Sets the value of Integer
    pub fn set_integer(&mut self, value: u64) {
        self.integer = Some(value);
    }

    /// Gets the value of Integer
    pub fn get_integer(&self) -> Option<&u64> {
        self.integer.as_ref()
    }
}

