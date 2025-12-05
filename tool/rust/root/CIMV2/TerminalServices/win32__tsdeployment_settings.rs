// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSDeploymentSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSDeploymentSettings {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// Allow Font Smoothing
    #[serde(rename = "AllowFontSmoothing")]
    pub allow_font_smoothing: Option<bool>,

/// Certificate Expires On, stored as a 64bit FILETIME format
    #[serde(rename = "CertificateExpiresOn")]
    pub certificate_expires_on: Option<String>,

/// Certificate used to sign RDP files
    #[serde(rename = "CertificateHash")]
    pub certificate_hash: Vec<u8>,

/// Certificate Issued By
    #[serde(rename = "CertificateIssuedBy")]
    pub certificate_issued_by: Option<String>,

/// Certificate Issued To
    #[serde(rename = "CertificateIssuedTo")]
    pub certificate_issued_to: Option<String>,

/// Color Bit Depth
    #[serde(rename = "ColorBitDepth")]
    pub color_bit_depth: Option<TSDeploymentSettings_ColorBitDepth>,

/// Contents of the RDP file corresponding to the Custom RDP Settings
    #[serde(rename = "CustomRDPSettings")]
    pub custom_rdpsettings: Option<String>,

/// Contents of the RDP file corresponding to the Deployment Settings, if this is set the corresponding Redirection settings and other Deployment settings are ignored and this RDP file is used.
    #[serde(rename = "DeploymentRDPSettings")]
    pub deployment_rdpsettings: Option<String>,

/// Farm Name
    #[serde(rename = "FarmName")]
    pub farm_name: Option<String>,

/// Gateway Authentication Mode
    #[serde(rename = "GatewayAuthMode")]
    pub gateway_auth_mode: Option<TSDeploymentSettings_GatewayAuthMode>,

/// Gateway Name
    #[serde(rename = "GatewayName")]
    pub gateway_name: Option<String>,

/// How Gateway is Used
    #[serde(rename = "GatewayUsage")]
    pub gateway_usage: Option<TSDeploymentSettings_GatewayUsage>,

/// Use the same user credentials for TS Gateway and TS Server when possible
    #[serde(rename = "GatewayUseCachedCreds")]
    pub gateway_use_cached_creds: Option<bool>,

/// Use a Certificate to Sign the RDP Files
    #[serde(rename = "HasCertificate")]
    pub has_certificate: Option<bool>,

/// RDP Port
    #[serde(rename = "Port")]
    pub port: Option<i32>,

/// Redirection Options is configured by adding the following flags  None(0), Drives(1), Printers(2), Clipboard(4), Plug and Play(8), Smart Card(16)
    #[serde(rename = "RedirectionOptions")]
    pub redirection_options: Option<i32>,

/// Require Server Authentication (DEPRECATED)
    #[serde(rename = "RequireServerAuth")]
    pub require_server_auth: Option<bool>,

/// Enable Multi-Monitor for desktop (not RAIL)
    #[serde(rename = "UseMultimon")]
    pub use_multimon: Option<bool>,
}

