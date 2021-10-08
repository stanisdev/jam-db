use std::mem::MaybeUninit;
use std::collections::HashMap;
use std::sync::{Once};
use super::types::Dictionary;
use super::source::Source;

pub fn get_container() -> Dictionary {
    static mut CONTAINER: MaybeUninit<HashMap<&str, &str>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONTAINER.write(HashMap::new());
    });

    unsafe { &mut *CONTAINER.as_mut_ptr() }
}

pub fn get_source() -> &'static Source<'static> {
    static mut config_contents: String = String::new();
    static mut SOURCE: MaybeUninit<Source> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        SOURCE.write(Source::new(&mut config_contents));
    });

    unsafe { &*SOURCE.as_ptr() }
}
