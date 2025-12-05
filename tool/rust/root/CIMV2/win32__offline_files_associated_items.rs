// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesAssociatedItems struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesAssociatedItems {

/// 
    #[serde(rename = "Antecedent")]
    pub antecedent: Option<Win32_OfflineFilesCache>,

/// 
    #[serde(rename = "Dependent")]
    pub dependent: Option<Win32_OfflineFilesItem>,
}

impl Win32_OfflineFilesAssociatedItems {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            antecedent: None,
            dependent: None,
        }
    }


    /// Sets the value of Antecedent
    pub fn set_antecedent(&mut self, value: Win32_OfflineFilesCache) {
        self.antecedent = Some(value);
    }

    /// Gets the value of Antecedent
    pub fn get_antecedent(&self) -> Option<&Win32_OfflineFilesCache> {
        self.antecedent.as_ref()
    }

    /// Sets the value of Dependent
    pub fn set_dependent(&mut self, value: Win32_OfflineFilesItem) {
        self.dependent = Some(value);
    }

    /// Gets the value of Dependent
    pub fn get_dependent(&self) -> Option<&Win32_OfflineFilesItem> {
        self.dependent.as_ref()
    }
}

