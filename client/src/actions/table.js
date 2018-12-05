
import { WEBSOCKET_CONNECT, WEBSOCKET_SEND } from '@giantmachines/redux-websocket'
import { WS_URL } from './config'

import { ACTIONS } from './index'

export const tableWantsToLoad = (name) => {
  const url = `${WS_URL}/table/${name}`
  return {
    type: WEBSOCKET_CONNECT,
    payload: { url }
  }
}

export const requestingTableData = () => {
  return async (dispatch, getState) => {

    let sendGetTable = {
      action: 'getTable',
    }

    let sendGetTableData = {
      action: 'getTableData',
      begin: 0,
      end: 500,
    }

    return dispatch([
      {
        type: WEBSOCKET_SEND,
        payload: sendGetTable,
      },
      {
        type: WEBSOCKET_SEND,
        payload: sendGetTableData,
      },
    ])
  }
}

export const addRow = (idx) => {

}

export const deleteRow = (idx) => {
  return async (dispatch, getState) => {
    let state = getState()

    let data = state.table.data
    let columns = state.table.columns
    let primaryKey = state.table.primaryKey

    let primaryKeyIndex = columns.findIndex(x => x === primaryKey)
    let value = data[idx][primaryKeyIndex]

    let deleteRow = {
      action: 'delete',
      data: value,
    }

    console.log('action: ', deleteRow)

    return dispatch([
      {
        type: WEBSOCKET_SEND,
        payload: deleteRow,
      },
      {
        type: ACTIONS.DELETE_ROW,
        idx: idx,
      },
    ])
  }
}

export const modifyValue = (rowIdx, colIdx, value) => {

}