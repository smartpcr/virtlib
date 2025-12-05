// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ContextRegistersARM64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextRegistersARM64 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "Cpsr")]
    pub cpsr: Option<u32>,

/// 
    #[serde(rename = "Fp")]
    pub fp: Option<u64>,

/// 
    #[serde(rename = "Lr")]
    pub lr: Option<u64>,

/// 
    #[serde(rename = "Pc")]
    pub pc: Option<u64>,

/// 
    #[serde(rename = "Sp")]
    pub sp: Option<u64>,

/// 
    #[serde(rename = "X0")]
    pub x0: Option<u64>,

/// 
    #[serde(rename = "X1")]
    pub x1: Option<u64>,

/// 
    #[serde(rename = "X10")]
    pub x10: Option<u64>,

/// 
    #[serde(rename = "X11")]
    pub x11: Option<u64>,

/// 
    #[serde(rename = "X12")]
    pub x12: Option<u64>,

/// 
    #[serde(rename = "X13")]
    pub x13: Option<u64>,

/// 
    #[serde(rename = "X14")]
    pub x14: Option<u64>,

/// 
    #[serde(rename = "X15")]
    pub x15: Option<u64>,

/// 
    #[serde(rename = "X16")]
    pub x16: Option<u64>,

/// 
    #[serde(rename = "X17")]
    pub x17: Option<u64>,

/// 
    #[serde(rename = "X18")]
    pub x18: Option<u64>,

/// 
    #[serde(rename = "X19")]
    pub x19: Option<u64>,

/// 
    #[serde(rename = "X2")]
    pub x2: Option<u64>,

/// 
    #[serde(rename = "X20")]
    pub x20: Option<u64>,

/// 
    #[serde(rename = "X21")]
    pub x21: Option<u64>,

/// 
    #[serde(rename = "X22")]
    pub x22: Option<u64>,

/// 
    #[serde(rename = "X23")]
    pub x23: Option<u64>,

/// 
    #[serde(rename = "X24")]
    pub x24: Option<u64>,

/// 
    #[serde(rename = "X25")]
    pub x25: Option<u64>,

/// 
    #[serde(rename = "X26")]
    pub x26: Option<u64>,

/// 
    #[serde(rename = "X27")]
    pub x27: Option<u64>,

/// 
    #[serde(rename = "X28")]
    pub x28: Option<u64>,

/// 
    #[serde(rename = "X3")]
    pub x3: Option<u64>,

/// 
    #[serde(rename = "X4")]
    pub x4: Option<u64>,

/// 
    #[serde(rename = "X5")]
    pub x5: Option<u64>,

/// 
    #[serde(rename = "X6")]
    pub x6: Option<u64>,

/// 
    #[serde(rename = "X7")]
    pub x7: Option<u64>,

/// 
    #[serde(rename = "X8")]
    pub x8: Option<u64>,

/// 
    #[serde(rename = "X9")]
    pub x9: Option<u64>,
}

