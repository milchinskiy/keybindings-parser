use keybindings_parser::{KeyAction, Keybindings, ModMask};
use keysymdefs::{get_item_by_cleared_name, keys};

#[test]
fn set_keybingings_ok() {
    let mut keyb = Keybindings::default();
    let result = keyb.add("super + d", Box::new(TestAction {}));
    assert!(result.is_ok());
}

#[test]
fn set_keybingings_fail() {
    let mut keyb = Keybindings::default();
    assert!(keyb
        .add("super + SomeNotExistingKey", Box::new(TestAction {}))
        .is_err());
    assert!(keyb.add("super + shift", Box::new(TestAction {})).is_err());
    assert!(keyb.add("super + super", Box::new(TestAction {})).is_err());
    assert!(keyb.add("a + b", Box::new(TestAction {})).is_err());
}

#[test]
fn set_keybingings_chain() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyb = Keybindings::default();
    let action1 = Box::new(TestAction {});
    let action2 = Box::new(TestAction {});
    assert!(keyb.add("super + d", action1).is_ok());
    assert!(keyb.add("alt + d", action2).is_ok());

    let result = keyb.add("alt + s", Box::new(TestAction2::Spawn("Hello".to_string())));
    assert!(result.is_ok());

    let result = keyb.add(
        "super + meta + shift + d",
        Box::new(TestAction2::Spawn("Hello".to_string())),
    );
    assert!(result.is_ok());

    let result = keyb.add("alt + q", Box::new(TestAction2::SomeOtherAction));
    assert!(result.is_ok());

    let handler = keyb
        .handle(
            ModMask::MOD4,
            get_item_by_cleared_name("d").unwrap().keysym(),
        )
        .unwrap();
    let act = handler.action();
    act.run()?;
    Ok(())
}

#[test]
fn set_duplicate_keybinding_fail() {
    let mut keyb = Keybindings::default();
    let result = keyb.add("super + d", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add("super + d", Box::new(TestAction {}));
    assert_eq!(
        result.err().unwrap().to_string(),
        "duplicate keybinding".to_string()
    );

    let result = keyb.add("super + a", Box::new(TestAction {}));
    assert!(result.is_ok());
}

#[test]
fn check_mod_case_sensitivity() {
    let mut keyb = Keybindings::default();
    let result = keyb.add("sUpEr + d", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add("SuPeR + e", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add("ConTROl + d", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add("AlT + sHiFt + d", Box::new(TestAction {}));
    assert!(result.is_ok());

    assert!(keyb.handle(ModMask::MOD4, keys::XK_d).is_some());
    assert!(keyb.handle(ModMask::MOD4, keys::XK_e).is_some());
    assert!(keyb.handle(ModMask::CONTROL, keys::XK_d).is_some());
    assert!(keyb
        .handle(ModMask::MOD1 | ModMask::SHIFT, keys::XK_d)
        .is_some());
}

#[test]
fn check_trim() {
    let mut keyb = Keybindings::default();
    let result = keyb.add(" super+ d ", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add(
        "super                 +           e      ",
        Box::new(TestAction {}),
    );
    assert!(result.is_ok());

    let result = keyb.add(" control+d ", Box::new(TestAction {}));
    assert!(result.is_ok());

    let result = keyb.add(" alt+ shift+d", Box::new(TestAction {}));
    assert!(result.is_ok());

    assert!(keyb.handle(ModMask::MOD4, keys::XK_d).is_some());
    assert!(keyb.handle(ModMask::MOD4, keys::XK_e).is_some());
    assert!(keyb.handle(ModMask::CONTROL, keys::XK_d).is_some());
    assert!(keyb
        .handle(ModMask::MOD1 | ModMask::SHIFT, keys::XK_d)
        .is_some());
}

struct TestAction {}
impl KeyAction for TestAction {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

enum TestAction2 {
    Spawn(String),
    SomeOtherAction,
}
impl KeyAction for TestAction2 {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TestAction2::Spawn(s) => println!("{s}"),
            TestAction2::SomeOtherAction => println!("Some other action"),
        }
        Ok(())
    }
}
