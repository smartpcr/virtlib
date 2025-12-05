// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPacketDirectSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPacketDirectSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<MSFT_NetAdapter_PacketDirectCapabilities>,

/// 
    #[serde(rename = "DiagnosticCode")]
    pub diagnostic_code: Option<u32>,

/// 
    #[serde(rename = "DmaAddressWidth")]
    pub dma_address_width: Option<u8>,

/// 
    #[serde(rename = "DomainId")]
    pub domain_id: Option<u32>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "Operational")]
    pub operational: Option<bool>,
}

impl MSFT_NetAdapterPacketDirectSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            capabilities: None,
            diagnostic_code: None,
            dma_address_width: None,
            domain_id: None,
            enabled: None,
            operational: None,
        }
    }


    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: MSFT_NetAdapter_PacketDirectCapabilities) {
        self.capabilities = Some(value);
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> Option<&MSFT_NetAdapter_PacketDirectCapabilities> {
        self.capabilities.as_ref()
    }

    /// Sets the value of DiagnosticCode
    pub fn set_diagnostic_code(&mut self, value: u32) {
        self.diagnostic_code = Some(value);
    }

    /// Gets the value of DiagnosticCode
    pub fn get_diagnostic_code(&self) -> Option<&u32> {
        self.diagnostic_code.as_ref()
    }

    /// Sets the value of DmaAddressWidth
    pub fn set_dma_address_width(&mut self, value: u8) {
        self.dma_address_width = Some(value);
    }

    /// Gets the value of DmaAddressWidth
    pub fn get_dma_address_width(&self) -> Option<&u8> {
        self.dma_address_width.as_ref()
    }

    /// Sets the value of DomainId
    pub fn set_domain_id(&mut self, value: u32) {
        self.domain_id = Some(value);
    }

    /// Gets the value of DomainId
    pub fn get_domain_id(&self) -> Option<&u32> {
        self.domain_id.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of Operational
    pub fn set_operational(&mut self, value: bool) {
        self.operational = Some(value);
    }

    /// Gets the value of Operational
    pub fn get_operational(&self) -> Option<&bool> {
        self.operational.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterPacketDirectSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterPacketDirectSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterPacketDirectSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterPacketDirectSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

