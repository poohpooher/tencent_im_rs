use bitflags::bitflags;
use bitflags_serde_shim::impl_serde_for_bitflags;
bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct AssistFlag: u8 {
        const PINNED = 0b00000001; // 1
        const EMPTY = 0b00000010; // 2
        const PAGINATING_PINNED = 0b00000100; // 3
    }
}
impl_serde_for_bitflags!(AssistFlag);
