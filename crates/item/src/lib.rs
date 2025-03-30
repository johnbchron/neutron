use std::{fmt, time::SystemTime};

use nanorand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(ulid::Ulid);

impl fmt::Debug for ItemId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("ItemId").field(&self.0.to_string()).finish()
  }
}

impl ItemId {
  pub fn new() -> Self {
    ItemId(ulid::Ulid::from_parts(
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("non-monotonic clock")
        .as_millis()
        .try_into()
        .expect("millis since unix epoch doesn't fit into u64"),
      nanorand::tls_rng().generate::<u128>(),
    ))
  }
  pub fn inner(&self) -> &ulid::Ulid { &self.0 }
}

impl Default for ItemId {
  fn default() -> Self { Self::new() }
}

pub struct Item {
  id:           ItemId,
  data:         ItemData,
  determinator: Determinator,
}

impl Item {
  pub fn new(data: ItemData, determinator: Determinator) -> Self {
    Item {
      id: ItemId::new(),
      data,
      determinator,
    }
  }

  pub fn id(&self) -> ItemId { self.id }
  pub fn data(&self) -> &ItemData { &self.data }
  pub fn determinator(&self) -> &Determinator { &self.determinator }
}

pub struct ItemData {
  pub content:     String,
  pub description: Option<String>,
}

pub enum Determinator {
  Dependency(Dependency),
}

pub enum Dependency {
  All(Vec<ItemId>),
}
