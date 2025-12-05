// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_HBAAdapterMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_HBAAdapterMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSFC_HBAAdapterMethods {
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

    /// * `discovered_port_index` -  (u32)
    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `port_attributes` -  (MSFC_HBAPortAttributesResults)
    pub fn get_discovered_port_attributes(&self, port_index: u32, discovered_port_index: u32, hbastatus: &mut u32, port_attributes: &mut MSFC_HBAPortAttributesResults) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "DiscoveredPortIndex".to_string(), value: discovered_port_index.into() });

        let result = self.invoke_method("GetDiscoveredPortAttributes", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_attributes = result.get_value("PortAttributes")?;
        Ok(result.return_value)

    }


/// 

    /// * `wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `port_attributes` -  (MSFC_HBAPortAttributesResults)
    pub fn get_port_attributes_by_wwn(&self, wwn: &Vec<u8>, hbastatus: &mut u32, port_attributes: &mut MSFC_HBAPortAttributesResults) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "wwn".to_string(), value: wwn.into() });

        let result = self.invoke_method("GetPortAttributesByWWN", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_attributes = result.get_value("PortAttributes")?;
        Ok(result.return_value)

    }


/// 
    pub fn refresh_information(&self) -> Result<(), WmiError> {
        self.invoke_method("RefreshInformation", &[])

    }


