// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ComputerSystem {
    #[serde(flatten)]
    pub base: CIM_System,

/// 
    #[serde(rename = "Dedicated")]
    pub dedicated: Vec<u16>,

/// 
    #[serde(rename = "IdentifyingDescriptions")]
    pub identifying_descriptions: Vec<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,
}

impl CIM_ComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_System::new(),
            dedicated: Vec::new(),
            identifying_descriptions: Vec::new(),
            other_identifying_info: Vec::new(),
        }
    }


    /// Sets the value of Dedicated
    pub fn set_dedicated(&mut self, value: Vec<u16>) {
        self.dedicated = value;
    }

    /// Gets the value of Dedicated
    pub fn get_dedicated(&self) -> &Vec<u16> {
        &self.dedicated
    }

    /// Sets the value of IdentifyingDescriptions
    pub fn set_identifying_descriptions(&mut self, value: Vec<String>) {
        self.identifying_descriptions = value;
    }

    /// Gets the value of IdentifyingDescriptions
    pub fn get_identifying_descriptions(&self) -> &Vec<String> {
        &self.identifying_descriptions
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }
}

