// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustFormat : IFormat
    {
        #region Constructor
        public RustFormat(string[] wmiNamespaces, string outDir, bool recurse) : base(wmiNamespaces, outDir, recurse)
        {
        }
        #endregion

        protected override WmiNamespace GetWmiNamespace(string wmins)
        {
            return new RustWmiNamespace(wmins);
        }
    }
}
