# Program Plans

Plan
Setup DB
Setup Rust Endpoints

## Backend

I'll be evaluating:

- The experience of using SQLite as Relational, Document, and Key Value Database.
- The ergonomics of writing CRUD APIs in a given language.

### Data Relationships & Addressing

- Lists (LID)
  - Sets (LID, SID)
    - To Dos (LID, SID, TDID)
  - To Dos (LID, TDID)

### Endpoint Patterns

Generally Speaking this what the endpoints will look like

- /api/<entity>/create
- /api/<entity>/read
- /api/<entity>/update
- /api/<entity>/delete

### API Schema

[AI generated protobuf of the schema.](schema.proto)

#### Base Types

Entity IDs

List ID will not collide. All Entities within a container are guaranteed not to collide, there maybe collisions within the total set of a given entity.

- LID (List ID)
- SID (Set ID)
- TDID (To Do ID)

Entity Addresses

- SetAddress
  - WholeList (LID)
  - Singular (LID, SID)

- ToDoAddress
  - WholeList (LID)
  - WholeSet (LID, SID)
  - Singular (LID, SID, TDID)

Entities

- List
  - LID
  - title: string
  - sets: Set[]
  - to dos: ToDo[]
- Set
  - LID
  - SID
  - title: string
  - to dos: ToDo[]
- ToDo
  - LID
  - SID?
  - TDID
  - title: string
  - complete: bool
  - due date: Datetime

Method Parameters

- ListCreateData
  - title: string
- SetCreateData
  - title: string
- ToDoCreateData
  - title: string
  - due date: DateTime
- ListChangeData
  - LID
  - title: string
- SetChangeData
  - LID
  - SID
  - title: string
- ToDoChangeData
  - LID
  - SID?
  - TDID
  - title: string
  - due date: DateTime
  - complete: bool

#### CRUD Methods & Signatures

All methods are assumed to be atomic

##### CREATE

- createLists : `(ListCreateData[]) -> bool`
- createSets : `(SetCreateData[]) -> bool`
- createToDos : `(ToDoCreateData[]) -> bool`

##### READ

- readLists : `(lid[]?) -> List[]`
- readSets : `(SetAddress[]?) -> Set[]`
- readToDos : `(ToDoAddress[]? ) -> ToDo[]`

##### UPDATE

- updateLists : `(ListChangeData[]) -> bool`
- updateSets : `(SetChangeData[]) -> bool`
- updateToDos : `(ToDoChangeData[]) -> bool`

##### DELETE

- deleteLists : `(lid[]) -> bool`
- deleteSets : `(SetAddress[]) -> bool`
- deleteToDos : `(ToDoAddress[]? ) -> bool`
