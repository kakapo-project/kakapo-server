

import { ACTIONS } from './index'

import { API_URL } from './config'
import { DEFAULT_TYPE, ALL_TYPES } from './columns'

export const setScriptName = (name) => {
  return {
    type: ACTIONS.ENTITY_CREATOR.SET_SCRIPT_NAME,
    name: name,
  }
}
