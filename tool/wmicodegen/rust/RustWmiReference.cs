// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System.Globalization;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiReference : WmiReference
    {
        public RustWmiReference(string reference) : base(reference)
        {
        }

        public override string GetSourceCode()
        {
            // Convert to Rust use statement
            // e.g., "crate::wmi" becomes "use crate::wmi;"
            return string.Format(CultureInfo.InvariantCulture, "use {0};", Reference);
        }
    }
}
