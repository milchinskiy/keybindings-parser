use keybindings_parser::{KeyAction, Keybindings, ModMask};
use keysymdefs::keys;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
enum TestAction {
    Kill,
    ToggleFloat,
    ToggleFullscreen,
    ToggleTabBar,
    SwapWindows,
    Spawn(String),
}

impl KeyAction for TestAction {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TestAction::Kill => {}
            TestAction::Spawn(_) => {}
            TestAction::SwapWindows => {}
            TestAction::ToggleFloat => {
                return Err("some error")?;
            }
            TestAction::ToggleFullscreen => {}
            TestAction::ToggleTabBar => {}
        };
        Ok(())
    }
}

#[test]
fn test_handle_single_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyb = Keybindings::default();
    let origin = "ctrl + a";
    keyb.add(origin, Box::new(TestAction::ToggleFloat))?;

    if let Some(handler) = keyb.handle(ModMask::CONTROL, keysymdefs::keys::XK_a) {
        assert_eq!(handler.modifier(), ModMask::CONTROL);
        assert_eq!(handler.key(), keys::XK_a);
        assert_eq!(handler.origin(), origin);
        assert!(handler.action().run().is_err());
        assert_eq!(
            handler.action().run().err().unwrap().to_string(),
            "some error".to_string()
        );
    }

    Ok(())
}

#[test]
fn test_capital_letter() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyb = Keybindings::default();

    keyb.add("command + A", Box::new(TestAction::Kill))?;

    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_A).is_some());
    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a).is_none());

    Ok(())
}

#[test]
fn test_ghost_modificator() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyb = Keybindings::default();
    keyb.add("command + a", Box::new(TestAction::Kill))?;

    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a).is_some());
    assert!(keyb
        .handle(ModMask::MOD4 | ModMask::MOD2, keysymdefs::keys::XK_a)
        .is_some());
    assert!(keyb
        .handle(ModMask::MOD4 | ModMask::MOD5, keysymdefs::keys::XK_a)
        .is_some());
    assert!(keyb
        .handle(
            ModMask::MOD4 | ModMask::MOD5 | ModMask::MOD2,
            keysymdefs::keys::XK_a
        )
        .is_some());

    Ok(())
}

#[test]
fn test_no_ghost_modificators() {
    let mut keyb = Keybindings::new('+', ModMask::NONE);
    assert!(keyb.add("command + a", Box::new(TestAction::Kill)).is_ok());

    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a).is_some());
    assert!(keyb
        .handle(ModMask::MOD4 | ModMask::MOD2, keysymdefs::keys::XK_a)
        .is_none());
    assert!(keyb
        .handle(ModMask::MOD4 | ModMask::MOD5, keysymdefs::keys::XK_a)
        .is_none());
    assert!(keyb
        .handle(
            ModMask::MOD4 | ModMask::MOD5 | ModMask::MOD2,
            keysymdefs::keys::XK_a
        )
        .is_none());
}

#[test]
fn test_custom_delimiter() {
    let mut keyb = Keybindings::new('/', ModMask::NONE);
    assert!(keyb.add("command / a", Box::new(TestAction::Kill)).is_ok());
    assert!(keyb.add("command + b", Box::new(TestAction::Kill)).is_err());
    assert!(keyb
        .add("command // b", Box::new(TestAction::Kill))
        .is_err());
    assert!(keyb
        .add("command \\// b", Box::new(TestAction::Kill))
        .is_err());

    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a).is_some());
    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_b).is_none());
}

#[test]
fn test_no_modkey() {
    let mut keyb = Keybindings::default();
    assert!(keyb.add("command + a", Box::new(TestAction::Kill)).is_ok());
    assert!(keyb.add("b", Box::new(TestAction::Kill)).is_ok());

    assert!(keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a).is_some());
    assert!(keyb.handle(ModMask::NONE, keysymdefs::keys::XK_b).is_some());
}

#[test]
fn test_no_modkey_ghost_modkeys() {
    let mut keyb = Keybindings::default();
    assert!(keyb.add("command + a", Box::new(TestAction::Kill)).is_ok());
    assert!(keyb.add("b", Box::new(TestAction::Kill)).is_ok());

    assert!(keyb
        .handle(ModMask::MOD4 | ModMask::MOD2, keysymdefs::keys::XK_a)
        .is_some());
    assert!(keyb
        .handle(ModMask::NONE | ModMask::MOD5, keysymdefs::keys::XK_b)
        .is_some());
    assert!(keyb.handle(ModMask::MOD5, keysymdefs::keys::XK_b).is_some());
}
