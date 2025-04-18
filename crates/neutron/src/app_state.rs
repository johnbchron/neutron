mod test_data;

use std::collections::{HashMap, hash_map::Entry};

use item::*;

pub enum Screen {
  ItemList(ItemListState),
  ItemGraph(ItemGraphState),
}

pub struct ItemListState {
  pub selected: Option<ItemId>,
}

pub struct ItemGraphState {}

pub struct AppState {
  pub shutdown: bool,
  pub items:    ItemStore,
  pub screen:   Screen,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      shutdown: false,
      items:    self::test_data::test_items(),
      screen:   Screen::ItemList(ItemListState { selected: None }),
    }
  }
}

pub struct ItemStore {
  item_map: HashMap<ItemId, Item>,
}

#[derive(Debug)]
pub struct DuplicateItemError;

impl ItemStore {
  fn new() -> Self {
    ItemStore {
      item_map: HashMap::new(),
    }
  }

  pub fn item_iter(&self) -> impl IntoIterator<Item = &Item> {
    self.item_map.values()
  }

  pub fn add_item(&mut self, item: Item) -> Result<ItemId, DuplicateItemError> {
    match self.item_map.entry(item.id()) {
      Entry::Occupied(_) => Err(DuplicateItemError),
      entry => Ok(entry.insert_entry(item).get().id()),
    }
  }
}
