
import { createStore, applyMiddleware } from 'redux'
import thunk from 'redux-thunk'
import websocket from '@giantmachines/redux-websocket'
import multi from 'redux-multi'


import rootReducer from './reducers'

// Note: this API requires redux@>=3.1.0
const store = createStore(
  rootReducer,
  applyMiddleware(
    websocket,
    thunk,
    multi,
  )
)

export default store