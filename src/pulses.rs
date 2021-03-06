/* Comments are a little archeology to remember what I was doing 8 months ago
 * I mostly forgot, but am slowly remembering.
 * A SyncSignal represents the output generated by mult430 */

pub enum SyncKind {
    PulseTrain(u16),
    SquareWave,
}

pub struct SyncSignal {
    pub low_time : u16,
    pub high_time : u16,
    pub multiplicand : u16,
    kind : SyncKind,
}

impl SyncSignal {
    pub fn new(kind : SyncKind) -> SyncSignal {
        SyncSignal {
            /// Units are arbitrary clock ticks
            low_time: 0,
            /// Units are arbitrary clock ticks
            high_time: 0,
            multiplicand: 1,
            kind,
        }
    }

    pub fn set_length(&mut self, length: u16) {
        self.high_time = match self.kind {
            SyncKind::PulseTrain(high_length) => high_length,
            SyncKind::SquareWave => length / 2,
        };

        self.low_time = length - self.high_time;
    }
}
