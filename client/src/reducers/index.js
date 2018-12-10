
import { combineReducers } from 'redux'
import table from './table'
import sidebar from './sidebar'
import entityCreator from './entityCreator'
import data from './data'

export default combineReducers({
  sidebar,
  table,
  entityCreator,
  data,
})
