// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_FibrePortNPIVMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_FibrePortNPIVMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSFC_FibrePortNPIVMethods {
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

    /// * `tag` -  (u8[])
    /// * `virtual_name` -  (u16[])
    /// * `wwnn` -  (u8[])
    /// * `wwpn` -  (u8[])

    /// * `status` -  (FibrePortNPIVMethods_Status)
    pub fn create_virtual_port(&self, status: &mut FibrePortNPIVMethods_Status, wwpn: &Option<Vec<u8>>, wwnn: &Option<Vec<u8>>, tag: &Option<Vec<u8>>, virtual_name: &Option<Vec<u16>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = wwpn {
            args.push(MethodParameter { name: "WWPN".to_string(), value: val.into() });
        }
        if let Some(val) = wwnn {
            args.push(MethodParameter { name: "WWNN".to_string(), value: val.into() });
        }
        if let Some(val) = tag {
            args.push(MethodParameter { name: "Tag".to_string(), value: val.into() });
        }
        if let Some(val) = virtual_name {
            args.push(MethodParameter { name: "VirtualName".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVirtualPort", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `wwpn` -  (u8[])

    /// * `status` -  (FibrePortNPIVMethods_Status)
    pub fn remove_virtual_port(&self, status: &mut FibrePortNPIVMethods_Status, wwpn: &Option<Vec<u8>>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = wwpn {
            args.push(MethodParameter { name: "WWPN".to_string(), value: val.into() });
        }

        let result = self.invoke_method("RemoveVirtualPort", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

