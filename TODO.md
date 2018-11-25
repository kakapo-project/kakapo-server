
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

Bugs:
- creating a new table with an old table name, will attempt to append the columns

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
 - implment get by chunksize

 Devops
 - Different functions
    - Auth controller: main entry point takes the input, authenticates and passes to **Accessor**
        - also does the websocket/pub-sub management
    - Accessor: handler for each user, one accessor per connected user
    - Docker Manager: not a docker instance, creates accessors on demand
    - Redis
    - Postgres
    - Frontend