// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdObjectElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdObjectElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the guid id of the object this element refers to.
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

impl BcdObjectElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            id: None,
        }
    }


    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}

