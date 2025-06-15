#[derive(Clone, Debug)]
pub enum PlayerInput {
    Noop,
    Up,
    Down,
    Left,
    Right,
    Shoot,
    Dodge,
    Accept,
    Cancel,
}
