// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdBooleanElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdBooleanElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the boolean value of the element.
    #[serde(rename = "Boolean")]
    pub boolean: Option<bool>,
}

impl BcdBooleanElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            boolean: None,
        }
    }


    /// Sets the value of Boolean
    pub fn set_boolean(&mut self, value: bool) {
        self.boolean = Some(value);
    }

    /// Gets the value of Boolean
    pub fn get_boolean(&self) -> Option<&bool> {
        self.boolean.as_ref()
    }
}

