// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiConstructor : WmiConstructor
    {
        public RustWmiConstructor(RustWmiClass wClass, string derivation) : base(wClass, derivation)
        {
        }

        public override string GetSourceCode()
        {
            // Rust uses associated functions (new) instead of constructors
            // This is handled in RustWmiClass.GetSourceCode()
            StringBuilder sb = new StringBuilder();

            sb.AppendLine("    /// Creates a new instance");
            foreach (var item in HeaderComments)
            {
                sb.AppendLine($"    /// {item}");
            }

            sb.AppendFormat(CultureInfo.InvariantCulture, "    pub fn new(");

            if (Params.Count > 0)
            {
                for (int i = 0; i < Params.Count; i++)
                {
                    if (i > 0) sb.Append(", ");
                    sb.Append(Params[i].GetSourceCode());
                }
            }

            sb.AppendLine(") -> Self {");
            sb.AppendLine("        Self {");
            sb.AppendLine(BodyText);
            sb.AppendLine("        }");
            sb.AppendLine("    }");

            return sb.ToString();
        }

        protected override string GetMethodBodyText(MethodData mData)
        {
            throw new NotImplementedException();
        }

        protected override WmiMethodParam GetWmiMethodParam(PropertyData pData, ParamType pType, WmiMethod wMethod, bool optional)
        {
            throw new NotImplementedException();
        }

        protected override WmiMethodParam GetWmiMethodParam(string comment, ParamType pType, string Type, string Name, bool custom)
        {
            throw new NotImplementedException();
        }
    }
}
