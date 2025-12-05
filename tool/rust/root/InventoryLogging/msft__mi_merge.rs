// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiMerge struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiMerge {
    #[serde(flatten)]
    pub base: Msft_MiStream,

/// 
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Msft_MiStream>,
}

impl Msft_MiMerge {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_MiStream::new(),
            inputs: Vec::new(),
        }
    }


    /// Sets the value of Inputs
    pub fn set_inputs(&mut self, value: Vec<Msft_MiStream>) {
        self.inputs = value;
    }

    /// Gets the value of Inputs
    pub fn get_inputs(&self) -> &Vec<Msft_MiStream> {
        &self.inputs
    }
}

