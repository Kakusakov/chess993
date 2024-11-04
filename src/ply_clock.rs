use bytemuck::Contiguous;
use constmuck::contiguous;

#[derive(Debug, Clone, Copy)]
pub struct PlyClock(u8);

impl PlyClock {
    pub const fn zero() -> Self {
        Self(0)
    }
    pub fn with_tick(self) -> Self {
        Self((self.0 + 1).clamp(0, 50))
    }
}
