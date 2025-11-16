# Endpoints

/api/rdb/todo/
/api/kv/todo/
/api/doc/todo/

Database Types
Relational
Key Value
Document

Join operations

AllLists () -> List[]
AllInList (lid) -> List
AllSetsInList (lid) -> Set[]
AllToDosInList (lid) -> ToDo[]
AllToDosInSet (lid, sid) -> ToDo[]

List (lid) -> List
SetInList (lid, sid) -> Set[]
ToDoInList (lid,tdid) -> ToDo[]
ToDoInSet (lid, sid, tdid) -> ToDo[]


- Lists
  - Sets
    - To Dos
  - To Dos
