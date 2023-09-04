use std::sync::RwLock;

pub struct RwUsize {
    value: RwLock<usize>,
}

impl RwUsize {
    pub fn new() -> Self {
        let value: RwLock<usize> = RwLock::new(0);
        RwUsize { value }
    }

    pub fn increment(&self) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                *write_guard += 1;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn decrement(&self) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                if *write_guard == 0 {
                    return Err("Thread count is already 0");
                }
                *write_guard -= 1;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn add(&self, value: usize) -> Result<(), &'static str> {
        match self.value.write() {
            Ok(mut write_guard) => {
                *write_guard += value;
                Ok(())
            }
            Err(_) => Err("Failed to acquire write lock"),
        }
    }

    pub fn get_count(&self) -> Result<usize, &'static str> {
        match self.value.read() {
            Ok(read_guard) => Ok(*read_guard),
            Err(_) => Err("Failed to acquire read lock"),
        }
    }
}
