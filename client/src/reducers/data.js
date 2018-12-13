
import { ACTIONS } from '../actions'

const initialState = {
  error: null,
  tables: [],
  queries: [],
  script: [],
}

const entityCreator = (state = initialState, action) => {
  switch (action.type) {
    case ACTIONS.PULL_DATA_ERROR:
      return {
        ...state,
        error: action.msg,
      }
    case ACTIONS.CLEAR_PULL_DATA_ERROR:
      return {
        ...state,
        error: null,
      }
    case ACTIONS.SET_TABLE_DATA:
      return {
        ...state,
        tables: action.entities,
      }
    case ACTIONS.SET_QUERY_DATA:
      return {
        ...state,
        queries: action.entities,
      }
    case ACTIONS.SET_SCRIPT_DATA:
      return {
        ...state,
        script: action.entities,
      }
    default:
      return state
  }
}

export default entityCreator