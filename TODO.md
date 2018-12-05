
* Dependency inversion for items in `handlers.rs`

Grid Frontend
- Grid is the frontend for views and tables

- Figure out what other sql function we want to have
  - join
  - Groupby
  - sum, avg, ...
- Adding new items take 60 seconds to filter
- Save + refresh buttons for view (Save is update or create new)
- views can act on queries or tables
- <pre> for json, checkbox for boolean, date for date
- show red for unique or foreign key error, and yellow for unique key error
- add button to grid for creating new tables and queries
- Render background, figure out teletyping
- Fix slow rendering (too many cells are being rendered) -- figured out that this is due to development flags in react...
- For creating new table: add option for advanced sql features (i.e. precision for integer)
- rename string to Text, and integer -> Number
- holding shift for multiple selections
- do debounce on all inputs
- double click not working correctly, it should only double click if it is on different table cells
- clicking out of context menu should get out of it, don't need to click the cancel button
- TODO: look into `https://github.com/reduxjs/reselect`
- add sql creator (gui for full schema design)

Bugs:
- creating a new table with an old table name, will attempt to append the columns
- minor: adding a row should defocus the value input field
- until the data is updated, the user's input value change should be reflected
- sending integers not working
- adding a new row with an old key should give an error?
- save the selection when returning
- inserting data into table doesn't fail properly

Backend
- Hook up table creation to websocket
- Hook up table get data to websocket
- Hook up table create row to websocket
- Hook up table updates to websocket
- Hook up table delete to websocket
- Figure out docker
- Figure out authentication
- Excel + CSV maker
- plugins:
  - S3
  - airflow
 - Hook read all tables to websockets
 - implment get by chunksize ? maybe
 - fix code duplication in manage.rs: use inheritance to capture entity
 - script runner should ignore if no json is added for `/api/script/...`

Bugs:
- sending integer as string puts in garbage, or nothing
- post query (/script) should only update current query, not insert other query or update
- error handling query script
- weird bug: garbage when running the following, notice the location of ORDER with respect to character
```
SELECT * FROM "character"
ORDER BY "age" DESC
LIMIT 3;
```
- segmentation fault on some queries log:
```
final result: QueryWithData { query: Query { name: "my_special_query", description: "", statement: "SELECT * FROM \"diesel_demo\".\"character\"\n  LIMIT 3;" }, data: RowsFlatData { columns: [], data: [] } }
final result: Object({"columns": Array([]), "data": Array([])})
query already loaded: DataQuery { query_id: 3, entity_id: 13, name: "my_special_query" }
encountered error: DatabaseError(DatabaseError(ForeignKeyViolation, "insert or update on table \"query_history\" violates foreign key constraint \"query_history_modified_by_fkey\""))

Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)
```
- fix is_deleted, especially for onDuplicate=fail / onDuplicate=ignore needlessly fails


 Devops
 - Different functions
    - Auth controller: main entry point takes the input, authenticates and passes to **Accessor**
        - also does the websocket/pub-sub management
    - Accessor: handler for each user, one accessor per connected user
    - Docker Manager: not a docker instance, creates accessors on demand
    - Redis
    - Postgres
    - Frontend
- add devtools (redux)


Actual TODOS:
- Delete implementation
- differentiate boolean, integer, string, json in the front end
- Build the docker setup
- scripts support
- upload data from csv
- authentication
- upload files
- api documentation
- packaging with ./configure && make && make install