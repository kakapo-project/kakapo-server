
export * from './table'

export const ACTIONS = {
  OPEN_SIDEBAR: 'OPEN_SIDEBAR',
  CLOSE_SIDEBAR: 'CLOSE_SIDEBAR',
  TOGGLE_SIDEBAR: 'TOGGLE_SIDEBAR',

  ADD_ROW: 'ADD_ROW',
  DELETE_ROW: 'DELETE_ROW',
  UPDATE_VALUE: 'UPDATE_VALUE',
}

export const clickToggleSidebar = () => {
  return { type: ACTIONS.TOGGLE_SIDEBAR }
}

export const loadedPage = (page) => {
  switch (page) {
    case 'Home':
      return { type: ACTIONS.OPEN_SIDEBAR }
    default:
      return { type: ACTIONS.CLOSE_SIDEBAR }
  }
}