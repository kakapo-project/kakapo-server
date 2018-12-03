
import { WEBSOCKET_CONNECT, WEBSOCKET_SEND } from '@giantmachines/redux-websocket'
import { WS_URL } from './config'

export const tableWantsToLoad = (name) => {
  const url = `${WS_URL}/table/${name}`
  return {
    type: WEBSOCKET_CONNECT,
    payload: { url }
  }
}

export const requestingTableData = () => {

  let sendGetTable = {
    action: 'getTable',
  }

  let sendGetTableData = {
    action: 'getTableData',
    begin: 0,
    end: 500,
  }

  return [
    {
      type: WEBSOCKET_SEND,
      payload: sendGetTable,
    },
    {
      type: WEBSOCKET_SEND,
      payload: sendGetTableData,
    },
  ]
}