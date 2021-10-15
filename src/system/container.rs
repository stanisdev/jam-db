use std::mem::MaybeUninit;
use std::collections::HashMap;
use std::sync::{Once};
use super::source::Source;

pub struct Container {
    storage: HashMap<&'static str, String>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &'static str, value: String) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &str) -> &str {
        self.storage.get(key).unwrap().as_str()
    }
}

pub fn get_container() -> &'static mut Container {
    static mut CONTAINER: MaybeUninit<Container> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONTAINER.write(Container::new());
    });

    unsafe { &mut *CONTAINER.as_mut_ptr() }
}

pub fn get_source() -> &'static Source<'static> {
    static mut CONFIG_CONTENTS: String = String::new();
    static mut SOURCE: MaybeUninit<Source> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        SOURCE.write(Source::new(&mut CONFIG_CONTENTS));
    });

    unsafe { &*SOURCE.as_ptr() }
}
