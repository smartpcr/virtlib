// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_TargetInformationMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_TargetInformationMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MS_SM_TargetInformationMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_entry_count` -  (u32)

    /// * `entry` -  (MS_SMHBA_SCSIENTRY[])
    /// * `hbastatus` -  (u32)
    /// * `out_entry_count` -  (u32)
    /// * `total_entry_count` -  (u32)
    pub fn sm__get_target_mapping(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, in_entry_count: u32, hbastatus: &mut u32, total_entry_count: &mut u32, out_entry_count: &mut u32, entry: &mut Vec<MS_SMHBA_SCSIENTRY>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });

        let result = self.invoke_method("SM_GetTargetMapping", &args)?;
        let entry = result.get_value("Entry")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_entry_count = result.get_value("OutEntryCount")?;
        let total_entry_count = result.get_value("TotalEntryCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])

    /// * `flags` -  (u32)
    /// * `hbastatus` -  (u32)
    pub fn sm__get_binding_capability(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, hbastatus: &mut u32, flags: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });

        let result = self.invoke_method("SM_GetBindingCapability", &args)?;
        let flags = result.get_value("Flags")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])

    /// * `flags` -  (u32)
    /// * `hbastatus` -  (u32)
    pub fn sm__get_binding_support(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, hbastatus: &mut u32, flags: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });

        let result = self.invoke_method("SM_GetBindingSupport", &args)?;
        let flags = result.get_value("Flags")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `flags` -  (u32)
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__set_binding_support(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, flags: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("SM_SetBindingSupport", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_entry_count` -  (u32)

    /// * `entry` -  (MS_SMHBA_BINDINGENTRY[])
    /// * `hbastatus` -  (u32)
    /// * `out_entry_count` -  (u32)
    /// * `total_entry_count` -  (u32)
    pub fn sm__get_persistent_binding(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, in_entry_count: u32, hbastatus: &mut u32, total_entry_count: &mut u32, out_entry_count: &mut u32, entry: &mut Vec<MS_SMHBA_BINDINGENTRY>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });

        let result = self.invoke_method("SM_GetPersistentBinding", &args)?;
        let entry = result.get_value("Entry")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_entry_count = result.get_value("OutEntryCount")?;
        let total_entry_count = result.get_value("TotalEntryCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `entry` -  (MS_SMHBA_BINDINGENTRY[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_entry_count` -  (u32)

    /// * `entry_status` -  (u32[])
    /// * `hbastatus` -  (u32)
    /// * `out_status_count` -  (u32)
    pub fn sm__set_persistent_binding(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, in_entry_count: u32, entry: &Vec<MS_SMHBA_BINDINGENTRY>, hbastatus: &mut u32, out_status_count: &mut u32, entry_status: &mut Vec<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });
        args.push(MethodParameter { name: "Entry".to_string(), value: entry.into() });

        let result = self.invoke_method("SM_SetPersistentBinding", &args)?;
        let entry_status = result.get_value("EntryStatus")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_status_count = result.get_value("OutStatusCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `entry` -  (MS_SMHBA_BINDINGENTRY[])
    /// * `entry_count` -  (u32)
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__remove_persistent_binding(&self, hba_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, entry_count: u32, entry: &Vec<MS_SMHBA_BINDINGENTRY>, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "EntryCount".to_string(), value: entry_count.into() });
        args.push(MethodParameter { name: "Entry".to_string(), value: entry.into() });

        let result = self.invoke_method("SM_RemovePersistentBinding", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `lunit` -  (HBAScsiID)

    /// * `hbastatus` -  (u32)
    /// * `protocol_statistics` -  (MS_SMHBA_PROTOCOLSTATISTICS)
    pub fn sm__get_lunstatistics(&self, lunit: HBAScsiID, hbastatus: &mut u32, protocol_statistics: &mut MS_SMHBA_PROTOCOLSTATISTICS) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Lunit".to_string(), value: lunit.into() });

        let result = self.invoke_method("SM_GetLUNStatistics", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let protocol_statistics = result.get_value("ProtocolStatistics")?;
        Ok(result.return_value)

    }

}

