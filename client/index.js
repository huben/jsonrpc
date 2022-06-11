import utils from './utils.js'

const jsonrpc = new utils.JSONRpc('http://127.0.0.1:3030/')

;(async function() {
  const result = await jsonrpc.request('say_hello', [])
})()
