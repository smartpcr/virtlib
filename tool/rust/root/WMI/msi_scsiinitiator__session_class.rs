// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_SessionClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_SessionClass {

/// Information about the connections for this session
    #[serde(rename = "ConnectionInformation")]
    pub connection_information: Vec<MSiSCSIInitiator_ConnectionInformation>,

/// Information about the devices exposed by this session
    #[serde(rename = "Devices")]
    pub devices: Vec<MSiSCSIInitiator_DeviceOnSession>,

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,

/// 
    #[serde(rename = "ISID")]
    pub isid: Vec<u8>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<String>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,

/// 
    #[serde(rename = "TargetNodeName")]
    pub target_node_name: Option<String>,

/// 
    #[serde(rename = "TSID")]
    pub tsid: Vec<u8>,
}

impl MSiSCSIInitiator_SessionClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_information: Vec::new(),
            devices: Vec::new(),
            initiator_name: None,
            isid: Vec::new(),
            session_id: None,
            target_name: None,
            target_node_name: None,
            tsid: Vec::new(),
        }
    }


    /// Sets the value of ConnectionInformation
    pub fn set_connection_information(&mut self, value: Vec<MSiSCSIInitiator_ConnectionInformation>) {
        self.connection_information = value;
    }

    /// Gets the value of ConnectionInformation
    pub fn get_connection_information(&self) -> &Vec<MSiSCSIInitiator_ConnectionInformation> {
        &self.connection_information
    }

    /// Sets the value of Devices
    pub fn set_devices(&mut self, value: Vec<MSiSCSIInitiator_DeviceOnSession>) {
        self.devices = value;
    }

    /// Gets the value of Devices
    pub fn get_devices(&self) -> &Vec<MSiSCSIInitiator_DeviceOnSession> {
        &self.devices
    }

    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }

    /// Sets the value of ISID
    pub fn set_isid(&mut self, value: Vec<u8>) {
        self.isid = value;
    }

    /// Gets the value of ISID
    pub fn get_isid(&self) -> &Vec<u8> {
        &self.isid
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: String) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of TargetNodeName
    pub fn set_target_node_name(&mut self, value: String) {
        self.target_node_name = Some(value);
    }

    /// Gets the value of TargetNodeName
    pub fn get_target_node_name(&self) -> Option<&String> {
        self.target_node_name.as_ref()
    }

    /// Sets the value of TSID
    pub fn set_tsid(&mut self, value: Vec<u8>) {
        self.tsid = value;
    }

    /// Gets the value of TSID
    pub fn get_tsid(&self) -> &Vec<u8> {
        &self.tsid
    }

/// 

    /// * `return_value` -  (u32)
    pub fn logout(&self) -> Result<(), WmiError> {
        self.invoke_method("Logout", &[])

    }


/// 

    /// * `evpd_cmddt` -  (u8)
    /// * `lun` -  (u64)
    /// * `page_code` -  (u8)

    /// * `response_buffer` -  (u8[])
    /// * `return_value` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    pub fn send_scsi_inquiry(&self, lun: u64, evpd_cmddt: u8, page_code: u8, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Lun".to_string(), value: lun.into() });
        args.push(MethodParameter { name: "EvpdCmddt".to_string(), value: evpd_cmddt.into() });
        args.push(MethodParameter { name: "PageCode".to_string(), value: page_code.into() });

        let result = self.invoke_method("SendScsiInquiry", &args)?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        Ok(result.return_value)

    }


/// 

    /// * `lun` -  (u64)

    /// * `response_buffer` -  (u8[])
    /// * `return_value` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    pub fn send_scsi_read_capacity(&self, lun: u64, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Lun".to_string(), value: lun.into() });

        let result = self.invoke_method("SendScsiReadCapacity", &args)?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        Ok(result.return_value)

    }


/// 

    /// * `response_buffer` -  (u8[])
    /// * `return_value` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    pub fn send_scsi_report_luns(&self, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("SendScsiReportLuns", &[])?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        Ok(result.return_value)

    }


/// 

    /// * `initiator_port_number` -  (u32)
    /// * `key` -  (u8[])
    /// * `login_options` -  (MSiSCSIInitiator_TargetLoginOptions)
    /// * `security_flags` -  (u64)
    /// * `target_portal` -  (MSiSCSIInitiator_Portal)

    /// * `return_value` -  (u32)
    /// * `unique_connection_id` -  (String)
    pub fn add_connection(&self, initiator_port_number: u32, target_portal: MSiSCSIInitiator_Portal, security_flags: u64, login_options: MSiSCSIInitiator_TargetLoginOptions, key: &Vec<u8>, unique_connection_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitiatorPortNumber".to_string(), value: initiator_port_number.into() });
        args.push(MethodParameter { name: "TargetPortal".to_string(), value: target_portal.into() });
        args.push(MethodParameter { name: "SecurityFlags".to_string(), value: security_flags.into() });
        args.push(MethodParameter { name: "LoginOptions".to_string(), value: login_options.into() });
        args.push(MethodParameter { name: "key".to_string(), value: key.into() });

        let result = self.invoke_method("AddConnection", &args)?;
        let unique_connection_id = result.get_value("UniqueConnectionId")?;
        Ok(result.return_value)

    }


/// 

    /// * `unique_connection_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_connection(&self, unique_connection_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UniqueConnectionId".to_string(), value: unique_connection_id.into() });
        self.invoke_method("RemoveConnection", &args)

    }

}

