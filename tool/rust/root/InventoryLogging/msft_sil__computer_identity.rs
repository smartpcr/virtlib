// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_ComputerIdentity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_ComputerIdentity {
    #[serde(flatten)]
    pub base: MsftSil_Data,

/// 
    #[serde(rename = "HypervisorHostName")]
    pub hypervisor_host_name: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "UUID")]
    pub uuid: Option<String>,

/// 
    #[serde(rename = "VMGUID")]
    pub vmguid: Option<String>,
}

impl MsftSil_ComputerIdentity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsftSil_Data::new(),
            hypervisor_host_name: None,
            id: None,
            name: None,
            uuid: None,
            vmguid: None,
        }
    }


    /// Sets the value of HypervisorHostName
    pub fn set_hypervisor_host_name(&mut self, value: String) {
        self.hypervisor_host_name = Some(value);
    }

    /// Gets the value of HypervisorHostName
    pub fn get_hypervisor_host_name(&self) -> Option<&String> {
        self.hypervisor_host_name.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of UUID
    pub fn set_uuid(&mut self, value: String) {
        self.uuid = Some(value);
    }

    /// Gets the value of UUID
    pub fn get_uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }

    /// Sets the value of VMGUID
    pub fn set_vmguid(&mut self, value: String) {
        self.vmguid = Some(value);
    }

    /// Gets the value of VMGUID
    pub fn get_vmguid(&self) -> Option<&String> {
        self.vmguid.as_ref()
    }
}

