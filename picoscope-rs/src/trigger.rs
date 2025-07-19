pub struct Trigger {
    pub mask: u8,
    pub kind: TriggerKind,
}

pub enum TriggerKind {
    Rising,
    Falling,
}
