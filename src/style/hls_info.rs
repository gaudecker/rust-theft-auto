pub struct HlsInfo {
    pub h: i16,
    pub l: i16,
    pub s: i16
}

impl HlsInfo {
    pub fn new(h: i16, l: i16, s: i16) -> HlsInfo {
        HlsInfo {
            h: h,
            l: l,
            s: s
        }
    }

    pub fn zero() -> HlsInfo {
        HlsInfo {
            h: 0,
            l: 0,
            s: 0
        }
    }
}
