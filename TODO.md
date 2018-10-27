
* Dependency inversion for items in `handlers.rs`

Oct 26, Grid Frontend
- Grid is the frontend for views and tables
- Banners at the top
  - Select
  - Where
  - Sort
  - Limit/Offset
- Less important banners
  - join
  - Groupby
  - sum, avg, ...
- Column headers have icon of key for primary key and link for foreign key
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
-