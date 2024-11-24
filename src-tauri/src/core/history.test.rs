use tauri::test::MockRuntime;

use super::History;
use crate::core::actions::MockAction;

#[test]
fn test_push() {
  let mut history = History::<MockRuntime>::default();

  history.push(Box::new(MockAction));
  assert_eq!(history.undo_stack.len(), 1);
  assert_eq!(history.redo_stack.len(), 0);

  history.push(Box::new(MockAction));
  assert_eq!(history.undo_stack.len(), 2);
  assert_eq!(history.redo_stack.len(), 0);
}

#[test]
fn test_undo() {
  let mut history = History::<MockRuntime>::default();

  history.push(Box::new(MockAction));
  history.push(Box::new(MockAction));
  assert_eq!(history.undo_stack.len(), 2);
  assert_eq!(history.redo_stack.len(), 0);

  assert!(history.undo().is_some());
  assert_eq!(history.undo_stack.len(), 1);
  assert_eq!(history.redo_stack.len(), 1);

  assert!(history.undo().is_some());
  assert_eq!(history.undo_stack.len(), 0);
  assert_eq!(history.redo_stack.len(), 2);
  assert!(history.undo().is_none());
}

#[test]
fn test_redo() {
  let mut history = History::<MockRuntime>::default();
  history.push(Box::new(MockAction));
  history.push(Box::new(MockAction));
  history.undo();

  assert!(history.redo().is_some());
  assert_eq!(history.undo_stack.len(), 2);
  assert_eq!(history.redo_stack.len(), 0);
  assert!(history.redo().is_none());
}
