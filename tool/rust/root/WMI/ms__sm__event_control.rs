// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_EventControl struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_EventControl {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MS_SM_EventControl {
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

    /// * `all_targets` -  (u32)
    /// * `discovered_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__add_target(&self, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, all_targets: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "AllTargets".to_string(), value: all_targets.into() });

        let result = self.invoke_method("SM_AddTarget", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_targets` -  (u32)
    /// * `discovered_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__remove_target(&self, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, all_targets: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "AllTargets".to_string(), value: all_targets.into() });

        let result = self.invoke_method("SM_RemoveTarget", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `event_type` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__add_port(&self, port_wwn: &Vec<u8>, event_type: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "EventType".to_string(), value: event_type.into() });

        let result = self.invoke_method("SM_AddPort", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `event_type` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    pub fn sm__remove_port(&self, port_wwn: &Vec<u8>, event_type: u32, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "EventType".to_string(), value: event_type.into() });

        let result = self.invoke_method("SM_RemovePort", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `hbastatus` -  (u32)
    pub fn sm__add_link(&self, hbastatus: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("SM_AddLink", &[])?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `hbastatus` -  (u32)
    pub fn sm__remove_link(&self, hbastatus: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("SM_RemoveLink", &[])?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }

}

