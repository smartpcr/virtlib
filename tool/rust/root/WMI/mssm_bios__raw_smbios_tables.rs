// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSmBios_RawSMBiosTables struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSmBios_RawSMBiosTables {
    #[serde(flatten)]
    pub base: MS_SmBios,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DmiRevision")]
    pub dmi_revision: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "SMBiosData")]
    pub smbios_data: Vec<u8>,

/// 
    #[serde(rename = "SmbiosMajorVersion")]
    pub smbios_major_version: Option<u8>,

/// 
    #[serde(rename = "SmbiosMinorVersion")]
    pub smbios_minor_version: Option<u8>,

/// 
    #[serde(rename = "Used20CallingMethod")]
    pub used20_calling_method: Option<bool>,
}

impl MSSmBios_RawSMBiosTables {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MS_SmBios::new(),
            active: None,
            dmi_revision: None,
            instance_name: None,
            size: None,
            smbios_data: Vec::new(),
            smbios_major_version: None,
            smbios_minor_version: None,
            used20_calling_method: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of DmiRevision
    pub fn set_dmi_revision(&mut self, value: u8) {
        self.dmi_revision = Some(value);
    }

    /// Gets the value of DmiRevision
    pub fn get_dmi_revision(&self) -> Option<&u8> {
        self.dmi_revision.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of SMBiosData
    pub fn set_smbios_data(&mut self, value: Vec<u8>) {
        self.smbios_data = value;
    }

    /// Gets the value of SMBiosData
    pub fn get_smbios_data(&self) -> &Vec<u8> {
        &self.smbios_data
    }

    /// Sets the value of SmbiosMajorVersion
    pub fn set_smbios_major_version(&mut self, value: u8) {
        self.smbios_major_version = Some(value);
    }

    /// Gets the value of SmbiosMajorVersion
    pub fn get_smbios_major_version(&self) -> Option<&u8> {
        self.smbios_major_version.as_ref()
    }

    /// Sets the value of SmbiosMinorVersion
    pub fn set_smbios_minor_version(&mut self, value: u8) {
        self.smbios_minor_version = Some(value);
    }

    /// Gets the value of SmbiosMinorVersion
    pub fn get_smbios_minor_version(&self) -> Option<&u8> {
        self.smbios_minor_version.as_ref()
    }

    /// Sets the value of Used20CallingMethod
    pub fn set_used20_calling_method(&mut self, value: bool) {
        self.used20_calling_method = Some(value);
    }

    /// Gets the value of Used20CallingMethod
    pub fn get_used20_calling_method(&self) -> Option<&bool> {
        self.used20_calling_method.as_ref()
    }
}

