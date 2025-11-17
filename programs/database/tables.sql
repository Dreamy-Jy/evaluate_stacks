-- ============================================================================
-- To Do List Database Schema
-- ============================================================================
-- Enable foreign key constraints (SQLite specific)
PRAGMA foreign_keys = ON;

-- ============================================================================
-- Lists Table
-- ============================================================================
CREATE TABLE lists (
  lid INT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL
);

-- ============================================================================
-- Sets Table
-- ============================================================================
CREATE TABLE sets (
  lid INT NOT NULL,
  sid INT NOT NULL,
  title TEXT NOT NULL,

  PRIMARY KEY (lid, sid),

  FOREIGN KEY (lid) REFERENCES lists (lid) ON DELETE CASCADE
);

-- ============================================================================
-- To Dos Table
-- ============================================================================
CREATE TABLE todos (
  lid INT NOT NULL,
  sid INT, -- NULL means todo belongs directly to list
  tdid INT NOT NULL,
  title TEXT NOT NULL,
  complete BOOLEAN NOT NULL DEFAULT 0,
  due_date DATETIME, -- NULL means no due date

  PRIMARY KEY (lid, tdid),

  FOREIGN KEY (lid) REFERENCES lists (lid) ON DELETE CASCADE,
  FOREIGN KEY (lid, sid) REFERENCES sets (lid, sid) ON DELETE CASCADE
);