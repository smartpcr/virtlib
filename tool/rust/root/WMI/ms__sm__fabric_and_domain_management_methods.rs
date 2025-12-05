// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_FabricAndDomainManagementMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_FabricAndDomainManagementMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MS_SM_FabricAndDomainManagementMethods {
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

    /// * `dest_fcid` -  (u32)
    /// * `dest_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `req_buffer` -  (u8[])
    /// * `req_buffer_size` -  (u32)

    /// * `hbastatus` -  (u32)
    pub fn sm__send_test(&self, hba_port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, dest_fcid: u32, req_buffer_size: u32, req_buffer: &Vec<u8>, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "DestFCID".to_string(), value: dest_fcid.into() });
        args.push(MethodParameter { name: "ReqBufferSize".to_string(), value: req_buffer_size.into() });
        args.push(MethodParameter { name: "ReqBuffer".to_string(), value: req_buffer.into() });

        let result = self.invoke_method("SM_SendTEST", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_fcid` -  (u32)
    /// * `dest_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `req_buffer` -  (u8[])
    /// * `req_buffer_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    pub fn sm__send_echo(&self, hba_port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, dest_fcid: u32, in_resp_buffer_max_size: u32, req_buffer_size: u32, req_buffer: &Vec<u8>, hbastatus: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "DestFCID".to_string(), value: dest_fcid.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "ReqBufferSize".to_string(), value: req_buffer_size.into() });
        args.push(MethodParameter { name: "ReqBuffer".to_string(), value: req_buffer.into() });

        let result = self.invoke_method("SM_SendECHO", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_port_wwn` -  (u8[])
    /// * `domain_port_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `req_buffer` -  (u8[])
    /// * `req_buffer_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_smppass_thru(&self, hba_port_wwn: &Vec<u8>, dest_port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, in_resp_buffer_max_size: u32, req_buffer_size: u32, req_buffer: &Vec<u8>, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DestPortWWN".to_string(), value: dest_port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "ReqBufferSize".to_string(), value: req_buffer_size.into() });
        args.push(MethodParameter { name: "ReqBuffer".to_string(), value: req_buffer.into() });

        let result = self.invoke_method("SM_SendSMPPassThru", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `req_buffer` -  (u8[])
    /// * `req_buffer_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_ctpass_thru(&self, hba_port_wwn: &Vec<u8>, in_resp_buffer_max_size: u32, req_buffer_size: u32, req_buffer: &Vec<u8>, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });
        args.push(MethodParameter { name: "ReqBufferSize".to_string(), value: req_buffer_size.into() });
        args.push(MethodParameter { name: "ReqBuffer".to_string(), value: req_buffer.into() });

        let result = self.invoke_method("SM_SendCTPassThru", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `hbastatus` -  (u32)
    /// * `mgmt_info` -  (HBAFC3MgmtInfo)
    pub fn sm__get_rnidmgmt_info(&self, hbastatus: &mut u32, mgmt_info: &mut HBAFC3MgmtInfo) -> Result<(), WmiError> {

        let result = self.invoke_method("SM_GetRNIDMgmtInfo", &[])?;
        let hbastatus = result.get_value("HBAStatus")?;
        let mgmt_info = result.get_value("MgmtInfo")?;
        Ok(result.return_value)

    }


/// 

    /// * `mgmt_info` -  (HBAFC3MgmtInfo)

    /// * `hbastatus` -  (u32)
    pub fn sm__set_rnidmgmt_info(&self, mgmt_info: HBAFC3MgmtInfo, hbastatus: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MgmtInfo".to_string(), value: mgmt_info.into() });

        let result = self.invoke_method("SM_SetRNIDMgmtInfo", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_fcid` -  (u32)
    /// * `dest_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `node_id_data_format` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_rnid(&self, hba_port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, dest_fcid: u32, node_id_data_format: u32, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "DestFCID".to_string(), value: dest_fcid.into() });
        args.push(MethodParameter { name: "NodeIdDataFormat".to_string(), value: node_id_data_format.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendRNID", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `agent_domain` -  (u32)
    /// * `agent_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_rpl(&self, hba_port_wwn: &Vec<u8>, agent_wwn: &Vec<u8>, agent_domain: u32, port_index: u32, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "AgentWWN".to_string(), value: agent_wwn.into() });
        args.push(MethodParameter { name: "AgentDomain".to_string(), value: agent_domain.into() });
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendRPL", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `agent_domain` -  (u32)
    /// * `agent_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `object_port_number` -  (u32)
    /// * `object_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_rps(&self, hba_port_wwn: &Vec<u8>, agent_wwn: &Vec<u8>, object_wwn: &Vec<u8>, agent_domain: u32, object_port_number: u32, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "AgentWWN".to_string(), value: agent_wwn.into() });
        args.push(MethodParameter { name: "ObjectWWN".to_string(), value: object_wwn.into() });
        args.push(MethodParameter { name: "AgentDomain".to_string(), value: agent_domain.into() });
        args.push(MethodParameter { name: "ObjectPortNumber".to_string(), value: object_port_number.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendRPS", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain` -  (u32)
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_srl(&self, hba_port_wwn: &Vec<u8>, wwn: &Vec<u8>, domain: u32, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "WWN".to_string(), value: wwn.into() });
        args.push(MethodParameter { name: "Domain".to_string(), value: domain.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendSRL", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_wwn` -  (u8[])
    /// * `function` -  (u8)
    /// * `in_resp_buffer_max_size` -  (u32)
    /// * `source_wwn` -  (u8[])
    /// * `type` -  (u8)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_lirr(&self, source_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, function: u8, type: u8, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceWWN".to_string(), value: source_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "Function".to_string(), value: function.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendLIRR", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `dest_wwn` -  (u8[])
    /// * `hba_port_wwn` -  (u8[])
    /// * `in_resp_buffer_max_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_resp_buffer_size` -  (u32)
    /// * `resp_buffer` -  (u8[])
    /// * `total_resp_buffer_size` -  (u32)
    pub fn sm__send_rls(&self, hba_port_wwn: &Vec<u8>, dest_wwn: &Vec<u8>, in_resp_buffer_max_size: u32, hbastatus: &mut u32, total_resp_buffer_size: &mut u32, out_resp_buffer_size: &mut u32, resp_buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HbaPortWWN".to_string(), value: hba_port_wwn.into() });
        args.push(MethodParameter { name: "DestWWN".to_string(), value: dest_wwn.into() });
        args.push(MethodParameter { name: "InRespBufferMaxSize".to_string(), value: in_resp_buffer_max_size.into() });

        let result = self.invoke_method("SM_SendRLS", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_resp_buffer_size = result.get_value("OutRespBufferSize")?;
        let resp_buffer = result.get_value("RespBuffer")?;
        let total_resp_buffer_size = result.get_value("TotalRespBufferSize")?;
        Ok(result.return_value)

    }

}

