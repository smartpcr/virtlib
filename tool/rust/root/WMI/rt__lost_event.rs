// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RT_LostEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RT_LostEvent {
    #[serde(flatten)]
    pub base: Lost_Event,
}

impl RT_LostEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Lost_Event::new(),
        }
    }

}

