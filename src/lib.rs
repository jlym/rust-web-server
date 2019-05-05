use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;

pub struct Semaphore {
    max: u32,
    usage: Arc<(Mutex<u32>, Condvar)>,
}

impl Semaphore {
    pub fn new(max: u32) -> Semaphore {
        return Semaphore{
            max,
            usage: Arc::new((Mutex::new(max), Condvar::new()))
        }
    }

    pub fn clone(&self) -> Semaphore {
        return Semaphore {
            max: self.max,
            usage: self.usage.clone(),
        };
    }

    pub fn enter(&self) {
        let &(ref mutex_count, ref condvar) = &*self.usage;
        let mut count = mutex_count.lock().unwrap();
        if *count > 0 {
            *count -= 1;
            return;
        }

        loop {
            count = condvar.wait(count).unwrap();
            if *count > 0 {
                *count -= 1;
                return;
            }
        }        
    }

    pub fn exit(&self) {
        let &(ref mutex_count, ref condvar) = &*self.usage;
        let mut count = mutex_count.lock().unwrap();
        *count += 1;
        condvar.notify_one();

    }
}
