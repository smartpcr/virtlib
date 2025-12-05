// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdElement {

/// This is the id of the object that this element belongs to.
    #[serde(rename = "ObjectId")]
    pub object_id: Option<String>,

/// This is the file path of the store that this element is a part of.
    #[serde(rename = "StoreFilePath")]
    pub store_file_path: Option<String>,

/// The upper 4 bits (28-31) determine the class of the element. The next 4 bits (24-27) determine the format of the element data. The lower 24 bits (0-23) determine the sub-type of the element.
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl BcdElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            object_id: None,
            store_file_path: None,
            type: None,
        }
    }


    /// Sets the value of ObjectId
    pub fn set_object_id(&mut self, value: String) {
        self.object_id = Some(value);
    }

    /// Gets the value of ObjectId
    pub fn get_object_id(&self) -> Option<&String> {
        self.object_id.as_ref()
    }

    /// Sets the value of StoreFilePath
    pub fn set_store_file_path(&mut self, value: String) {
        self.store_file_path = Some(value);
    }

    /// Gets the value of StoreFilePath
    pub fn get_store_file_path(&self) -> Option<&String> {
        self.store_file_path.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

