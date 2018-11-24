
* Dependency inversion for items in `handlers.rs`

Oct 26, Grid Frontend
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
- Fix slow rendering (too many cells are being rendered)

Bugs:
- creating a new table with an old table name, will attempt to append the columns

Oct 28, Backend
- Figure out websocket
- Hook up table creation to websocket
- Hook up table get data to websocket
- Hook up table create row to websocket
- Hook up table updates to websocket
- Hook up table delete to websocket
- When queries are run, there is no way of determining the type of object, therefore we need to add `PQfType` to diesel to do the dynamic type inference
- Figure out docker
- Figure out authentication
- Excel + CSV maker
- plugins:
  - S3
  - airflow
 
 
 Devops
 - Different functions
    - Auth controller: main entry point takes the input, authenticates and passes to **Accessor**
        - also does the websocket/pub-sub management
    - Accessor: handler for each user, one accessor per connected user
    - Docker Manager: not a docker instance, creates accessors on demand
    - Redis
    - Postgres
    - Frontend