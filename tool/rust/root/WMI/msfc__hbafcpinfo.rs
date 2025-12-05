// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_HBAFCPInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_HBAFCPInfo {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSFC_HBAFCPInfo {
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

    /// * `hba_port_wwn` -  (u8[])
    /// * `in_entry_count` -  (u32)

    /// * `entry` -  (HBAFCPScsiEntry[])
    /// * `hbastatus` -  (u32)
    /// * `out_entry_count` -  (u32)
    /// * `total_entry_count` -  (u32)
    pub fn get_fcp_target_mapping(&self, hba_port_wwn: &Vec<u8>, in_entry_count: u32, hbastatus: &mut u32, total_entry_count: &mut u32, out_entry_count: &mut u32, entry: &mut Vec<HBAFCPScsiEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });

        let result = self.invoke_method("GetFcpTargetMapping", &args)?;
        let entry = result.get_value("Entry")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_entry_count = result.get_value("OutEntryCount")?;
        let total_entry_count = result.get_value("TotalEntryCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `in_entry_count` -  (u32)

    /// * `entry` -  (HBAFCPBindingEntry[])
    /// * `hbastatus` -  (u32)
    /// * `out_entry_count` -  (u32)
    /// * `total_entry_count` -  (u32)
    pub fn get_fcp_persistent_binding(&self, in_entry_count: u32, hbastatus: &mut u32, total_entry_count: &mut u32, out_entry_count: &mut u32, entry: &mut Vec<HBAFCPBindingEntry>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });

        let result = self.invoke_method("GetFcpPersistentBinding", &args)?;
        let entry = result.get_value("Entry")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_entry_count = result.get_value("OutEntryCount")?;
        let total_entry_count = result.get_value("TotalEntryCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `port_wwn` -  (u8[])

    /// * `bind_type` -  (u32)
    /// * `hbastatus` -  (u32)
    pub fn get_binding_capability(&self, port_wwn: &Vec<u8>, hbastatus: &mut u32, bind_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });

        let result = self.invoke_method("GetBindingCapability", &args)?;
        let bind_type = result.get_value("BindType")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `port_wwn` -  (u8[])

    /// * `bind_type` -  (u32)
    /// * `hbastatus` -  (u32)
    pub fn get_binding_support(&self, port_wwn: &Vec<u8>, hbastatus: &mut u32, bind_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });

        let result = self.invoke_method("GetBindingSupport", &args)?;
        let bind_type = result.get_value("BindType")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `bind_type` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn set_binding_support(&self, port_wwn: &Vec<u8>, bind_type: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "BindType".to_string(), value: bind_type.into() });

        let result = self.invoke_method("SetBindingSupport", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `in_entry_count` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `bindings` -  (HBAFCPBindingEntry2[])
    /// * `hbastatus` -  (u32)
    /// * `out_entry_count` -  (u32)
    /// * `total_entry_count` -  (u32)
    pub fn get_persistent_binding2(&self, port_wwn: &Vec<u8>, in_entry_count: u32, hbastatus: &mut u32, total_entry_count: &mut u32, out_entry_count: &mut u32, bindings: &mut Vec<HBAFCPBindingEntry2>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "InEntryCount".to_string(), value: in_entry_count.into() });

        let result = self.invoke_method("GetPersistentBinding2", &args)?;
        let bindings = result.get_value("Bindings")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_entry_count = result.get_value("OutEntryCount")?;
        let total_entry_count = result.get_value("TotalEntryCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `binding` -  (HBAFCPBindingEntry2)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn set_persistent_entry(&self, port_wwn: &Vec<u8>, binding: HBAFCPBindingEntry2, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "Binding".to_string(), value: binding.into() });

        let result = self.invoke_method("SetPersistentEntry", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `binding` -  (HBAFCPBindingEntry2)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn remove_persistent_entry(&self, port_wwn: &Vec<u8>, binding: HBAFCPBindingEntry2, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "Binding".to_string(), value: binding.into() });

        let result = self.invoke_method("RemovePersistentEntry", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }

}

