
import { WEBSOCKET_CONNECT, WEBSOCKET_SEND } from '@giantmachines/redux-websocket'
import { WS_URL } from './config'

import { ACTIONS } from './index'

import ClipBoard from 'clipboard'
const csvParse = require('csv-stringify/lib/sync')

const encodeByType = (data, type) => {
  switch (type) {
    case 'string':
      return data
    case 'integer':
      return parseInt(data)
  }
}

function clipboardWrite(text, event) {
  const cb = new ClipBoard('.null', {
      text: () => text
  });

  cb.on('success', function(e) {
      console.log(e);
      cb.off('error');
      cb.off('success');
  });

  cb.on('error', function(e) {
      console.log(e);
      cb.off('error');
      cb.off('success');
  });

  cb.onClick(event);
}

export const tableWantsToLoad = (name) => {
  return async (dispatch, getState) => {
    const url = `${WS_URL}/table/${name}`
    return dispatch({
      type: WEBSOCKET_CONNECT,
      payload: { url }
    })
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

export const copySelection = (topLeft, bottomRight, e) => {
  return async (dispatch, getState) => {
    let y0 = topLeft.idx
    let x0 = topLeft.col
    let y1 = bottomRight.idx
    let x1 = bottomRight.col

    let state = getState()
    let data = state.table.data

    let filteredData = data.slice(y0, y1 + 1).map(x => x.slice(x0, x1 + 1))

    let output = csvParse(filteredData) //need to use the sync api for the clipboard write to work (this is a browser restriction)
    clipboardWrite(output, e)
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

    let deletedRow = {
      action: 'delete',
      key: key,
    }

    return dispatch([
      {
        type: WEBSOCKET_SEND,
        payload: deletedRow,
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
    let columnData = state.table.columnInfo
    console.log('columnData: ', columnData)

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
        let type = columnData[columnName].dataType
        newRow[columnName] = encodeByType((idx == colIdx) ? value : row[idx], type)
      })
      return dispatch(insertRow(newRow))
    }
    //case 3, a value was modified
    else {
      let otherColumnName = columns[colIdx]
      let type = columnData[otherColumnName].dataType
      let newRow = {
        [otherColumnName]: encodeByType(value, type),
      }
      return dispatch(updateValue(key, newRow))
    }
  }
}