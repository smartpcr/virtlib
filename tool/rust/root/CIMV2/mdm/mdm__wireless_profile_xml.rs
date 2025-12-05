// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WirelessProfileXml struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WirelessProfileXml {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProfileXml")]
    pub profile_xml: Option<String>,
}

impl MDM_WirelessProfileXml {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            profile_xml: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProfileXml
    pub fn set_profile_xml(&mut self, value: String) {
        self.profile_xml = Some(value);
    }

    /// Gets the value of ProfileXml
    pub fn get_profile_xml(&self) -> Option<&String> {
        self.profile_xml.as_ref()
    }
}

