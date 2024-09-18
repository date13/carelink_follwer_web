import {hideLoading, Msg, showLoading} from './tools'
import axios, {AxiosResponse, Method, ResponseType} from 'axios'
import qs from 'qs'
import {CONTENT_TYPE, REQ_HEADERS, RESPONSE_TYPE} from "@/model/model-type";

const ENV = import.meta.env
const protocol = location.protocol
export const API_DOMAIN = `${protocol}//${ENV.VITE_APP_API_DOMAIN}`
export const API_URL = `${API_DOMAIN}/${ENV.VITE_APP_API_CONTEXT}/`

// export const API_UPLOAD_URL = `${API_URL}mgr/upload?type=0` + (ENV.DEBUG ? `&${ENV.DEBUG_STR}` : '');
function checkResponse(response: AxiosResponse): Promise<AxiosResponse<any>> {
  hideLoading()
  if ((response as AxiosResponse).headers !== undefined) {
    const contentType = response.headers[REQ_HEADERS.contentType.toLocaleLowerCase()]
    if (response.status === 200) {
      if (contentType && contentType!.indexOf(CONTENT_TYPE.json) !== -1) {
        if (response.config.headers.skip) {
          response.data = JSON.parse(response.request.responseText).data
        } else {
          const result = response.data ? response.data : JSON.parse(response.request.responseText)
          const code = result.code
          if (code !== 0) {
            Msg.alert(result.msg || result.message)
            response.data = false
          } else {
            response.data = result.data || (result.code === '0')
          }
        }
      } else {
        Msg.errorMsg(`错误信息：${response.status} ${response.statusText}`)
        response.data = false
      }
    } else {
      Msg.errorMsg(`错误信息：${response.status} ${response.statusText}`)
      response.data = false
    }
    return Promise.resolve(response.data)
  } else {
    return Promise.resolve(response)
  }
}

// axios.defaults.withCredentials = true
axios.interceptors.response.use(checkResponse, (error) => {
  const status = error?.response?.status
  hideLoading()
  if (status === 422) {
    const data = error?.response.data
    const errMsg = `${data.msg}${JSON.stringify(data.data)}`
    Msg.errorMsg(errMsg)
    return Promise.resolve(false)
  } else if (status === 401) {
    const data = error?.response.data
    Msg.alert(`${data.msg},请重新登录`)
  } else {
    const errMsg = `系统错误：${error.request?.responseURL.replace(API_URL, '')},${error.message ? error.message : `Error:${status}`}`
    Msg.errorMsg(errMsg)
    return Promise.reject(error)
  }
})

export class HttpClient {
  static async call(url: string, {
    module = '',
    method = 'get',
    headers = {},
    body = {},
    mask = true,
    postData = false,
    external = false,
    responseType = RESPONSE_TYPE.json
  } = {}): Promise<any> {
    // let moduleName = prefix != '' ? `${prefix}-${module}` : module;
    url = external ? url : module ? `${module}/${url}` : url
    const params = {
      baseURL: !external ? API_URL : '',
      method: (<Method>method),
      url: url,
      responseType: (<ResponseType>responseType) || 'json',
      headers: headers,
      params: {},
      data: {}
    }
    if (method === 'get') {
      params.params = body
    } else {
      params.data = postData ? body : qs.stringify(body)
    }
    if (mask) {
      showLoading()
    }
    return axios(params)
  }

  static get(url: string, {
    body = {},
    mask = true,
    module = '',
    external = false,
    headers = {},
  } = {}) {
    return HttpClient.call(url, {
      body,
      module,
      headers,
      external,
      mask
    })
  }

  static getHtml(url: string, {
    mask = true,
    module = '',
    external = false,
    headers = {},
  } = {}, responseType = CONTENT_TYPE.html) {
    return HttpClient.call(url, {
      module,
      headers,
      external,
      mask,
      responseType
    })
  }

  static getXml(url: string, {
    mask = true,
    module = '',
    external = false,
    headers = {},
  } = {}, responseType = CONTENT_TYPE.xml) {
    return HttpClient.call(url, {
      module,
      headers,
      external,
      mask,
      responseType
    })
  }

  static getJavascript(url: string, {
    mask = true,
    module = '',
    external = false,
    headers = {},
  } = {}, responseType = CONTENT_TYPE.javascript) {
    return HttpClient.call(url, {
      module,
      headers,
      external,
      mask,
      responseType
    })
  }

  static post(url: string, {
    body = {},
    module = '',
    method = 'post',
    mask = true,
    postData = false,
    external = true,
    headers = {},
  } = {}) {
    return HttpClient.call(url, {
      module,
      method,
      headers: Object.assign(headers, {
        [REQ_HEADERS.contentType]: CONTENT_TYPE.form
      }),
      body,
      external,
      postData,
      mask,
    })
  }

  static put(url: string, {
    body = {},
    module = '',
    mask = true,
    postData = false,
    external = false,
    headers = {},
  } = {}) {
    return HttpClient.post(url, {
      body,
      module,
      method: 'put',
      postData,
      external,
      mask,
      headers
    })
  }

  static destroy(url: string, {
    body = {},
    module = '',
    mask = true,
    postData = false,
    external = false,
    headers = {},
  } = {}) {
    return HttpClient.post(url, {
      body,
      module,
      method: 'delete',
      postData,
      external,
      mask,
      headers,
    })
  }

  static upload(url: string, {
    body = {},
    module = '',
    mask = true,
    postData = true,
    external = false,
    headers = {},
  } = {}) {
    return HttpClient.call(url, {
      module,
      body,
      method: 'post',
      headers: Object.assign(headers, {
        [REQ_HEADERS.contentType]: CONTENT_TYPE.file
      }),
      postData,
      external,
      mask,
    })
  }

  static download(url: string, {
    module = '',
    mask = true,
    external = false,
    headers = {},
    responseType = 'blob',
  } = {}) {
    return HttpClient.call(url, {
      module,
      method: 'get',
      headers,
      external,
      mask,
      responseType,
    })
  }

  static getExcel(url: string, {
    body = {},
    mask = true,
    module = '',
    external = false,
    postData = true,
    headers = {},
    responseType = 'blob',
  } = {}) {
    return HttpClient.call(url, {
      postData,
      body,
      module,
      headers: Object.assign(headers, {
        [REQ_HEADERS.contentType]: CONTENT_TYPE.json
      }),
      method: 'put',
      external,
      mask,
      responseType,
    })
  }

  static postBody(url: string, {
    body = {},
    module = '',
    method = 'post',
    mask = true,
    postData = true,
    external = false,
    headers = {},
  }) {
    return HttpClient.call(url, {
      module,
      method,
      headers: Object.assign(headers, {
        [REQ_HEADERS.contentType]: CONTENT_TYPE.json
      }),
      body: body,
      postData,
      external,
      mask,
    })
  }

  static putBody(url: string, {
    body = {},
    module = '',
    mask = true,
    postData = true,
    external = false,
    headers = {},
  }) {
    return HttpClient.postBody(url, {
      body,
      module,
      method: 'put',
      mask,
      postData,
      external,
      headers,
    })
  }

  static destroyBody(url: string, {
    body = {},
    module = '',
    mask = true,
    postData = true,
    external = false,
    headers = {},
  }) {
    return HttpClient.postBody(url, {
      body,
      module,
      method: 'delete',
      mask,
      postData,
      external,
      headers,
    })
  }
}
