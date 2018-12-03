
export * from './table'

export const clickToggleSidebar = () => {
  return { type: 'TOGGLE_SIDEBAR' }
}

export const loadedPage = (page) => {
  switch (page) {
    case 'Home':
      return { type: 'OPEN_SIDEBAR' }
    default:
      return { type: 'CLOSE_SIDEBAR' }
  }
}