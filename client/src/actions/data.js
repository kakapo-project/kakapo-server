
import { ACTIONS } from './index'

import { API_URL } from './config'

export const clearPullDataError = () => {
  return {
    type: ACTIONS.CLEAR_PULL_DATA_ERROR
  }
}

export const pullData = () => {
  return async (dispatch, getState) => {
    //tables
    fetch(`${API_URL}/manage/table`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      console.log('received data: ', data)
      let entities = data.map(x => ({
        name: x.name,
        type: 'table',
        icon: 'database',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      dispatch([
        {
          type: ACTIONS.SET_TABLE_DATA,
          entities: entities,
        },
        {
          type: ACTIONS.ENTITY_CREATOR.CLEAR_DIRTY_ENTITIES,
        }
      ])
    })
    .catch(err => {
      console.log('err: ', err.message)
      dispatch({
        type: ACTIONS.PULL_DATA_ERROR,
        msg: err.message
      })
    })

    //queries
    fetch(`${API_URL}/manage/query`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      let entities = data.map(x => ({
        name: x.name,
        type: 'query',
        icon: 'search',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      dispatch([
        {
          type: ACTIONS.SET_QUERY_DATA,
          entities: entities,
        },
        {
          type: ACTIONS.ENTITY_CREATOR.CLEAR_DIRTY_ENTITIES,
        }
      ])
    })
    .catch(err => {
      console.log('err: ', err.message)
      dispatch({
        type: ACTIONS.PULL_DATA_ERROR,
        msg: err.message
      })
    })

    //scripts
    fetch(`${API_URL}/manage/script`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      let entities = data.map(x => ({
        name: x.name,
        type: 'script',
        icon: 'code',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      dispatch([
        {
          type: ACTIONS.SET_SCRIPT_DATA,
          entities: entities,
        },
        {
          type: ACTIONS.ENTITY_CREATOR.CLEAR_DIRTY_ENTITIES,
        }
      ])
    })
    .catch(err => {
      console.log('err: ', err.message)
      dispatch({
        type: ACTIONS.PULL_DATA_ERROR,
        msg: err.message
      })
    })
  }
}