

import { ACTIONS, pullData } from './index'

import { API_URL } from './config'
import { DEFAULT_TYPE, ALL_TYPES } from './columns'


export const setTableName = (name) => {
  return {
    type: ACTIONS.ENTITY_CREATOR.SET_TABLE_NAME,
    name: name,
  }
}

export const startCreatingEntities = () => {
  return {
    type: ACTIONS.ENTITY_CREATOR.START_CREATING_ENTITIES,
  }
}

export const exitCreatingEntities = () => {
  return {
    type: ACTIONS.ENTITY_CREATOR.COMMIT_TABLE_CHANGES,
  }
}

const getAllKeys = (obj) => Object.keys(obj).map(x => parseInt(x))

export const commitTableChanges = () => {

  return async (dispatch, getState) => {

    let data = getState().entityCreator
    console.log('entityCreator: ', data)
    if (!data.tableName) {
      return dispatch({
        type: ACTIONS.ENTITY_CREATOR.ERROR,
        msg: 'No table name given',
      })
    }
    let columnsObj = data.columns
    let primaryKeyColumn = columnsObj[data.primaryKey]
    if (!primaryKeyColumn || !primaryKeyColumn.name) {
      return dispatch({
        type: ACTIONS.ENTITY_CREATOR.ERROR,
        msg: 'Primary key is empty',
      })
    }

    let columnIdx = getAllKeys(columnsObj)
    columnIdx.sort()
    let columns = columnIdx
      .map(idx => columnsObj[idx])
      .filter(x => x !== null)
    for (let column of columns) {
      if (!column.name) {
        return dispatch({
          type: ACTIONS.ENTITY_CREATOR.ERROR,
          msg: 'column is empty',
        })
      }
    }
    //parse data
    let postData = {
      name: `${data.tableName}`,
      description: '',
      action: {
        type: 'create',
        columns: columns.map(x => (
          {
            name: x.name,
            dataType: x.typeName || DEFAULT_TYPE
          }
        )),
        constraint: [
          {
            key: primaryKeyColumn.name
          }
        ]
      }
    }

    //send
    fetch(`${API_URL}/manage/table`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json; charset=utf-8',
      },
      body: JSON.stringify(postData),
    })
      .then(response => {
        return response.json()
      })
      .then(data => {
        console.log('finished sending data')
        console.log(data)
        if (data.error) { //For some reason it returned an error message, but it was a 200 http code
          return dispatch({
            type: ACTIONS.ENTITY_CREATOR.ERROR,
            msg: data.error,
          })
        } else {
          return dispatch({
            type: ACTIONS.ENTITY_CREATOR.COMMIT_TABLE_CHANGES,
          })
        }
      })
      .catch(data => {
        return dispatch({
          type: ACTIONS.ENTITY_CREATOR.ERROR,
          msg: data && data.error,
        })
      })

  }
}

export const closeEntityCreatorErrorMessage = () => {
  return {
    type: ACTIONS.ENTITY_CREATOR.CLEAR_ERROR,
  }
}



export const setPrimaryKey = (key) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      primaryKey: key,
      columns: state.columns,
    })
  }
}

export const setColumnType = (key, typeName) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let columns = { ...state.columns }
    columns[key] = { ...columns[key], typeName: typeName }
    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: columns,
      primaryKey: state.primaryKey,
    })
  }
}

export const setColumnName = (key, name) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let columns = { ...state.columns }
    columns[key] = { ...columns[key], name: name }
    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: columns,
      primaryKey: state.primaryKey,
    })
  }
}



export const moveColumnUp = (key) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let newColumns = { ...state.columns }
    let columnKeys = getAllKeys(newColumns)
    columnKeys.sort()

    let columnKeyIndex = columnKeys.indexOf(key)
    if (columnKeyIndex === 0) {
      return
    }

    let temp = newColumns[columnKeys[columnKeyIndex - 1]]
    newColumns[columnKeys[columnKeyIndex - 1]] = newColumns[columnKeys[columnKeyIndex]]
    newColumns[columnKeys[columnKeyIndex]] = temp

    // for the primary keys
    let newPrimaryKey = state.primaryKey
    if (columnKeys[columnKeyIndex] === state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex - 1]
    } else if (columnKeys[columnKeyIndex - 1] === state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex]
    }

    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: newColumns,
      primaryKey: newPrimaryKey,
    })
  }
}

export const moveColumnDown = (key) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let newColumns = { ...state.columns }
    let columnKeys = getAllKeys(newColumns)
    columnKeys.sort()

    let columnKeyIndex = columnKeys.indexOf(key)
    if (columnKeyIndex === columnKeys.length - 1) {
      return
    }

    let temp = newColumns[columnKeys[columnKeyIndex + 1]]
    newColumns[columnKeys[columnKeyIndex + 1]] = newColumns[columnKeys[columnKeyIndex]]
    newColumns[columnKeys[columnKeyIndex]] = temp

    // for the primary keys
    let newPrimaryKey = state.primaryKey
    if (columnKeys[columnKeyIndex] === state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex + 1]
    } else if (columnKeys[columnKeyIndex + 1] === state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex]
    }

    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: newColumns,
      primaryKey: newPrimaryKey
    })
  }
}

export const addColumn = () => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let lastKey = Math.max(...getAllKeys(state.columns))
    let columns = {
      ...state.columns,
      [lastKey+1]: null
    }
    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: columns,
      primaryKey: state.primaryKey,
    })
  }
}

export const removeColumn = (key) => {
  return async (dispatch, getState) => {
    let state = getState().entityCreator
    let columns = { ...state.columns }
    delete columns[key]

    //handle primary key
    let primaryKey = state.primaryKey
    console.log('A: ', key, ' B:', state.primaryKey)
    if (key === this.state.primaryKey) {
      let allKeys = getAllKeys(columns)

      if (allKeys.length === 0) {  //handle remove all case
        columns = { 0: null }
        primaryKey = 0
      } else {
        primaryKey = allKeys[0]
      }
    }

    return dispatch({
      type: ACTIONS.ENTITY_CREATOR.MODIFY_STATE,
      columns: columns,
      primaryKey: primaryKey,
    })
  }
}
