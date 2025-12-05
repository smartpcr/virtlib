// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdStringElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdStringElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the string value of the element.
    #[serde(rename = "String")]
    pub string: Option<String>,
}

impl BcdStringElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            string: None,
        }
    }


    /// Sets the value of String
    pub fn set_string(&mut self, value: String) {
        self.string = Some(value);
    }

    /// Gets the value of String
    pub fn get_string(&self) -> Option<&String> {
        self.string.as_ref()
    }
}

