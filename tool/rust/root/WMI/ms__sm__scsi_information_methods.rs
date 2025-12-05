// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_ScsiInformationMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_ScsiInformationMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MS_SM_ScsiInformationMethods {
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

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `in_sense_buffer_max_size` -  (u32)
    /// * `smhba_lun` -  (u64)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `out_sense_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    pub fn sm__scsi_inquiry(&self, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, smhba_lun: u64, cdb: &Vec<u8>, in_resp_buffer_max_size: u32, in_sense_buffer_max_size: u32, hbastatus: &mut u32, scsi_status: &mut u8, out_resp_buffer_size: &mut u32, out_sense_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "SmhbaLUN".to_string(), value: smhba_lun.into() });
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "InSenseBufferMaxSize".to_string(), value: in_sense_buffer_max_size.into() });

        let result = self.invoke_method("SM_ScsiInquiry", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let out_sense_buffer_size = result.get_value("OutSenseBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        Ok(result.return_value)

    }


/// 

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `in_sense_buffer_max_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `out_sense_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__scsi_report_luns(&self, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, cdb: &Vec<u8>, in_resp_buffer_max_size: u32, in_sense_buffer_max_size: u32, hbastatus: &mut u32, scsi_status: &mut u8, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, out_sense_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "InSenseBufferMaxSize".to_string(), value: in_sense_buffer_max_size.into() });

        let result = self.invoke_method("SM_ScsiReportLuns", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let out_sense_buffer_size = result.get_value("OutSenseBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `cdb` -  (u8[])
    /// * `discovered_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `in_sense_buffer_max_size` -  (u32)
    /// * `smhba_lun` -  (u64)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `out_sense_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `scsi_status` -  (u8)
    /// * `sense_buffer` -  (u8[])
    pub fn sm__scsi_read_capacity(&self, hba_port_wwn: &Vec<u8>, discovered_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, smhba_lun: u64, cdb: &Vec<u8>, in_resp_buffer_max_size: u32, in_sense_buffer_max_size: u32, hbastatus: &mut u32, scsi_status: &mut u8, out_resp_buffer_size: &mut u32, out_sense_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>, sense_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DiscoveredPortWWN".to_string(), value: discovered_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "SmhbaLUN".to_string(), value: smhba_lun.into() });
        args.push(MethodParameter { name: "Cdb".to_string(), value: cdb.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "InSenseBufferMaxSize".to_string(), value: in_sense_buffer_max_size.into() });

        let result = self.invoke_method("SM_ScsiReadCapacity", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let out_sense_buffer_size = result.get_value("OutSenseBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let scsi_status = result.get_value("ScsiStatus")?;
        let sense_buffer = result.get_value("SenseBuffer")?;
        Ok(result.return_value)

    }

}

