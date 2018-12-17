
* Dependency inversion for items in `handlers.rs`

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
- TODO: look into `https://github.com/reduxjs/reselect`
- add sql creator (gui for full schema design)

Bugs:
- creating a new table with an old table name, will attempt to append the columns
- adding a new row with an old key should give an error?
- save the selection when returning
- inserting data into table doesn't fail properly

Backend
- Proper message queues and websockets
- Figure out docker
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
- differentiate boolean, integer, string, json in the front end
- Build the docker setup
- scripts support
- upload data from csv
- authentication
- upload files
- api documentation
- packaging with ./configure && make && make install
- onDuplicate=fail table row data should fail, not return empty array, onDuplicate=ignore should return old value
- Better color feedback for data entry