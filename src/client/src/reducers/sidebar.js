
import { ACTIONS } from '../actions'

const initialState = {
  isOpen: false,
}

const sidebar = (state = initialState, action) => {
  switch (action.type) {
    case ACTIONS.TOGGLE_SIDEBAR:
      return {
        ...state,
        isOpen: !state.isOpen,
      }
    case ACTIONS.OPEN_SIDEBAR:
      return {
        ...state,
        isOpen: true,
      }
    case ACTIONS.CLOSE_SIDEBAR:
      return {
        ...state,
        isOpen: false,
      }
    default:
      return state
  }
}

export default sidebar