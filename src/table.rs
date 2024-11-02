#[derive(Debug, Clone, Copy, Default)]
pub struct Entry<T> {
    pub hash: u64,
    pub value: T
}

#[derive(Debug, Clone, Copy)]
pub struct Table<T, const SIZE: usize>([Entry<T>; SIZE]);

impl<T, const SIZE: usize> Table<T, SIZE>
where
    T: Copy
{
    fn from_copy(value: T) -> Self {
        let entry = Entry {
            hash: 0,
            value,
        };
        Self([entry; SIZE])
    }
}

impl<T, const SIZE: usize> Table<T, SIZE> {
    pub fn entry(&self, hash: u64) -> &Entry<T> {
        &self.0[(hash & Self::MASK) as usize]
    }
    pub fn entry_mut(&mut self, hash: u64) -> &mut Entry<T> {
        &mut self.0[(hash & Self::MASK) as usize]
    }
    pub fn get(&self, hash: u64) -> Option<&T> {
        let entry = self.entry(hash);
        match entry.hash == hash {
            true => Some(&entry.value),
            false => None,
        }
    }
    pub fn get_mut(&mut self, hash: u64) -> Option<&mut T> {
        let entry = self.entry_mut(hash);
        match entry.hash == hash {
            true => Some(&mut entry.value),
            false => None,
        }
    }
}

impl<T, const SIZE: usize> Table<T, SIZE> {
    const _TEST: () = {
        assert!(SIZE.is_power_of_two(), "size must be a power of two");
    };
    const MASK: u64 = SIZE as u64 - 1;
}
