// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorId {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "HostType")]
    pub host_type: Vec<u16>,

/// 
    #[serde(rename = "InitiatorAddress")]
    pub initiator_address: Option<String>,

/// 
    #[serde(rename = "OtherHostTypeDescription")]
    pub other_host_type_description: Vec<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,
}

impl MSFT_InitiatorId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            host_type: Vec::new(),
            initiator_address: None,
            other_host_type_description: Vec::new(),
            type: None,
        }
    }


    /// Sets the value of HostType
    pub fn set_host_type(&mut self, value: Vec<u16>) {
        self.host_type = value;
    }

    /// Gets the value of HostType
    pub fn get_host_type(&self) -> &Vec<u16> {
        &self.host_type
    }

    /// Sets the value of InitiatorAddress
    pub fn set_initiator_address(&mut self, value: String) {
        self.initiator_address = Some(value);
    }

    /// Gets the value of InitiatorAddress
    pub fn get_initiator_address(&self) -> Option<&String> {
        self.initiator_address.as_ref()
    }

    /// Sets the value of OtherHostTypeDescription
    pub fn set_other_host_type_description(&mut self, value: Vec<String>) {
        self.other_host_type_description = value;
    }

    /// Gets the value of OtherHostTypeDescription
    pub fn get_other_host_type_description(&self) -> &Vec<String> {
        &self.other_host_type_description
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

