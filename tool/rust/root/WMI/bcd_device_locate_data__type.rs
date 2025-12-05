// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BcdDeviceLocateData_Type
//////////////////////////////////////////////

/// BcdDeviceLocateData_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BcdDeviceLocateData_Type {
    /// Element
    #[serde(rename = "Element")]
    Element = 0,
    /// String
    #[serde(rename = "String")]
    String = 1,
    /// ElementChild
    #[serde(rename = "ElementChild")]
    ElementChild = 2,
}

impl Default for BcdDeviceLocateData_Type {
    fn default() -> Self {
        Self::Element
    }
}

