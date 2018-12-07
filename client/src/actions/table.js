
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
  return {
    type: ACTIONS.ADD_ROW,
    idx: idx,
  }
}

export const deleteRow = (idx) => {
  return async (dispatch, getState) => {
    let state = getState()

    let data = state.table.data
    let columns = state.table.columns
    let primaryKey = state.table.primaryKey

    let primaryKeyIdx = columns.findIndex(x => x === primaryKey)
    let key = data[idx][primaryKeyIdx]

    let deleteRow = {
      action: 'delete',
      data: key,
    }

    return dispatch([
      {
        type: WEBSOCKET_SEND,
        payload: deleteRow,
      },
      {
        type: ACTIONS.DELETE_ROW,
        idx: idx,
        key: key,
      },
    ])
  }
}

export const modifyValue = (rowIdx, colIdx, value) => {

  const updateVirtualValue = () => {
    return {
      type: ACTIONS.UPDATE_VALUE,
      rowIdx: rowIdx,
      colIdx: colIdx,
      value: value,
    }
  }

  const updateValue = (key, newRow) => {
    return [
      {
        type: WEBSOCKET_SEND,
        payload: {
          action: 'update',
          data: newRow,
          key: key,
        },
      },
      {
        type: ACTIONS.UPDATE_VALUE,
        rowIdx: rowIdx,
        colIdx: colIdx,
        value: value,
      },
    ]
  }

  const insertRow = (newRow) => {
    return [
      {
        type: WEBSOCKET_SEND,
        payload: {
          action: 'create',
          data: newRow,
        },
      },
      {
        type: ACTIONS.UPDATE_VALUE,
        rowIdx: rowIdx,
        colIdx: colIdx,
        value: value,
      },
    ]
  }

  return async (dispatch, getState) => {
    let state = getState()

    let data = state.table.data
    let columns = state.table.columns
    let primaryKey = state.table.primaryKey

    let primaryKeyIdx = columns.findIndex(x => x === primaryKey)
    let row = data[rowIdx]
    let key = row[primaryKeyIdx]

    //case 1, row is actually new, don't push to database until we have a key
    if (key === null && primaryKeyIdx !== colIdx) {
      return dispatch(updateVirtualValue())
    }
    //case 2, row is actually new, but we have the key and we can push
    else if (key === null && primaryKeyIdx === colIdx) {
      let newRow = {}
      columns.map((columnName, idx) => {
        newRow[columnName] = (idx == colIdx) ? value : row[idx]
      })
      return dispatch(insertRow(newRow))
    }
    //case 3, a value was modified
    else {
      let otherColumnName = columns[colIdx]
      let newRow = {
        [otherColumnName]: value,
      }
      return dispatch(updateValue(key, newRow))
    }
  }
}