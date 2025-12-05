// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VolumeToLog_Flags
//////////////////////////////////////////////

/// VolumeToLog_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VolumeToLog_Flags {
    /// a
    #[serde(rename = "a")]
    A = 1,
    /// b
    #[serde(rename = "b")]
    B = 2,
    /// c
    #[serde(rename = "c")]
    C = 3,
    /// d
    #[serde(rename = "d")]
    D = 4,
    /// e
    #[serde(rename = "e")]
    E = 5,
    /// f
    #[serde(rename = "f")]
    F = 6,
    /// g
    #[serde(rename = "g")]
    G = 7,
    /// h
    #[serde(rename = "h")]
    H = 8,
    /// i
    #[serde(rename = "i")]
    I = 9,
    /// j
    #[serde(rename = "j")]
    J = 10,
    /// k
    #[serde(rename = "k")]
    K = 11,
    /// l
    #[serde(rename = "l")]
    L = 12,
    /// m
    #[serde(rename = "m")]
    M = 13,
    /// n
    #[serde(rename = "n")]
    N = 14,
    /// o
    #[serde(rename = "o")]
    O = 15,
    /// p
    #[serde(rename = "p")]
    P = 16,
    /// q
    #[serde(rename = "q")]
    Q = 17,
    /// r
    #[serde(rename = "r")]
    R = 18,
    /// s
    #[serde(rename = "s")]
    S = 19,
    /// t
    #[serde(rename = "t")]
    T = 20,
    /// u
    #[serde(rename = "u")]
    U = 21,
    /// v
    #[serde(rename = "v")]
    V = 22,
    /// w
    #[serde(rename = "w")]
    W = 23,
    /// x
    #[serde(rename = "x")]
    X = 24,
    /// y
    #[serde(rename = "y")]
    Y = 25,
    /// z
    #[serde(rename = "z")]
    Z = 26,
    /// all
    #[serde(rename = "all")]
    All = 27,
    /// local
    #[serde(rename = "local")]
    Local = 28,
    /// network
    #[serde(rename = "network")]
    Network = 29,
}

impl Default for VolumeToLog_Flags {
    fn default() -> Self {
        Self::A
    }
}

