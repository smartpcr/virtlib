// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Powershellv3
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_ModuleToModuleFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_ModuleToModuleFile {
    #[serde(flatten)]
    pub base: CIM_Dependency,
}

impl PS_ModuleToModuleFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
        }
    }

}

