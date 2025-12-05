// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Set2_Flags
//////////////////////////////////////////////

/// Set2_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Set2_Flags {
    /// acquireforsectionsynchronization
    #[serde(rename = "acquireforsectionsynchronization")]
    Acquireforsectionsynchronization = 1,
    /// releaseforsectionsynchronization
    #[serde(rename = "releaseforsectionsynchronization")]
    Releaseforsectionsynchronization = 2,
    /// acquireformodwrite
    #[serde(rename = "acquireformodwrite")]
    Acquireformodwrite = 3,
    /// releaseformodwrite
    #[serde(rename = "releaseformodwrite")]
    Releaseformodwrite = 4,
    /// acquireforccflush
    #[serde(rename = "acquireforccflush")]
    Acquireforccflush = 5,
    /// releaseforccflush
    #[serde(rename = "releaseforccflush")]
    Releaseforccflush = 6,
    /// notifystreamfileobject
    #[serde(rename = "notifystreamfileobject")]
    Notifystreamfileobject = 7,
    /// fastiocheckifpossible
    #[serde(rename = "fastiocheckifpossible")]
    Fastiocheckifpossible = 8,
    /// networkqueryopen
    #[serde(rename = "networkqueryopen")]
    Networkqueryopen = 9,
    /// mdlread
    #[serde(rename = "mdlread")]
    Mdlread = 10,
    /// mdlreadcomplete
    #[serde(rename = "mdlreadcomplete")]
    Mdlreadcomplete = 11,
    /// preparemdlwrite
    #[serde(rename = "preparemdlwrite")]
    Preparemdlwrite = 12,
    /// mdlwritecomplete
    #[serde(rename = "mdlwritecomplete")]
    Mdlwritecomplete = 13,
    /// volumemount
    #[serde(rename = "volumemount")]
    Volumemount = 14,
    /// volumedismount
    #[serde(rename = "volumedismount")]
    Volumedismount = 15,
}

impl Default for Set2_Flags {
    fn default() -> Self {
        Self::Acquireforsectionsynchronization
    }
}

