pub trait ANSICode: Send + Sync {
    fn value(&self) -> String;
    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

#[derive(Clone)]
pub struct CombinedANSI(String);

impl ANSICode for CombinedANSI {
    fn value(&self) -> String {
        self.0.clone()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct EscapeANSI;
impl ANSICode for EscapeANSI {
    fn value(&self) -> String {
        "\x1B".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CtrlSeqANSI;
impl ANSICode for CtrlSeqANSI {
    fn value(&self) -> String {
        "\x5B".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct DeviceCtrlANSI;
impl ANSICode for DeviceCtrlANSI {
    fn value(&self) -> String {
        "\x50".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct TerminatorANSI;
impl ANSICode for TerminatorANSI {
    fn value(&self) -> String {
        "\x5C".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct OSCommandANSI;
impl ANSICode for OSCommandANSI {
    fn value(&self) -> String {
        "\x5D".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct SingleShift2ANSI;
impl ANSICode for SingleShift2ANSI {
    fn value(&self) -> String {
        "\x4E".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct SingleShift3ANSI;
impl ANSICode for SingleShift3ANSI {
    fn value(&self) -> String {
        "\x4F".to_string()
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct ResetANSI;
impl ANSICode for ResetANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "0m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlackANSI;
impl ANSICode for BlackANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "30m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct RedANSI;
impl ANSICode for RedANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "31m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct GreenANSI;
impl ANSICode for GreenANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "32m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct YellowANSI;
impl ANSICode for YellowANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "33m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlueANSI;
impl ANSICode for BlueANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "34m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct MagentaANSI;
impl ANSICode for MagentaANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "35m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CyanANSI;
impl ANSICode for CyanANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "36m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct WhiteANSI;
impl ANSICode for WhiteANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "37m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlackBackgroundANSI;
impl ANSICode for BlackBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "40m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct RedBackgroundANSI;
impl ANSICode for RedBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "41m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct GreenBackgroundANSI;
impl ANSICode for GreenBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "42m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct YellowBackgroundANSI;
impl ANSICode for YellowBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "43m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlueBackgroundANSI;
impl ANSICode for BlueBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "44m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct MagentaBackgroundANSI;
impl ANSICode for MagentaBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "45m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CyanBackgroundANSI;
impl ANSICode for CyanBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "46m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct WhiteBackgroundANSI;
impl ANSICode for WhiteBackgroundANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "47m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct UnderlineOnANSI;
impl ANSICode for UnderlineOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "4m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct UnderlineOffANSI;
impl ANSICode for UnderlineOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "24m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct ItalicOnANSI;
impl ANSICode for ItalicOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "3m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct ItalicOffANSI;
impl ANSICode for ItalicOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "23m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BoldOnANSI;
impl ANSICode for BoldOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "1m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BoldOffANSI;
impl ANSICode for BoldOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "22m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct DimOnANSI;
impl ANSICode for DimOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "2m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct DimOffANSI;
impl ANSICode for DimOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "22m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct StrikeThroughOnANSI;
impl ANSICode for StrikeThroughOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "9m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct StrikeThroughOffANSI;
impl ANSICode for StrikeThroughOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "29m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlinkOnANSI;
impl ANSICode for BlinkOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "5m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct BlinkOffANSI;
impl ANSICode for BlinkOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "25m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct InvisibleOnANSI;
impl ANSICode for InvisibleOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "8m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct InvisibleOffANSI;
impl ANSICode for InvisibleOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "28m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct InverseOnANSI;
impl ANSICode for InverseOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "7m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct InverseOffANSI;
impl ANSICode for InverseOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "27m"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CursorOnANSI;
impl ANSICode for CursorOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "?25h"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct CursorOffANSI;
impl ANSICode for CursorOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "?25l"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct LineWrapOnANSI;
impl ANSICode for LineWrapOnANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "?7h"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}

#[derive(Copy, Clone)]
pub struct LineWrapOffANSI;
impl ANSICode for LineWrapOffANSI {
    fn value(&self) -> String {
        let csi: String = EscapeANSI.combine(&CtrlSeqANSI).value();
        csi + "?7l"
    }

    fn combine(&self, other: &dyn ANSICode) -> CombinedANSI {
        CombinedANSI(self.value() + &other.value())
    }
}
