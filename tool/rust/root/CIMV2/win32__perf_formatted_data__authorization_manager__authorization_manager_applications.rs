// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_AuthorizationManager_AuthorizationManagerApplications struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_AuthorizationManager_AuthorizationManagerApplications {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "NumberofScopesloadedinmemory")]
    pub numberof_scopesloadedinmemory: Option<u32>,

/// 
    #[serde(rename = "Totalnumberofscopes")]
    pub totalnumberofscopes: Option<u32>,
}

impl Win32_PerfFormattedData_AuthorizationManager_AuthorizationManagerApplications {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            numberof_scopesloadedinmemory: None,
            totalnumberofscopes: None,
        }
    }


    /// Sets the value of NumberofScopesloadedinmemory
    pub fn set_numberof_scopesloadedinmemory(&mut self, value: u32) {
        self.numberof_scopesloadedinmemory = Some(value);
    }

    /// Gets the value of NumberofScopesloadedinmemory
    pub fn get_numberof_scopesloadedinmemory(&self) -> Option<&u32> {
        self.numberof_scopesloadedinmemory.as_ref()
    }

    /// Sets the value of Totalnumberofscopes
    pub fn set_totalnumberofscopes(&mut self, value: u32) {
        self.totalnumberofscopes = Some(value);
    }

    /// Gets the value of Totalnumberofscopes
    pub fn get_totalnumberofscopes(&self) -> Option<&u32> {
        self.totalnumberofscopes.as_ref()
    }
}

