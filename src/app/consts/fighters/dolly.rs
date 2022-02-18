#[repr(u32)]
pub enum FighterDollyFinalModuleCall {
    StartInit = 0x0,
    StartExit = 0x1,
    ReadyInit = 0x2,
    ReadyExec = 0x3,
    ReadyExitPre = 0x4,
    ReadyExit = 0x5,
    Scene01Init = 0x6,
    Scene01Exit = 0x7,
    Scene02Init = 0x8,
    Scene02Exit = 0x9,
    Scene02ExecFixPos = 0xA,
    Scene03Init = 0xB,
    Scene03ExecFixPos = 0xC,
    Scene03Exit = 0xD,
    Scene04Init = 0xE,
    Scene04Exit = 0xF,
    Scene05Init = 0x10,
    Scene05ExecFixPos = 0x11,
    Scene05Exit = 0x12,
    EndInit = 0x13,
    EndExec = 0x14,
    EndExit = 0x15,
    Num = 0x16,
}