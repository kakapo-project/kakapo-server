
import { combineReducers } from 'redux'
import table from './table'
import sidebar from './sidebar'
import entityCreator from './entityCreator'
import data from './data'

import createTable from './createTable'
import createScript from './createScript'
import createQuery from './createQuery'

export default combineReducers({
  sidebar,
  table,
  entityCreator,
  data,
  createQuery,
  createTable,
  createScript,
})
