// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __IndicationRelated struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __IndicationRelated {
    #[serde(flatten)]
    pub base: __SystemClass,
}

impl __IndicationRelated {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
        }
    }

}