impl ContextRegistersARM64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            cpsr: None,
            fp: None,
            lr: None,
            pc: None,
            sp: None,
            x0: None,
            x1: None,
            x10: None,
            x11: None,
            x12: None,
            x13: None,
            x14: None,
            x15: None,
            x16: None,
            x17: None,
            x18: None,
            x19: None,
            x2: None,
            x20: None,
            x21: None,
            x22: None,
            x23: None,
            x24: None,
            x25: None,
            x26: None,
            x27: None,
            x28: None,
            x3: None,
            x4: None,
            x5: None,
            x6: None,
            x7: None,
            x8: None,
            x9: None,
        }
    }


    /// Sets the value of Cpsr
    pub fn set_cpsr(&mut self, value: u32) {
        self.cpsr = Some(value);
    }

    /// Gets the value of Cpsr
    pub fn get_cpsr(&self) -> Option<&u32> {
        self.cpsr.as_ref()
    }

    /// Sets the value of Fp
    pub fn set_fp(&mut self, value: u64) {
        self.fp = Some(value);
    }

    /// Gets the value of Fp
    pub fn get_fp(&self) -> Option<&u64> {
        self.fp.as_ref()
    }

    /// Sets the value of Lr
    pub fn set_lr(&mut self, value: u64) {
        self.lr = Some(value);
    }

    /// Gets the value of Lr
    pub fn get_lr(&self) -> Option<&u64> {
        self.lr.as_ref()
    }

    /// Sets the value of Pc
    pub fn set_pc(&mut self, value: u64) {
        self.pc = Some(value);
    }

    /// Gets the value of Pc
    pub fn get_pc(&self) -> Option<&u64> {
        self.pc.as_ref()
    }

    /// Sets the value of Sp
    pub fn set_sp(&mut self, value: u64) {
        self.sp = Some(value);
    }

    /// Gets the value of Sp
    pub fn get_sp(&self) -> Option<&u64> {
        self.sp.as_ref()
    }

    /// Sets the value of X0
    pub fn set_x0(&mut self, value: u64) {
        self.x0 = Some(value);
    }

    /// Gets the value of X0
    pub fn get_x0(&self) -> Option<&u64> {
        self.x0.as_ref()
    }

    /// Sets the value of X1
    pub fn set_x1(&mut self, value: u64) {
        self.x1 = Some(value);
    }

    /// Gets the value of X1
    pub fn get_x1(&self) -> Option<&u64> {
        self.x1.as_ref()
    }

    /// Sets the value of X10
    pub fn set_x10(&mut self, value: u64) {
        self.x10 = Some(value);
    }

    /// Gets the value of X10
    pub fn get_x10(&self) -> Option<&u64> {
        self.x10.as_ref()
    }

    /// Sets the value of X11
    pub fn set_x11(&mut self, value: u64) {
        self.x11 = Some(value);
    }

    /// Gets the value of X11
    pub fn get_x11(&self) -> Option<&u64> {
        self.x11.as_ref()
    }

    /// Sets the value of X12
    pub fn set_x12(&mut self, value: u64) {
        self.x12 = Some(value);
    }

    /// Gets the value of X12
    pub fn get_x12(&self) -> Option<&u64> {
        self.x12.as_ref()
    }

    /// Sets the value of X13
    pub fn set_x13(&mut self, value: u64) {
        self.x13 = Some(value);
    }

    /// Gets the value of X13
    pub fn get_x13(&self) -> Option<&u64> {
        self.x13.as_ref()
    }

    /// Sets the value of X14
    pub fn set_x14(&mut self, value: u64) {
        self.x14 = Some(value);
    }

    /// Gets the value of X14
    pub fn get_x14(&self) -> Option<&u64> {
        self.x14.as_ref()
    }

    /// Sets the value of X15
    pub fn set_x15(&mut self, value: u64) {
        self.x15 = Some(value);
    }

    /// Gets the value of X15
    pub fn get_x15(&self) -> Option<&u64> {
        self.x15.as_ref()
    }

    /// Sets the value of X16
    pub fn set_x16(&mut self, value: u64) {
        self.x16 = Some(value);
    }

    /// Gets the value of X16
    pub fn get_x16(&self) -> Option<&u64> {
        self.x16.as_ref()
    }

    /// Sets the value of X17
    pub fn set_x17(&mut self, value: u64) {
        self.x17 = Some(value);
    }

    /// Gets the value of X17
    pub fn get_x17(&self) -> Option<&u64> {
        self.x17.as_ref()
    }

    /// Sets the value of X18
    pub fn set_x18(&mut self, value: u64) {
        self.x18 = Some(value);
    }

    /// Gets the value of X18
    pub fn get_x18(&self) -> Option<&u64> {
        self.x18.as_ref()
    }

    /// Sets the value of X19
    pub fn set_x19(&mut self, value: u64) {
        self.x19 = Some(value);
    }

    /// Gets the value of X19
    pub fn get_x19(&self) -> Option<&u64> {
        self.x19.as_ref()
    }

    /// Sets the value of X2
    pub fn set_x2(&mut self, value: u64) {
        self.x2 = Some(value);
    }

    /// Gets the value of X2
    pub fn get_x2(&self) -> Option<&u64> {
        self.x2.as_ref()
    }

    /// Sets the value of X20
    pub fn set_x20(&mut self, value: u64) {
        self.x20 = Some(value);
    }

    /// Gets the value of X20
    pub fn get_x20(&self) -> Option<&u64> {
        self.x20.as_ref()
    }

    /// Sets the value of X21
    pub fn set_x21(&mut self, value: u64) {
        self.x21 = Some(value);
    }

    /// Gets the value of X21
    pub fn get_x21(&self) -> Option<&u64> {
        self.x21.as_ref()
    }

    /// Sets the value of X22
    pub fn set_x22(&mut self, value: u64) {
        self.x22 = Some(value);
    }

    /// Gets the value of X22
    pub fn get_x22(&self) -> Option<&u64> {
        self.x22.as_ref()
    }

    /// Sets the value of X23
    pub fn set_x23(&mut self, value: u64) {
        self.x23 = Some(value);
    }

    /// Gets the value of X23
    pub fn get_x23(&self) -> Option<&u64> {
        self.x23.as_ref()
    }

    /// Sets the value of X24
    pub fn set_x24(&mut self, value: u64) {
        self.x24 = Some(value);
    }

    /// Gets the value of X24
    pub fn get_x24(&self) -> Option<&u64> {
        self.x24.as_ref()
    }

    /// Sets the value of X25
    pub fn set_x25(&mut self, value: u64) {
        self.x25 = Some(value);
    }

    /// Gets the value of X25
    pub fn get_x25(&self) -> Option<&u64> {
        self.x25.as_ref()
    }

    /// Sets the value of X26
    pub fn set_x26(&mut self, value: u64) {
        self.x26 = Some(value);
    }

    /// Gets the value of X26
    pub fn get_x26(&self) -> Option<&u64> {
        self.x26.as_ref()
    }

    /// Sets the value of X27
    pub fn set_x27(&mut self, value: u64) {
        self.x27 = Some(value);
    }

    /// Gets the value of X27
    pub fn get_x27(&self) -> Option<&u64> {
        self.x27.as_ref()
    }

    /// Sets the value of X28
    pub fn set_x28(&mut self, value: u64) {
        self.x28 = Some(value);
    }

    /// Gets the value of X28
    pub fn get_x28(&self) -> Option<&u64> {
        self.x28.as_ref()
    }

    /// Sets the value of X3
    pub fn set_x3(&mut self, value: u64) {
        self.x3 = Some(value);
    }

    /// Gets the value of X3
    pub fn get_x3(&self) -> Option<&u64> {
        self.x3.as_ref()
    }

    /// Sets the value of X4
    pub fn set_x4(&mut self, value: u64) {
        self.x4 = Some(value);
    }

    /// Gets the value of X4
    pub fn get_x4(&self) -> Option<&u64> {
        self.x4.as_ref()
    }

    /// Sets the value of X5
    pub fn set_x5(&mut self, value: u64) {
        self.x5 = Some(value);
    }

    /// Gets the value of X5
    pub fn get_x5(&self) -> Option<&u64> {
        self.x5.as_ref()
    }

    /// Sets the value of X6
    pub fn set_x6(&mut self, value: u64) {
        self.x6 = Some(value);
    }

    /// Gets the value of X6
    pub fn get_x6(&self) -> Option<&u64> {
        self.x6.as_ref()
    }

    /// Sets the value of X7
    pub fn set_x7(&mut self, value: u64) {
        self.x7 = Some(value);
    }

    /// Gets the value of X7
    pub fn get_x7(&self) -> Option<&u64> {
        self.x7.as_ref()
    }

    /// Sets the value of X8
    pub fn set_x8(&mut self, value: u64) {
        self.x8 = Some(value);
    }

    /// Gets the value of X8
    pub fn get_x8(&self) -> Option<&u64> {
        self.x8.as_ref()
    }

    /// Sets the value of X9
    pub fn set_x9(&mut self, value: u64) {
        self.x9 = Some(value);
    }

    /// Gets the value of X9
    pub fn get_x9(&self) -> Option<&u64> {
        self.x9.as_ref()
    }
}

