// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShareToDirectory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShareToDirectory {

/// 
    #[serde(rename = "Share")]
    pub share: Option<Win32_Share>,

/// 
    #[serde(rename = "SharedElement")]
    pub shared_element: Option<CIM_Directory>,
}

impl Win32_ShareToDirectory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            share: None,
            shared_element: None,
        }
    }


    /// Sets the value of Share
    pub fn set_share(&mut self, value: Win32_Share) {
        self.share = Some(value);
    }

    /// Gets the value of Share
    pub fn get_share(&self) -> Option<&Win32_Share> {
        self.share.as_ref()
    }

    /// Sets the value of SharedElement
    pub fn set_shared_element(&mut self, value: CIM_Directory) {
        self.shared_element = Some(value);
    }

    /// Gets the value of SharedElement
    pub fn get_shared_element(&self) -> Option<&CIM_Directory> {
        self.shared_element.as_ref()
    }
}

