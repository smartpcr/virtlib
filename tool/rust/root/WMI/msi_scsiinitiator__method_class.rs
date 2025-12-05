// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_MethodClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_MethodClass {

/// 
    #[serde(rename = "iSCSINodeName")]
    pub i_scsinode_name: Option<String>,
}

impl MSiSCSIInitiator_MethodClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsinode_name: None,
        }
    }


    /// Sets the value of iSCSINodeName
    pub fn set_i_scsinode_name(&mut self, value: String) {
        self.i_scsinode_name = Some(value);
    }

    /// Gets the value of iSCSINodeName
    pub fn get_i_scsinode_name(&self) -> Option<&String> {
        self.i_scsinode_name.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn refresh_target_list(&self) -> Result<(), WmiError> {
        self.invoke_method("RefreshTargetList", &[])

    }


/// 

    /// * `destination_address` -  (String)
    /// * `initiator_name` -  (String)
    /// * `initiator_port_number` -  (u32)
    /// * `outer_mode_address` -  (String)
    /// * `persist` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_iscsi_tunnel_mode_outer_address(&self, initiator_name: &String, initiator_port_number: u32, destination_address: &String, outer_mode_address: &String, persist: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorName".to_string(), value: initiator_name.into() });
        args.push(MethodParameter { name: "InitiatorPortNumber".to_string(), value: initiator_port_number.into() });
        args.push(MethodParameter { name: "DestinationAddress".to_string(), value: destination_address.into() });
        args.push(MethodParameter { name: "OuterModeAddress".to_string(), value: outer_mode_address.into() });
        args.push(MethodParameter { name: "Persist".to_string(), value: persist.into() });
        self.invoke_method("SetIScsiTunnelModeOuterAddress", &args)

    }


/// 

    /// * `auth_info` -  (MSiSCSIInitiator_IKEPresharedKeyAuthenticationInfo)
    /// * `initiator_name` -  (String)
    /// * `initiator_port_number` -  (u32)
    /// * `persist` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_iscsi_ikeinfo(&self, initiator_name: &String, initiator_port_number: u32, auth_info: MSiSCSIInitiator_IKEPresharedKeyAuthenticationInfo, persist: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorName".to_string(), value: initiator_name.into() });
        args.push(MethodParameter { name: "InitiatorPortNumber".to_string(), value: initiator_port_number.into() });
        args.push(MethodParameter { name: "AuthInfo".to_string(), value: auth_info.into() });
        args.push(MethodParameter { name: "Persist".to_string(), value: persist.into() });
        self.invoke_method("SetIScsiIKEInfo", &args)

    }


/// 

    /// * `key` -  (u8[])
    /// * `persist` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_iscsi_group_preshared_key(&self, key: &Vec<u8>, persist: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });
        args.push(MethodParameter { name: "Persist".to_string(), value: persist.into() });
        self.invoke_method("SetIScsiGroupPresharedKey", &args)

    }


/// 

    /// * `shared_secret` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn set_iscsi_initiator_chapshared_secret(&self, shared_secret: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SharedSecret".to_string(), value: shared_secret.into() });
        self.invoke_method("SetIScsiInitiatorCHAPSharedSecret", &args)

    }


/// 

    /// * `shared_secret` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn set_iscsi_initiator_radiusshared_secret(&self, shared_secret: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SharedSecret".to_string(), value: shared_secret.into() });
        self.invoke_method("SetIScsiInitiatorRADIUSSharedSecret", &args)

    }


/// 

    /// * `initiator_node_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_iscsi_initiator_node_name(&self, initiator_node_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorNodeName".to_string(), value: initiator_node_name.into() });
        self.invoke_method("SetIScsiInitiatorNodeName", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn setup_persistent_iscsi_volumes(&self) -> Result<(), WmiError> {
        self.invoke_method("SetupPersistentIScsiVolumes", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_persistent_iscsi_volumes(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearPersistentIScsiVolumes", &[])

    }

}

