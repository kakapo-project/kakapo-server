
### Data
- Other SQL databases, MySQL, Sqlite, etc.
- No SQL databases, redis, mongodb, cassandra, hive - **IMPORTANT**
- Data stores, file system, amazon S3
- REST api creator
- REST api creator GUI
- View generator, generate SQL - **IMPORTANT**
- More functions like SUM, Groupby, average, etc.
- SQL creator GUI
- Scaling GUI

### Scripts
- Each script should have a separate user
- Run on other environments
    - docker
    - JVM
    - virtualization
    - serverless
- Support other languages
    - scala
    - java
    - javascript
    - R
- schedule tasks (cron) - **IMPORTANT**
- Task dependencies
- Take a look at celery to get some influence on what we may need
- Take a look at airflow to get some influence on what we may need

### Frontend
- Adding new items take 60 seconds to filter
- Save + refresh buttons for view (Save is update or create new)
- views can act on queries or tables
- \<pre\> for json, checkbox for boolean, date for date
- show red for unique or foreign key error, and yellow for unique key error
- add button to grid for creating new tables and queries
- Render background, figure out teletyping
- Fix slow rendering (too many cells are being rendered) -- figured out that this is due to development flags in react...
- For creating new table: add option for advanced sql features (i.e. precision for integer)
- rename string to Text, and integer -> Number
- holding shift for multiple selections
- do debounce on all inputs
- TODO: look into `https://github.com/reduxjs/reselect`

### Bugs:
- creating a new table with an old table name, will attempt to append the columns
- adding a new row with an old key should give an error?
- save the selection when returning
- inserting data into table doesn't fail properly
- script runner should ignore if no json is added for `/api/script/...`
- fix is_deleted, especially for onDuplicate=fail / onDuplicate=ignore needlessly fails
- sending integer as string puts in garbage, or nothing
- post query (`/script`) should only update current query, not insert other query or update
- differentiate boolean, integer, string, json in the front end
- upload data from csv
- api documentation
- `onDuplicate=fail` table row data should fail, not return empty array, `onDuplicate=ignore` should return old value
- Better color feedback for data entry

### PERFORMANCE
* if the post request is big, try async message handlers

### Quality of Life:
- add devtools (redux)
- More tests
- clippy