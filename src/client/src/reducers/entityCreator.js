
import { ACTIONS } from '../actions'

const initialState = {
  creatingEntities: false,
  entitiesDirty: false,
  mode: 'Table',

  error: null,
  tableName: null,
  scriptName: null,
  queryName: null,

  primaryKey: 0,
  columns: { 0: null },
}

const entityCreator = (state = initialState, action) => {
  switch (action.type) {
    case ACTIONS.ENTITY_CREATOR.ERROR:
      return {
        ...state,
        error: action.msg,
      }
    case ACTIONS.ENTITY_CREATOR.SET_MODE:
      return {
        ...state,
        mode: action.mode,
      }
    case ACTIONS.ENTITY_CREATOR.CLEAR_ERROR:
      return {
        ...state,
        error: null,
      }
    case ACTIONS.ENTITY_CREATOR.CLEAR_DIRTY_ENTITIES:
      return {
        ...state,
        entitiesDirty: false,
      }
    case ACTIONS.ENTITY_CREATOR.START_CREATING_ENTITIES:
      return {
        ...state,
        creatingEntities: true,
      }
    case ACTIONS.ENTITY_CREATOR.COMMIT_CHANGES:
      return {
        ...state,
        creatingEntities: false,
        entitiesDirty: true,
      }
    case ACTIONS.ENTITY_CREATOR.SET_TABLE_NAME:
      return {
        ...state,
        tableName: action.name,
      }
    case ACTIONS.ENTITY_CREATOR.SET_TABLE_NAME:
      return {
        ...state,
        tableName: action.name,
      }
    case ACTIONS.ENTITY_CREATOR.SET_QUERY_NAME:
      return {
        ...state,
        queryName: action.name,
      }
    case ACTIONS.ENTITY_CREATOR.SET_SCRIPT_NAME:
      return {
        ...state,
        scriptName: action.name,
      }
    case ACTIONS.ENTITY_CREATOR.MODIFY_STATE:
      return {
        ...state,
        columns: action.columns,
        primaryKey: action.primaryKey,
      }
    default:
      return state
  }
}

export default entityCreator