// Targets the Little-endian Cortex-R4F/R5F processor (ARMv7-R)

use crate::spec::{
    Cc, FloatAbi, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "armv7r-none-eabihf".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("Armv7-R, hardfloat".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            abi: "eabihf".into(),
            llvm_floatabi: Some(FloatAbi::Hard),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            relocation_model: RelocModel::Static,
            panic_strategy: PanicStrategy::Abort,
            features: "+vfp3d16".into(),
            max_atomic_width: Some(64),
            emit_debug_gdb_scripts: false,
            // GCC defaults to 8 for arm-none here.
            c_enum_min_bits: Some(8),
            ..Default::default()
        },
    }
}
