use mlua::UserData;

use super::celltype::Celltype;

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
pub struct Cell {
    pub gx: u32,
    pub gy: u32,
    pub direction: Direction,
    pub celltype: Celltype,
}
impl Cell {
    pub fn name(&self) -> String { return self.celltype.name.clone(); }
    pub fn id(&self) -> String { return self.celltype.id.clone(); }
    pub fn pos(&self) -> (u32, u32) { return (self.gx.clone(), self.gy.clone()) }
    // pub fn delete(&self) -> (u32, u32) {return true }
    pub fn forward(&self, power: i32, forward: i32) -> (u32, u32) {
        return (self.gx.clone(), self.gy.clone())
    }
}
// impl LuaUserData for Celltype {}
impl UserData for Cell {
    fn add_methods<'lua, M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("name", |_, this, _: ()| {
            Ok(this.name())
        });
    }
}