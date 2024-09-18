import {HttpClient} from '@/utils/http-client'

export class BaseService {
  apiContext: string
  module: string

  constructor(apiContext = '', module = '') {
    this.module = module
    this.apiContext = apiContext
  }

  get(id: string, params = {}) {
    return HttpClient.get(`${this.apiContext}id/${id}`, params)
  }

  load(id: string, params = {}) {
    return HttpClient.get(`${this.apiContext}?id=${id}`, params)
  }

  updateWithId(params: any = {}) {
    return HttpClient[params.id ? 'putBody' : 'postBody'](`${this.apiContext}${params.id ? `id/${params.id}` : ''}`, {
      body: params
    })
  }

  update(params: any = {}) {
    return HttpClient[params.id ? 'putBody' : 'postBody'](`${this.apiContext}`, {
      body: params
    })
  }

  updatePost(params: any = {}) {
    return HttpClient.postBody(`${this.apiContext}`, {
      body: params
    })
  }

  getModulesName(context = this.apiContext) {
    if (context.endsWith("y/")) {
      return context.replace('y/', 'ies')
    } else if (context.endsWith("sx/") || context.endsWith("sh/") || context.endsWith("ch/")) {
      return context.replace('/', 'es')
    } else if (context.endsWith("an/")) {
      return context.replace('an/', 'en')
    }
    return context.replace('/', 's')
  }

  list(params: any = {}) {
    return HttpClient.putBody(`${this.module}${this.getModulesName()}`, {
      body: params
    })
  }

  listGet(params: any = {}) {
    return HttpClient.get(`${this.getModulesName()}`, params)
  }

  remove(id: string, params: any = {}) {
    // params.module = this.module
    return HttpClient.destroy(`${this.apiContext}id/${id}`, params)
  }

  removeByIds(ids) {
    return HttpClient.destroyBody(`${this.getModulesName()}`, {
      body: {
        ids
      }
    })
  }

  removeBody(params: any = {}) {
    return HttpClient.destroyBody(`${this.apiContext}`, {
      body: params
    })
  }
}
