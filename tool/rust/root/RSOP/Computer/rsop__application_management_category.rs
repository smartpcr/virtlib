// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ApplicationManagementCategory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ApplicationManagementCategory {

/// 
    #[serde(rename = "CategoryId")]
    pub category_id: Option<String>,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl RSOP_ApplicationManagementCategory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            category_id: None,
            creation_time: None,
            name: None,
        }
    }


    /// Sets the value of CategoryId
    pub fn set_category_id(&mut self, value: String) {
        self.category_id = Some(value);
    }

    /// Gets the value of CategoryId
    pub fn get_category_id(&self) -> Option<&String> {
        self.category_id.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

