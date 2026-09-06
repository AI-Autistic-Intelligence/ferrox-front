use wasm_bindgen_test::*;
use ferrox_front_core::reactivity::Signal;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_signal_reactivity() {
    let mut signal = Signal::new(10);
    assert_eq!(signal.get(), 10);
    
    signal.set(20);
    assert_eq!(signal.get(), 20);
    
    signal.update(|val| *val += 5);
    assert_eq!(signal.get(), 25);
}
