// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderBasicEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderBasicEvent {
    #[serde(flatten)]
    pub base: Image_V2,
}

impl LoaderBasicEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image_V2::new(),
        }
    }

}

