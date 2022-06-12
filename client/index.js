import utils from './utils.js'

const jsonrpc = new utils.JSONRpc('http://127.0.0.1:3030/')

;(async function() {
  await jsonrpc.request('say_hello', [])
  await jsonrpc.request('echo', [])
  await jsonrpc.request('echo', { a: 1, b: 2 })
  await jsonrpc.request('add', [1, 2])
  await jsonrpc.request('addWithDesc', ['addWithDesc', { a: 1, b: 2 }])
  await jsonrpc.request('addWithMarco', [1, 2])
  await jsonrpc.request('addWithMarcoDesc', ['addWithMarcoDesc', { a: 1, b: 2 }])
})()
