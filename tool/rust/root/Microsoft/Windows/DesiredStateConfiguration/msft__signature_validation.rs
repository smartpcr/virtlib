// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SignatureValidation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SignatureValidation {
    #[serde(flatten)]
    pub base: OMI_MetaConfigurationResource,

/// 
    #[serde(rename = "SignedItemType")]
    pub signed_item_type: Vec<String>,

/// 
    #[serde(rename = "TrustedStorePath")]
    pub trusted_store_path: Option<String>,
}

impl MSFT_SignatureValidation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_MetaConfigurationResource::new(),
            signed_item_type: Vec::new(),
            trusted_store_path: None,
        }
    }


    /// Sets the value of SignedItemType
    pub fn set_signed_item_type(&mut self, value: Vec<String>) {
        self.signed_item_type = value;
    }

    /// Gets the value of SignedItemType
    pub fn get_signed_item_type(&self) -> &Vec<String> {
        &self.signed_item_type
    }

    /// Sets the value of TrustedStorePath
    pub fn set_trusted_store_path(&mut self, value: String) {
        self.trusted_store_path = Some(value);
    }

    /// Gets the value of TrustedStorePath
    pub fn get_trusted_store_path(&self) -> Option<&String> {
        self.trusted_store_path.as_ref()
    }
}

