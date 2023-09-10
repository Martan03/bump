// Hi Bonny4, I'm using your library
// There's so much, I know! But I'm tired now

use raplay::Sink;

pub struct Player {
    sink: Sink
}

impl Player {
    pub fn new() -> Self {
        Player { sink: Sink::default() }
    }
}

