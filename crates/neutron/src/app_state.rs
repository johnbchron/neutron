use std::collections::{HashMap, hash_map::Entry};

use item::{Dependency, Determinator, Item, ItemData, ItemId};

pub struct AppState {
  pub shutdown: bool,
  pub items:    ItemStore,
}

#[allow(clippy::derivable_impls)]
impl Default for AppState {
  fn default() -> Self {
    let mut items = ItemStore::new();
    let first_item = items
      .add_item(Item::new(
        ItemData {
          content:     "This Is A Test Item".to_string(),
          description: Some(
            "I created this item as a test of the item list view.".to_string(),
          ),
        },
        Determinator::Dependency(Dependency::All(Vec::new())),
      ))
      .unwrap();
    items
      .add_item(Item::new(
        ItemData {
          content:     "I'm a Second Test Item".to_owned(),
          description: Some(
            "I depend on the first item, so that we can test dependencies."
              .to_owned(),
          ),
        },
        Determinator::Dependency(Dependency::All(vec![first_item])),
      ))
      .unwrap();

    AppState {
      shutdown: false,
      items,
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
