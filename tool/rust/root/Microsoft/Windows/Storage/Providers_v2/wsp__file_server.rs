// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSP_FileServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSP_FileServer {
    #[serde(flatten)]
    pub base: MSFT_FileServer,
}

impl WSP_FileServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_FileServer::new(),
        }
    }


/// 

    /// * `file_sharing_protocols` -  (u16[])
    /// * `friendly_name` -  (String)
    /// * `host_names` -  (String[])
    /// * `storage_sub_system_id` -  (String)

    /// * `created_file_server` -  (MSFT_FileServer)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_file_server(&self, storage_sub_system_id: &String, friendly_name: &String, file_sharing_protocols: &Vec<u16>, host_names: &Vec<String>, created_file_server: &mut MSFT_FileServer, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageSubSystemId".to_string(), value: storage_sub_system_id.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "FileSharingProtocols".to_string(), value: file_sharing_protocols.into() });
        args.push(MethodParameter { name: "HostNames".to_string(), value: host_names.into() });

        let result = self.invoke_method("CreateFileServer", &args)?;
        let created_file_server = result.get_value("CreatedFileServer")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

