// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionIPsecConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionIPsecConfiguration {
}

impl PS_VpnConnectionIPsecConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `authentication_transform_constants` -  (u32)
    /// * `cipher_transform_constants` -  (u32)
    /// * `connection_name` -  (String)
    /// * `dhgroup` -  (u32)
    /// * `encryption_method` -  (u32)
    /// * `force` -  (bool)
    /// * `integrity_check_method` -  (u32)
    /// * `pass_thru` -  (bool)
    /// * `pfs_group` -  (u32)

    /// * `cmdlet_output` -  (VpnConnectionIPsecConfiguration)
    /// * `return_value` -  (u32)
    pub fn set_by_custom_policy(&self, connection_name: &String, authentication_transform_constants: u32, cipher_transform_constants: u32, encryption_method: u32, integrity_check_method: u32, pfs_group: u32, dhgroup: u32, pass_thru: bool, force: bool, all_user_connection: bool, cmdlet_output: &mut VpnConnectionIPsecConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "AuthenticationTransformConstants".to_string(), value: authentication_transform_constants.into() });
        args.push(MethodParameter { name: "CipherTransformConstants".to_string(), value: cipher_transform_constants.into() });
        args.push(MethodParameter { name: "EncryptionMethod".to_string(), value: encryption_method.into() });
        args.push(MethodParameter { name: "IntegrityCheckMethod".to_string(), value: integrity_check_method.into() });
        args.push(MethodParameter { name: "PfsGroup".to_string(), value: pfs_group.into() });
        args.push(MethodParameter { name: "DHGroup".to_string(), value: dhgroup.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });

        let result = self.invoke_method("SetByCustomPolicy", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `connection_name` -  (String)
    /// * `force` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `revert_to_default` -  (bool)

    /// * `cmdlet_output` -  (VpnConnectionIPsecConfiguration)
    /// * `return_value` -  (u32)
    pub fn set_ip_sec_by_default(&self, connection_name: &String, revert_to_default: bool, pass_thru: bool, force: bool, all_user_connection: bool, cmdlet_output: &mut VpnConnectionIPsecConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "RevertToDefault".to_string(), value: revert_to_default.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });

        let result = self.invoke_method("SetIpSecByDefault", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