/// 

    /// * `port_wwn` -  (u8[])
    /// * `request_buffer` -  (u8[])
    /// * `request_buffer_count` -  (u32)

    /// * `actual_response_buffer_count` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `response_buffer` -  (u8[])
    /// * `total_response_buffer_count` -  (u32)
    pub fn send_ctpass_thru(&self, port_wwn: &Vec<u8>, request_buffer_count: u32, request_buffer: &Vec<u8>, hbastatus: &mut u32, total_response_buffer_count: &mut u32, actual_response_buffer_count: &mut u32, response_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "RequestBufferCount".to_string(), value: request_buffer_count.into() });
        args.push(MethodParameter { name: "RequestBuffer".to_string(), value: request_buffer.into() });

        let result = self.invoke_method("SendCTPassThru", &args)?;
        let actual_response_buffer_count = result.get_value("ActualResponseBufferCount")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let total_response_buffer_count = result.get_value("TotalResponseBufferCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `wwn` -  (u8[])
    /// * `wwntype` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `response_buffer` -  (u8[])
    /// * `response_buffer_count` -  (u32)
    pub fn send_rnid(&self, wwn: &Vec<u8>, wwntype: u32, hbastatus: &mut u32, response_buffer_count: &mut u32, response_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "wwn".to_string(), value: wwn.into() });
        args.push(MethodParameter { name: "wwntype".to_string(), value: wwntype.into() });

        let result = self.invoke_method("SendRNID", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let response_buffer_count = result.get_value("ResponseBufferCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_fcid` -  (u32)
    /// * `dest_wwn` -  (u8[])
    /// * `node_id_data_format` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_rnidv2(&self, port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, dest_fcid: u32, node_id_data_format: u32, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "DestFCID".to_string(), value: dest_fcid.into() });
        args.push(MethodParameter { name: "NodeIdDataFormat".to_string(), value: node_id_data_format.into() });

        let result = self.invoke_method("SendRNIDV2", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `hbastatus` -  (u32)
    /// * `mgmt_info` -  (HBAFC3MgmtInfo)
    pub fn get_fc3_mgmt_info(&self, hbastatus: &mut u32, mgmt_info: &mut HBAFC3MgmtInfo) -> Result<(), WmiError> {

        let result = self.invoke_method("GetFC3MgmtInfo", &[])?;
        let hbastatus = result.get_value("HBAStatus")?;
        let mgmt_info = result.get_value("MgmtInfo")?;
        Ok(result.return_value)

    }


/// 

    /// * `mgmt_info` -  (HBAFC3MgmtInfo)

    /// * `hbastatus` -  (u32)
    pub fn set_fc3_mgmt_info(&self, mgmt_info: HBAFC3MgmtInfo, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MgmtInfo".to_string(), value: mgmt_info.into() });

        let result = self.invoke_method("SetFC3MgmtInfo", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `agent_domain` -  (u32)
    /// * `agent_wwn` -  (u8[])
    /// * `port_index` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_rpl(&self, port_wwn: &Vec<u8>, agent_wwn: &Vec<u8>, agent_domain: u32, port_index: u32, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "AgentWWN".to_string(), value: agent_wwn.into() });
        args.push(MethodParameter { name: "agent_domain".to_string(), value: agent_domain.into() });
        args.push(MethodParameter { name: "portIndex".to_string(), value: port_index.into() });

        let result = self.invoke_method("SendRPL", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `agent_domain` -  (u32)
    /// * `agent_wwn` -  (u8[])
    /// * `object_port_number` -  (u32)
    /// * `object_wwn` -  (u8[])
    /// * `port_wwn` -  (u8[])

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_rps(&self, port_wwn: &Vec<u8>, agent_wwn: &Vec<u8>, object_wwn: &Vec<u8>, agent_domain: u32, object_port_number: u32, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "AgentWWN".to_string(), value: agent_wwn.into() });
        args.push(MethodParameter { name: "ObjectWWN".to_string(), value: object_wwn.into() });
        args.push(MethodParameter { name: "AgentDomain".to_string(), value: agent_domain.into() });
        args.push(MethodParameter { name: "ObjectPortNumber".to_string(), value: object_port_number.into() });

        let result = self.invoke_method("SendRPS", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain` -  (u32)
    /// * `port_wwn` -  (u8[])
    /// * `wwn` -  (u8[])

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_srl(&self, port_wwn: &Vec<u8>, wwn: &Vec<u8>, domain: u32, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "WWN".to_string(), value: wwn.into() });
        args.push(MethodParameter { name: "Domain".to_string(), value: domain.into() });

        let result = self.invoke_method("SendSRL", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_wwn` -  (u8[])
    /// * `function` -  (u8)
    /// * `source_wwn` -  (u8[])
    /// * `type` -  (u8)

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_lirr(&self, source_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, function: u8, type: u8, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceWWN".to_string(), value: source_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "Function".to_string(), value: function.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });

        let result = self.invoke_method("SendLIRR", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `fc4_type` -  (u8)
    /// * `port_wwn` -  (u8[])

    /// * `fc4_statistics` -  (MSFC_FC4STATISTICS)
    /// * `hbastatus` -  (u32)
    pub fn get_fc4_statistics(&self, port_wwn: &Vec<u8>, fc4_type: u8, hbastatus: &mut u32, fc4_statistics: &mut MSFC_FC4STATISTICS) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "FC4Type".to_string(), value: fc4_type.into() });

        let result = self.invoke_method("GetFC4Statistics", &args)?;
        let fc4_statistics = result.get_value("FC4Statistics")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `scsi_id` -  (HBAScsiID)

    /// * `fc4_statistics` -  (MSFC_FC4STATISTICS)
    /// * `hbastatus` -  (u32)
    pub fn get_fcpstatistics(&self, scsi_id: HBAScsiID, hbastatus: &mut u32, fc4_statistics: &mut MSFC_FC4STATISTICS) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScsiId".to_string(), value: scsi_id.into() });

        let result = self.invoke_method("GetFCPStatistics", &args)?;
        let fc4_statistics = result.get_value("FC4Statistics")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `fc_lun` -  (u64)
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `response_buffer` -  (u8[])
    /// * `response_buffer_size` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    /// * `sense_buffer_size` -  (u32)
    pub fn scsi_inquiry(&self, cdb: &Vec<u8>, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, fc_lun: u64, hbastatus: &mut u32, response_buffer_size: &mut u32, sense_buffer_size: &mut u32, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "FcLun".to_string(), value: fc_lun.into() });

        let result = self.invoke_method("ScsiInquiry", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let response_buffer_size = result.get_value("ResponseBufferSize")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        let sense_buffer_size = result.get_value("SenseBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `fc_lun` -  (u64)
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `response_buffer` -  (u8[])
    /// * `response_buffer_size` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    /// * `sense_buffer_size` -  (u32)
    pub fn scsi_read_capacity(&self, cdb: &Vec<u8>, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, fc_lun: u64, hbastatus: &mut u32, response_buffer_size: &mut u32, sense_buffer_size: &mut u32, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "FcLun".to_string(), value: fc_lun.into() });

        let result = self.invoke_method("ScsiReadCapacity", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let response_buffer_size = result.get_value("ResponseBufferSize")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        let sense_buffer_size = result.get_value("SenseBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `response_buffer` -  (u8[])
    /// * `response_buffer_size` -  (u32)
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    /// * `sense_buffer_size` -  (u32)
    pub fn scsi_report_luns(&self, cdb: &Vec<u8>, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, hbastatus: &mut u32, response_buffer_size: &mut u32, sense_buffer_size: &mut u32, scsi_status: &mut u8, response_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });

        let result = self.invoke_method("ScsiReportLuns", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let response_buffer = result.get_value("ResponseBuffer")?;
        let response_buffer_size = result.get_value("ResponseBufferSize")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        let sense_buffer_size = result.get_value("SenseBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `event_count` -  (u32)
    /// * `events` -  (MSFC_EventBuffer[])
    /// * `hbastatus` -  (u32)
    pub fn get_event_buffer(&self, hbastatus: &mut u32, event_count: &mut u32, events: &mut Vec<MSFC_EventBuffer>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetEventBuffer", &[])?;
        let event_count = result.get_value("EventCount")?;
        let events = result.get_value("Events")?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_wwn` -  (u8[])
    /// * `port_wwn` -  (u8[])

    /// * `actual_rsp_buffer_size` -  (u32)
    /// * `hbastatus` -  (u32)
    /// * `rsp_buffer` -  (u8[])
    /// * `total_rsp_buffer_size` -  (u32)
    pub fn send_rls(&self, port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, hbastatus: &mut u32, total_rsp_buffer_size: &mut u32, actual_rsp_buffer_size: &mut u32, rsp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });

        let result = self.invoke_method("SendRLS", &args)?;
        let actual_rsp_buffer_size = result.get_value("ActualRspBufferSize")?;
        let hbastatus = result.get_value("HBAStatus")?;
        let rsp_buffer = result.get_value("RspBuffer")?;
        let total_rsp_buffer_size = result.get_value("TotalRspBufferSize")?;
        Ok(result.return_value)

    }

}

