extern crate fmod_studio;

#[test]
fn basic_construction() {
    fmod_studio::system::System::new(512, false).unwrap();
}
