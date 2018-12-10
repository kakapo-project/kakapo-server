
import { ACTIONS, Selections } from '../actions'

const initialState = {
  selections: [Selections.tables],
}

const home = (state = initialState, action) => {
  switch (action.type) {
    case ACTIONS.SET_ENTITY_SELECTION:
      let newSelections = [...state.selections]
      let selection = action.selection

      if (newSelections.includes(selection)) {
        newSelections = newSelections.filter(x => x !== selection)
      } else {
        newSelections = newSelections.concat([selection])
      }

      return {
        ...state,
        selections: newSelections,
      }
    default:
      return state
  }
}

export default home