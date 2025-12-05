// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source OptionalData_Flags
//////////////////////////////////////////////

/// OptionalData_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum OptionalData_Flags {
    /// usercontext
    #[serde(rename = "usercontext")]
    Usercontext = 1,
    /// sessionid
    #[serde(rename = "sessionid")]
    Sessionid = 2,
    /// lastaccesstime
    #[serde(rename = "lastaccesstime")]
    Lastaccesstime = 3,
    /// callparameters
    #[serde(rename = "callparameters")]
    Callparameters = 4,
    /// callresultdata
    #[serde(rename = "callresultdata")]
    Callresultdata = 5,
    /// previousdata
    #[serde(rename = "previousdata")]
    Previousdata = 6,
    /// createonexistingfile
    #[serde(rename = "createonexistingfile")]
    Createonexistingfile = 7,
    /// processwindowstation
    #[serde(rename = "processwindowstation")]
    Processwindowstation = 8,
    /// blockpagingio
    #[serde(rename = "blockpagingio")]
    Blockpagingio = 9,
}

impl Default for OptionalData_Flags {
    fn default() -> Self {
        Self::Usercontext
    }
}

