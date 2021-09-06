use super::Item;

#[derive(Debug)]
pub enum Command {
    Create,
    AddItem(Item),
    Complete,
    Cancel,
}
