// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSettingDataIndexInPlan struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSettingDataIndexInPlan {
    #[serde(flatten)]
    pub base: CIM_ConcreteComponent,
}

impl Win32_PowerSettingDataIndexInPlan {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ConcreteComponent::new(),
        }
    }

}

