use std::collections::{BTreeMap, btree_map::Entry};

use item::*;

use super::ItemSortOrder;

pub struct ItemStore {
  item_map: BTreeMap<ItemId, Item>,
}

#[derive(Debug)]
pub struct DuplicateItemError;

#[derive(Debug)]
pub struct NonExistentItemError;

impl ItemStore {
  pub fn new() -> Self {
    ItemStore {
      item_map: BTreeMap::new(),
    }
  }

  /// Iterate on items by the given sort order.
  pub fn item_iter(
    &self,
    sort_order: &ItemSortOrder,
  ) -> impl IntoIterator<Item = &Item> {
    self
      .id_iter(sort_order)
      .into_iter()
      .map(|id| self.item_map.get(&id).unwrap())
  }

  fn id_iter(
    &self,
    sort_order: &ItemSortOrder,
  ) -> impl IntoIterator<Item = ItemId> {
    match sort_order {
      ItemSortOrder::IdLexicographic => self.item_map.keys().copied(),
    }
  }

  pub fn add_item(&mut self, item: Item) -> Result<ItemId, DuplicateItemError> {
    match self.item_map.entry(item.id()) {
      Entry::Occupied(_) => Err(DuplicateItemError),
      entry => Ok(entry.insert_entry(item).get().id()),
    }
  }

  pub fn before_item(
    &self,
    sort_order: &ItemSortOrder,
    current: ItemId,
  ) -> Result<Option<ItemId>, NonExistentItemError> {
    let mut iter = self.id_iter(sort_order).into_iter();

    // if no items, current doesn't exist
    let Some(mut prev) = iter.next() else {
      return Err(NonExistentItemError);
    };

    // if the first item is the current, there is no item before
    if prev == current {
      return Ok(None);
    }

    loop {
      // if we get to the end without finding current, it doesn't exist
      let Some(id) = iter.next() else {
        return Err(NonExistentItemError);
      };

      // if we found the current, return the previous
      if id == current {
        return Ok(Some(prev));
      }

      prev = id;
    }
  }

  /// Find the item after the current item in the given sort order.
  ///
  /// Returns `Err(NonExistentItemError)` if `current` doesn't exist.
  /// Returns `None` if `current` is the last item in the order.
  pub fn after_item(
    &self,
    sort_order: &ItemSortOrder,
    current: ItemId,
  ) -> Result<Option<ItemId>, NonExistentItemError> {
    let mut iter = self.id_iter(sort_order).into_iter();

    loop {
      // if we've run out, definitely return None
      let Some(id) = iter.next() else {
        return Ok(None);
      };

      // skip if we haven't found current
      if id != current {
        continue;
      }

      // return the next id
      return Ok(iter.next());
    }
  }

  /// Find the first item in the given sort order.
  //// Returns `None` if there are no items.
  pub fn first_item(&self, sort_order: &ItemSortOrder) -> Option<ItemId> {
    self.id_iter(sort_order).into_iter().next()
  }

  pub fn last_item(&self, sort_order: &ItemSortOrder) -> Option<ItemId> {
    self.id_iter(sort_order).into_iter().last()
  }
}
