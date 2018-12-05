
import { WEBSOCKET_CLOSED, WEBSOCKET_OPEN, WEBSOCKET_MESSAGE } from '@giantmachines/redux-websocket'

const initialState = {
  isConnected: false,
  isTableMetaLoaded: false, //TODO: put these in reselect
  isTableDataLoaded: false, //TODO: put these in reselect
  isLoaded: false,
  error: null,

  columnInfo: {},

  data: [[]],
  columns: [],
}

const handleWebsocketMessage = (action, data, state) => {

  console.log('action: ', action)
  console.log('data: ', data)

  switch (action) {
    case 'getTable':
      let schema = data.schema
      let columnSchema = schema.columns
      let constraint = schema.constraint

      //get primary key
      let primaryKey = constraint.map(x => x.key).filter(x => x)
      if (primaryKey.length !== 1) {
        return { error: 'This table does not have any keys' }
      }
      primaryKey = primaryKey[0]

      //format column infor into map
      let columnInfo = {}
      for (let x of columnSchema) {
        columnInfo[x.name] = x
      }

      //done
      return {
        columnInfo: columnInfo,
        constraint: constraint,
        primaryKey: primaryKey,

        isTableMetaLoaded: true,
        isLoaded: state.isTableDataLoaded
      }
    case 'getTableData':
      let dataset = data.data
      let columns = data.columns

      return {
        data: dataset,
        columns: columns,

        isTableDataLoaded: true,
        isLoaded: state.isTableMetaLoaded
      }

    case 'update':
    case 'create':

  }

  return state
}

const table = (state = initialState, action) => {
  console.log('received action: ', action)
  switch (action.type) {
    case WEBSOCKET_OPEN:
      return {
        ...state,
        isConnected: true,
        isLoaded: false,
        isTableMetaLoaded: false,
        isTableDataLoaded: false,
      }
    case WEBSOCKET_CLOSED:
      return {
        ...state,
        isConnected: false,
        isLoaded: false,
        isTableMetaLoaded: false,
        isTableDataLoaded: false,
      }
    case WEBSOCKET_MESSAGE:
      let { data, event } = action.payload
      console.log('received message: ', data)
      console.log('event: ', event)

      let json = JSON.parse(data)

      let stateModification = handleWebsocketMessage(json.action, json.data, state)
      return { ...state, ...stateModification }

    default:
      return state
  }
}

export default table