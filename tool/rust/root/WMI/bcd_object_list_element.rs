// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdObjectListElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdObjectListElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the array of object ids this element refers to.
    #[serde(rename = "Ids")]
    pub ids: Vec<String>,
}

impl BcdObjectListElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            ids: Vec::new(),
        }
    }


    /// Sets the value of Ids
    pub fn set_ids(&mut self, value: Vec<String>) {
        self.ids = value;
    }

    /// Gets the value of Ids
    pub fn get_ids(&self) -> &Vec<String> {
        &self.ids
    }
}

