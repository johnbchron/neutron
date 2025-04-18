mod item_store;
mod test_data;

use item::*;

pub use self::item_store::*;

pub enum Screen {
  ItemList(ItemListState),
  ItemGraph(ItemGraphState),
}

pub struct ItemListState {
  pub sort_order: ItemSortOrder,
  pub selected:   Option<ItemId>,
}

#[derive(Clone, Copy, Debug)]
pub enum ItemSortOrder {
  IdLexicographic,
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
      screen:   Screen::ItemList(ItemListState {
        selected:   None,
        sort_order: ItemSortOrder::IdLexicographic,
      }),
    }
  }
}
