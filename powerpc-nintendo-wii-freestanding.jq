#!/usr/bin/env bash
# (lovely hack, isn't this) \
exec jq -n --rawfile linker_script "${0%.jq}-linker-script.ld" -f "$0" > "${0%.jq}.json"
{
    "arch": "powerpc",
    "cpu": "750",
    "data-layout": "E-m:e-p:32:32-i64:64-n32",
    "dynamic-linking": false,
    "exe-suffix": ".elf",
    "executables": true,
    "has-rpath": true,
    "llvm-target": "powerpc-unknown-none-eabihf",
    "linker": "rust-lld",
    "link-script": $linker_script,
    "linker-flavor": "ld.lld",
    "os": "revolution",
    "panic-strategy": "abort",
    "relocation-model": "static",
    "target-endian": "big",
    "target-family": "unix",
    "target-mcount": "_mcount",
    "target-c-int-width": "32",
    "target-pointer-width": "32",
    "vendor": "nintendo"
}
