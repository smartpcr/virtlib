// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ContextRegistersAMD64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextRegistersAMD64 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "R10")]
    pub r10: Option<u64>,

/// 
    #[serde(rename = "R11")]
    pub r11: Option<u64>,

/// 
    #[serde(rename = "R12")]
    pub r12: Option<u64>,

/// 
    #[serde(rename = "R13")]
    pub r13: Option<u64>,

/// 
    #[serde(rename = "R14")]
    pub r14: Option<u64>,

/// 
    #[serde(rename = "R15")]
    pub r15: Option<u64>,

/// 
    #[serde(rename = "R8")]
    pub r8: Option<u64>,

/// 
    #[serde(rename = "R9")]
    pub r9: Option<u64>,

/// 
    #[serde(rename = "Rax")]
    pub rax: Option<u64>,

/// 
    #[serde(rename = "Rbp")]
    pub rbp: Option<u64>,

/// 
    #[serde(rename = "Rbx")]
    pub rbx: Option<u64>,

/// 
    #[serde(rename = "Rcx")]
    pub rcx: Option<u64>,

/// 
    #[serde(rename = "Rdi")]
    pub rdi: Option<u64>,

/// 
    #[serde(rename = "Rdx")]
    pub rdx: Option<u64>,

/// 
    #[serde(rename = "Rip")]
    pub rip: Option<u64>,

/// 
    #[serde(rename = "Rsi")]
    pub rsi: Option<u64>,

/// 
    #[serde(rename = "Rsp")]
    pub rsp: Option<u64>,
}

impl ContextRegistersAMD64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            r10: None,
            r11: None,
            r12: None,
            r13: None,
            r14: None,
            r15: None,
            r8: None,
            r9: None,
            rax: None,
            rbp: None,
            rbx: None,
            rcx: None,
            rdi: None,
            rdx: None,
            rip: None,
            rsi: None,
            rsp: None,
        }
    }


    /// Sets the value of R10
    pub fn set_r10(&mut self, value: u64) {
        self.r10 = Some(value);
    }

    /// Gets the value of R10
    pub fn get_r10(&self) -> Option<&u64> {
        self.r10.as_ref()
    }

    /// Sets the value of R11
    pub fn set_r11(&mut self, value: u64) {
        self.r11 = Some(value);
    }

    /// Gets the value of R11
    pub fn get_r11(&self) -> Option<&u64> {
        self.r11.as_ref()
    }

    /// Sets the value of R12
    pub fn set_r12(&mut self, value: u64) {
        self.r12 = Some(value);
    }

    /// Gets the value of R12
    pub fn get_r12(&self) -> Option<&u64> {
        self.r12.as_ref()
    }

    /// Sets the value of R13
    pub fn set_r13(&mut self, value: u64) {
        self.r13 = Some(value);
    }

    /// Gets the value of R13
    pub fn get_r13(&self) -> Option<&u64> {
        self.r13.as_ref()
    }

    /// Sets the value of R14
    pub fn set_r14(&mut self, value: u64) {
        self.r14 = Some(value);
    }

    /// Gets the value of R14
    pub fn get_r14(&self) -> Option<&u64> {
        self.r14.as_ref()
    }

    /// Sets the value of R15
    pub fn set_r15(&mut self, value: u64) {
        self.r15 = Some(value);
    }

    /// Gets the value of R15
    pub fn get_r15(&self) -> Option<&u64> {
        self.r15.as_ref()
    }

    /// Sets the value of R8
    pub fn set_r8(&mut self, value: u64) {
        self.r8 = Some(value);
    }

    /// Gets the value of R8
    pub fn get_r8(&self) -> Option<&u64> {
        self.r8.as_ref()
    }

    /// Sets the value of R9
    pub fn set_r9(&mut self, value: u64) {
        self.r9 = Some(value);
    }

    /// Gets the value of R9
    pub fn get_r9(&self) -> Option<&u64> {
        self.r9.as_ref()
    }

    /// Sets the value of Rax
    pub fn set_rax(&mut self, value: u64) {
        self.rax = Some(value);
    }

    /// Gets the value of Rax
    pub fn get_rax(&self) -> Option<&u64> {
        self.rax.as_ref()
    }

    /// Sets the value of Rbp
    pub fn set_rbp(&mut self, value: u64) {
        self.rbp = Some(value);
    }

    /// Gets the value of Rbp
    pub fn get_rbp(&self) -> Option<&u64> {
        self.rbp.as_ref()
    }

    /// Sets the value of Rbx
    pub fn set_rbx(&mut self, value: u64) {
        self.rbx = Some(value);
    }

    /// Gets the value of Rbx
    pub fn get_rbx(&self) -> Option<&u64> {
        self.rbx.as_ref()
    }

    /// Sets the value of Rcx
    pub fn set_rcx(&mut self, value: u64) {
        self.rcx = Some(value);
    }

    /// Gets the value of Rcx
    pub fn get_rcx(&self) -> Option<&u64> {
        self.rcx.as_ref()
    }

    /// Sets the value of Rdi
    pub fn set_rdi(&mut self, value: u64) {
        self.rdi = Some(value);
    }

    /// Gets the value of Rdi
    pub fn get_rdi(&self) -> Option<&u64> {
        self.rdi.as_ref()
    }

    /// Sets the value of Rdx
    pub fn set_rdx(&mut self, value: u64) {
        self.rdx = Some(value);
    }

    /// Gets the value of Rdx
    pub fn get_rdx(&self) -> Option<&u64> {
        self.rdx.as_ref()
    }

    /// Sets the value of Rip
    pub fn set_rip(&mut self, value: u64) {
        self.rip = Some(value);
    }

    /// Gets the value of Rip
    pub fn get_rip(&self) -> Option<&u64> {
        self.rip.as_ref()
    }

    /// Sets the value of Rsi
    pub fn set_rsi(&mut self, value: u64) {
        self.rsi = Some(value);
    }

    /// Gets the value of Rsi
    pub fn get_rsi(&self) -> Option<&u64> {
        self.rsi.as_ref()
    }

    /// Sets the value of Rsp
    pub fn set_rsp(&mut self, value: u64) {
        self.rsp = Some(value);
    }

    /// Gets the value of Rsp
    pub fn get_rsp(&self) -> Option<&u64> {
        self.rsp.as_ref()
    }
}

