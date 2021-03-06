use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::mem;

#[derive(Clone)]
pub struct MainWnd {
    pub inner: Arc<Mutex<u8>>,
}
impl MainWnd {
    #[allow(dead_code)]
    fn init(&self)
    {

    }
    #[allow(dead_code)]
    pub fn instance() -> MainWnd {
        // Initialize it to a null value
        static mut SINGLETON: *const MainWnd = 0 as *const MainWnd;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {

                // Make it
                let instance = MainWnd {
                    inner: Arc::new(Mutex::new(0))
                };

                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(instance));
            });

            // Now we give out a copy of the data that is safe to use concurrently.
            (*SINGLETON).clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_load_obj() {
    }
}
