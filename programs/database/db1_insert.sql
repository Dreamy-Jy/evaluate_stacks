-- ============================================================================
-- To Do List Data Import
-- ============================================================================
-- NOTE: This uses manually assigned IDs because the schema uses INT PRIMARY KEY.
-- For auto-increment, change the schema to use:
--   lid INTEGER PRIMARY KEY AUTOINCREMENT
--   sid INTEGER PRIMARY KEY AUTOINCREMENT (can't do this with composite keys)
--   tdid INTEGER PRIMARY KEY AUTOINCREMENT (can't do this with composite keys)
-- ============================================================================

-- ============================================================================
-- Insert Lists
-- ============================================================================

INSERT INTO lists (lid, title) VALUES
  (1, 'Chores'),
  (2, 'Community'),
  (3, 'Projects');

-- ============================================================================
-- Insert Sets
-- ============================================================================

INSERT INTO sets (lid, sid, title) VALUES
  (1, 1, 'Clean Pet'),
  (1, 2, 'Clean Bath Room'),
  (1, 3, 'Clean Room'),
  (1, 4, 'Grocery Shopping');

-- ============================================================================
-- Insert Todos
-- ============================================================================

-- Chores > Clean Bath Room (Today = 2025-11-18)
INSERT INTO todos (lid, sid, tdid, title, complete, due_date) VALUES
  (1, 2, 1, 'Scrub Toilet Bowl', 0, '2025-11-18T23:59:59Z'),
  (1, 2, 2, 'Disinfect Toilet', 0, '2025-11-18T23:59:59Z'),
  (1, 2, 3, 'Clean glass surfaces', 0, '2025-11-18T23:59:59Z'),
  (1, 2, 4, 'Sweep Floor', 0, '2025-11-18T23:59:59Z'),
  (1, 2, 5, 'Mop Floor', 0, '2025-11-18T23:59:59Z');

-- Chores > Clean Room (Yesterday = 2025-11-17)
INSERT INTO todos (lid, sid, tdid, title, complete, due_date) VALUES
  (1, 3, 6, 'Sweep Floor', 0, '2025-11-17T23:59:59Z'),
  (1, 3, 7, 'Mop Floor', 0, '2025-11-17T23:59:59Z'),
  (1, 3, 8, 'Clean Glass Surfaces', 0, '2025-11-17T23:59:59Z'),
  (1, 3, 9, 'Organize Room', 0, '2025-11-17T23:59:59Z'),
  (1, 3, 10, 'Fold Clothing', 0, '2025-11-17T23:59:59Z'),
  (1, 3, 11, 'Laundry', 0, '2025-11-17T23:59:59Z');

-- Chores (directly under list, no set)
INSERT INTO todos (lid, sid, tdid, title, complete, due_date) VALUES
  (1, NULL, 12, 'Sweep Floor', 0, NULL),
  (1, NULL, 13, 'Mop Floor', 0, NULL),
  (1, NULL, 14, 'Disinfect Surfaces', 0, NULL),
  (1, NULL, 15, 'Clean Glass Surfaces', 0, NULL),
  (1, NULL, 16, 'Clean Stove', 0, NULL),
  (1, NULL, 17, 'Clean Microwave', 0, NULL);

-- Community (all directly under list, no set)
INSERT INTO todos (lid, sid, tdid, title, complete, due_date) VALUES
  (2, NULL, 1, 'Go to NYC Resistor Craft Night', 0, '2025-11-20T23:59:59Z'),
  (2, NULL, 2, 'Go to Street Trash Cleanings', 0, '2025-11-25T23:59:59Z'),
  (2, NULL, 3, 'Go to Church', 0, '2025-11-17T23:59:59Z');

-- Projects list has no sets or todos