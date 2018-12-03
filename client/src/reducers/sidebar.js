
const initialState = {
  isOpen: false,
}

const sidebar = (state = initialState, action) => {
  switch (action.type) {
    case 'TOGGLE_SIDEBAR':
      return {
        ...state,
        isOpen: !state.isOpen,
      }
    case 'OPEN_SIDEBAR':
      return {
        ...state,
        isOpen: true,
      }
    case 'CLOSE_SIDEBAR':
      return {
        ...state,
        isOpen: false,
      }
    default:
      return state
  }
}

export default sidebar