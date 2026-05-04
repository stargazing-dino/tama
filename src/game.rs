use embassy_time::{Duration, Instant};

use crate::sprites;

#[derive(Copy, Clone, Debug)]
pub enum Button {
    A,
    B,
}

#[derive(Copy, Clone, PartialEq, Debug, defmt::Format)]
enum State {
    Idle,
    Walk,
    Sleep,
    Paw,
    Clean,
    Scared,
    Feed,
    Pet,
}

struct AnimDef {
    frames: &'static [[u16; sprites::SPRITE_W * sprites::SPRITE_H]],
    frame_ms: u32,
}

const IDLE_DEF: AnimDef = AnimDef { frames: &sprites::IDLE_A, frame_ms: 250 };
const WALK_DEF: AnimDef = AnimDef { frames: &sprites::WALK_A, frame_ms: 120 };
const SLEEP_DEF: AnimDef = AnimDef { frames: &sprites::SLEEP, frame_ms: 400 };
const PAW_DEF: AnimDef = AnimDef { frames: &sprites::PAW, frame_ms: 150 };
const CLEAN_DEF: AnimDef = AnimDef { frames: &sprites::CLEAN_A, frame_ms: 200 };
const SCARED_DEF: AnimDef = AnimDef { frames: &sprites::SCARED, frame_ms: 100 };
const FEED_DEF: AnimDef = AnimDef { frames: &sprites::JUMP, frame_ms: 120 };
const PET_DEF: AnimDef = AnimDef { frames: &sprites::IDLE_B, frame_ms: 250 };

fn anim_for(s: State) -> &'static AnimDef {
    match s {
        State::Idle => &IDLE_DEF,
        State::Walk => &WALK_DEF,
        State::Sleep => &SLEEP_DEF,
        State::Paw => &PAW_DEF,
        State::Clean => &CLEAN_DEF,
        State::Scared => &SCARED_DEF,
        State::Feed => &FEED_DEF,
        State::Pet => &PET_DEF,
    }
}

pub struct Cat {
    state: State,
    entered: Instant,
    last_roll: Instant,
    frame: usize,
    last_frame_at: Instant,
    rng: u32,
}

impl Cat {
    pub fn new(now: Instant) -> Self {
        Self {
            state: State::Idle,
            entered: now,
            last_roll: now,
            frame: 0,
            last_frame_at: now,
            rng: (now.as_ticks() as u32) | 1,
        }
    }

    pub fn tick(
        &mut self,
        now: Instant,
        btn: Option<Button>,
    ) -> &'static [u16; sprites::SPRITE_W * sprites::SPRITE_H] {
        if let Some(b) = btn {
            let next = match b {
                Button::A => State::Feed,
                Button::B => State::Pet,
            };
            self.transition(next, now);
        }

        let dwell = now - self.entered;
        let next = match self.state {
            State::Idle => self.idle_dice(now),
            State::Walk if dwell > Duration::from_secs(4) => Some(State::Idle),
            State::Sleep if dwell > Duration::from_secs(10) => Some(State::Idle),
            State::Paw if dwell > Duration::from_secs(2) => Some(State::Idle),
            State::Clean if dwell > Duration::from_secs(3) => Some(State::Idle),
            State::Scared if dwell > Duration::from_secs(2) => Some(State::Idle),
            State::Feed if dwell > Duration::from_secs(3) => Some(State::Idle),
            State::Pet if dwell > Duration::from_secs(3) => Some(State::Idle),
            _ => None,
        };
        if let Some(s) = next {
            self.transition(s, now);
        }

        let def = anim_for(self.state);
        if (now - self.last_frame_at).as_millis() >= def.frame_ms as u64 {
            self.frame = (self.frame + 1) % def.frames.len();
            self.last_frame_at = now;
        }
        &def.frames[self.frame]
    }

    fn transition(&mut self, next: State, now: Instant) {
        if next == self.state {
            return;
        }
        defmt::info!("cat: {:?} -> {:?}", self.state, next);
        self.state = next;
        self.entered = now;
        self.last_roll = now;
        self.frame = 0;
        self.last_frame_at = now;
    }

    fn idle_dice(&mut self, now: Instant) -> Option<State> {
        if now - self.last_roll < Duration::from_secs(1) {
            return None;
        }
        self.last_roll = now;
        match self.rand() % 24 {
            0 => Some(State::Walk),
            1 => Some(State::Paw),
            2 => Some(State::Clean),
            3 => Some(State::Sleep),
            _ => None,
        }
    }

    fn rand(&mut self) -> u32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        x
    }
}
