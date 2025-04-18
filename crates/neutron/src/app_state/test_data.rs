use item::*;

use super::ItemStore;

pub(super) fn test_items() -> ItemStore {
  let mut items = ItemStore::new();
  let first_item = items
    .add_item(Item::new(
      ItemMetadata {
        content:     "Test Condition".to_string(),
        description: Some(
          "An arbitrary condition, used for testing.".to_string(),
        ),
      },
      ItemType::Condition(Condition::Atomic { satisfied: false }),
    ))
    .unwrap();
  items
    .add_item(Item::new(
      ItemMetadata {
        content:     "Test Task".to_owned(),
        description: Some(
          "I depend on the condition, so that we can test dependencies."
            .to_owned(),
        ),
      },
      ItemType::Task(Task {
        completed:  false,
        dependence: Dependence::Single(first_item),
      }),
    ))
    .unwrap();

  items
}
