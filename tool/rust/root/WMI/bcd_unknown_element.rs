// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdUnknownElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdUnknownElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the actual value of the element inside the BCD store.
    #[serde(rename = "ActualType")]
    pub actual_type: Option<u32>,
}

impl BcdUnknownElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            actual_type: None,
        }
    }


    /// Sets the value of ActualType
    pub fn set_actual_type(&mut self, value: u32) {
        self.actual_type = Some(value);
    }

    /// Gets the value of ActualType
    pub fn get_actual_type(&self) -> Option<&u32> {
        self.actual_type.as_ref()
    }
}