impl Win32_TSDeploymentSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            allow_font_smoothing: None,
            certificate_expires_on: None,
            certificate_hash: Vec::new(),
            certificate_issued_by: None,
            certificate_issued_to: None,
            color_bit_depth: None,
            custom_rdpsettings: None,
            deployment_rdpsettings: None,
            farm_name: None,
            gateway_auth_mode: None,
            gateway_name: None,
            gateway_usage: None,
            gateway_use_cached_creds: None,
            has_certificate: None,
            port: None,
            redirection_options: None,
            require_server_auth: None,
            use_multimon: None,
        }
    }


    /// Sets the value of AllowFontSmoothing
    pub fn set_allow_font_smoothing(&mut self, value: bool) {
        self.allow_font_smoothing = Some(value);
    }

    /// Gets the value of AllowFontSmoothing
    pub fn get_allow_font_smoothing(&self) -> Option<&bool> {
        self.allow_font_smoothing.as_ref()
    }

    /// Sets the value of CertificateExpiresOn
    pub fn set_certificate_expires_on(&mut self, value: String) {
        self.certificate_expires_on = Some(value);
    }

    /// Gets the value of CertificateExpiresOn
    pub fn get_certificate_expires_on(&self) -> Option<&String> {
        self.certificate_expires_on.as_ref()
    }

    /// Sets the value of CertificateHash
    pub fn set_certificate_hash(&mut self, value: Vec<u8>) {
        self.certificate_hash = value;
    }

    /// Gets the value of CertificateHash
    pub fn get_certificate_hash(&self) -> &Vec<u8> {
        &self.certificate_hash
    }

    /// Sets the value of CertificateIssuedBy
    pub fn set_certificate_issued_by(&mut self, value: String) {
        self.certificate_issued_by = Some(value);
    }

    /// Gets the value of CertificateIssuedBy
    pub fn get_certificate_issued_by(&self) -> Option<&String> {
        self.certificate_issued_by.as_ref()
    }

    /// Sets the value of CertificateIssuedTo
    pub fn set_certificate_issued_to(&mut self, value: String) {
        self.certificate_issued_to = Some(value);
    }

    /// Gets the value of CertificateIssuedTo
    pub fn get_certificate_issued_to(&self) -> Option<&String> {
        self.certificate_issued_to.as_ref()
    }

    /// Sets the value of ColorBitDepth
    pub fn set_color_bit_depth(&mut self, value: TSDeploymentSettings_ColorBitDepth) {
        self.color_bit_depth = Some(value);
    }

    /// Gets the value of ColorBitDepth
    pub fn get_color_bit_depth(&self) -> Option<&TSDeploymentSettings_ColorBitDepth> {
        self.color_bit_depth.as_ref()
    }

    /// Sets the value of CustomRDPSettings
    pub fn set_custom_rdpsettings(&mut self, value: String) {
        self.custom_rdpsettings = Some(value);
    }

    /// Gets the value of CustomRDPSettings
    pub fn get_custom_rdpsettings(&self) -> Option<&String> {
        self.custom_rdpsettings.as_ref()
    }

    /// Sets the value of DeploymentRDPSettings
    pub fn set_deployment_rdpsettings(&mut self, value: String) {
        self.deployment_rdpsettings = Some(value);
    }

    /// Gets the value of DeploymentRDPSettings
    pub fn get_deployment_rdpsettings(&self) -> Option<&String> {
        self.deployment_rdpsettings.as_ref()
    }

    /// Sets the value of FarmName
    pub fn set_farm_name(&mut self, value: String) {
        self.farm_name = Some(value);
    }

    /// Gets the value of FarmName
    pub fn get_farm_name(&self) -> Option<&String> {
        self.farm_name.as_ref()
    }

    /// Sets the value of GatewayAuthMode
    pub fn set_gateway_auth_mode(&mut self, value: TSDeploymentSettings_GatewayAuthMode) {
        self.gateway_auth_mode = Some(value);
    }

    /// Gets the value of GatewayAuthMode
    pub fn get_gateway_auth_mode(&self) -> Option<&TSDeploymentSettings_GatewayAuthMode> {
        self.gateway_auth_mode.as_ref()
    }

    /// Sets the value of GatewayName
    pub fn set_gateway_name(&mut self, value: String) {
        self.gateway_name = Some(value);
    }

    /// Gets the value of GatewayName
    pub fn get_gateway_name(&self) -> Option<&String> {
        self.gateway_name.as_ref()
    }

    /// Sets the value of GatewayUsage
    pub fn set_gateway_usage(&mut self, value: TSDeploymentSettings_GatewayUsage) {
        self.gateway_usage = Some(value);
    }

    /// Gets the value of GatewayUsage
    pub fn get_gateway_usage(&self) -> Option<&TSDeploymentSettings_GatewayUsage> {
        self.gateway_usage.as_ref()
    }

    /// Sets the value of GatewayUseCachedCreds
    pub fn set_gateway_use_cached_creds(&mut self, value: bool) {
        self.gateway_use_cached_creds = Some(value);
    }

    /// Gets the value of GatewayUseCachedCreds
    pub fn get_gateway_use_cached_creds(&self) -> Option<&bool> {
        self.gateway_use_cached_creds.as_ref()
    }

    /// Sets the value of HasCertificate
    pub fn set_has_certificate(&mut self, value: bool) {
        self.has_certificate = Some(value);
    }

    /// Gets the value of HasCertificate
    pub fn get_has_certificate(&self) -> Option<&bool> {
        self.has_certificate.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: i32) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&i32> {
        self.port.as_ref()
    }

    /// Sets the value of RedirectionOptions
    pub fn set_redirection_options(&mut self, value: i32) {
        self.redirection_options = Some(value);
    }

    /// Gets the value of RedirectionOptions
    pub fn get_redirection_options(&self) -> Option<&i32> {
        self.redirection_options.as_ref()
    }

    /// Sets the value of RequireServerAuth
    pub fn set_require_server_auth(&mut self, value: bool) {
        self.require_server_auth = Some(value);
    }

    /// Gets the value of RequireServerAuth
    pub fn get_require_server_auth(&self) -> Option<&bool> {
        self.require_server_auth.as_ref()
    }

    /// Sets the value of UseMultimon
    pub fn set_use_multimon(&mut self, value: bool) {
        self.use_multimon = Some(value);
    }

    /// Gets the value of UseMultimon
    pub fn get_use_multimon(&self) -> Option<&bool> {
        self.use_multimon.as_ref()
    }
}

