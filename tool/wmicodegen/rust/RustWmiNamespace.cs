// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.IO;
using System.Linq;
using System.Management;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiNamespace : WmiNamespace
    {
        public RustWmiNamespace(string name) : base(name)
        {
        }

        protected override WmiReference GetReference(string reference)
        {
            return new RustWmiReference(reference);
        }

        protected override WmiSource GetWmiSource(ManagementClass wmiClass, WmiModule wModule)
        {
            return new RustWmiSource(wmiClass, wModule);
        }

        public override string GetSourceCode()
        {
            throw new NotImplementedException();
        }

        public override string GetModuleName(string source)
        {
            return Name.Split(new char[] { '/' }).Last();
        }

        protected override string GetCommonModuleName()
        {
            return GetModuleName("common");
        }

        public override void GenerateSources(string outputDir)
        {
            string path = Path.Combine(Environment.CurrentDirectory, outputDir, Name);
            if (!Directory.Exists(path)) { Directory.CreateDirectory(path); }

            foreach (var item in Modules)
            {
                if (!Directory.Exists(path)) Directory.CreateDirectory(path);
                item.Value.GenerateSources(path);
            }

            // Generate mod.rs file to expose all modules
            GenerateModFile(path);
        }

        private void GenerateModFile(string path)
        {
            string modFilePath = Path.Combine(path, "mod.rs");
            using (var sw = new StreamWriter(modFilePath))
            {
                sw.WriteLine("// Copyright 2019 (c) Microsoft Corporation.");
                sw.WriteLine("// Licensed under the MIT license.");
                sw.WriteLine();
                sw.WriteLine("// Auto-generated module declarations");
                sw.WriteLine();

                foreach (var module in Modules)
                {
                    foreach (var source in module.Value.Sources)
                    {
                        string moduleName = ToSnakeCase(source.Key);
                        sw.WriteLine($"pub mod {moduleName};");
                    }
                }
            }
            Logger.Info("Module file {0}", modFilePath);
        }

        private static string ToSnakeCase(string name)
        {
            if (string.IsNullOrEmpty(name))
                return name;

            var result = new System.Text.StringBuilder();
            for (int i = 0; i < name.Length; i++)
            {
                char c = name[i];
                if (char.IsUpper(c))
                {
                    if (i > 0 && !char.IsUpper(name[i - 1]))
                        result.Append('_');
                    result.Append(char.ToLowerInvariant(c));
                }
                else
                {
                    result.Append(c);
                }
            }
            return result.ToString().ToLowerInvariant();
        }
    }
}
