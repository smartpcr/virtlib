// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdIntegerListElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdIntegerListElement {
    #[serde(flatten)]
    pub base: BcdElement,

/// This is the array of integer values of the element.
    #[serde(rename = "Integers")]
    pub integers: Vec<u64>,
}

impl BcdIntegerListElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BcdElement::new(),
            integers: Vec::new(),
        }
    }


    /// Sets the value of Integers
    pub fn set_integers(&mut self, value: Vec<u64>) {
        self.integers = value;
    }

    /// Gets the value of Integers
    pub fn get_integers(&self) -> &Vec<u64> {
        &self.integers
    }
}

