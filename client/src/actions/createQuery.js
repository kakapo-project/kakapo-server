

import { ACTIONS } from './index'

import { API_URL } from './config'
import { DEFAULT_TYPE, ALL_TYPES } from './columns'

export const setQueryName = (name) => {
  return {
    type: ACTIONS.ENTITY_CREATOR.SET_QUERY_NAME,
    name: name,
  }
}
