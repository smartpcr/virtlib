// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationServiceSettingData_AuthenticationType
//////////////////////////////////////////////

/// VirtualSystemMigrationServiceSettingData_AuthenticationType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationServiceSettingData_AuthenticationType {
    /// CredSSP
    #[serde(rename = "CredSSP")]
    CredSSP = 0,
    /// Kerberos
    #[serde(rename = "Kerberos")]
    Kerberos = 1,
}

impl Default for VirtualSystemMigrationServiceSettingData_AuthenticationType {
    fn default() -> Self {
        Self::CredSSP
    }
}

