import RpcClient from 'jayson/lib/client/browser/index.js'
import fetch from 'node-fetch';

export class JSONRpc {
  constructor(url) {
    this.client = new RpcClient(async (request, callback) => {
      const options = {
        method: 'POST',
        body: request,
        headers: Object.assign(
          {
            'Content-Type': 'application/json',
          },
        ),
      };
      fetch(url, options)
        .then(function (res) { return res.text(); })
        .then(function (text) { callback(null, text); })
        .catch(function (err) { callback(err); });
    }, {})
  }
  request(method, params) {
    return new Promise((resolve, reject) => {
      console.log('-->', method, params)
      this.client.request(method, params, (error, response) => {
        console.log('<--', error, response)
        if (error) {
          reject(error)
        } else {
          resolve(response)
        }
      })
    })
  }
}

export default {
  JSONRpc
}