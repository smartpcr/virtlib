// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CollectionOfMSEs struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CollectionOfMSEs {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "CollectionID")]
    pub collection_id: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

impl CIM_CollectionOfMSEs {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            collection_id: None,
            description: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of CollectionID
    pub fn set_collection_id(&mut self, value: String) {
        self.collection_id = Some(value);
    }

    /// Gets the value of CollectionID
    pub fn get_collection_id(&self) -> Option<&String> {
        self.collection_id.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

