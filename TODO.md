
* Dependency inversion for items in `handlers.rs`

Oct 26, Grid Frontend
- Grid is the frontend for views and tables

- Figure out what other sql function we want to have
  - join
  - Groupby
  - sum, avg, ...
- right click column headers to
  - create sort filter
  - expand for join
  - add filter
- right click row index to
  - create new row
  - duplicate row
  - delete row
- Adding new items take 60 seconds to filter
- Save + refresh buttons for view (Save is update or create new)
- views can act on queries or tables
- align right for numbers, left for text, <pre> for json, checkbox for boolean
- show red for unique or foreign key error, and yellow for unique key error
- add button to grid for creating new tables and queries