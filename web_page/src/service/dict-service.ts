import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {CARELINK_DICT_KEY, CONST_VAR} from "@/views/const";
import {Tools} from "@/utils/tools";

export class DictService extends BaseService {
  constructor() {
    super('/system/', '');
  }

  async getDict(key: string, mask = true, option) {
    if (!CONST_VAR.isDemo) {
      const user = Tools.getUser()
      return HttpClient.get(`${this.apiContext}${!option.type ? 'dict' : 'dict_hash'}${option.user ? '/' + user?.name : ''}/${key}`, {
        mask
      })
    } else {
      let result = "{}"
      if (key !== CARELINK_DICT_KEY.carelinkMyData) {
        result = "{\"lastConduitTime\":\"2024-09-16 20:35:08\"}"
      }
      return new Promise((resolve, reject) => {
        resolve(result)
      })
    }
  }

  updateDict(params: any, option) {
    const user = Tools.getUser()
    return HttpClient.postBody(`${this.apiContext}dict${option.user ? '/' + user?.name : ''}`, {
      body: params
    })
  }

  getDemo() {
    return HttpClient.get(`${location.href}/data.json`, {
      headers: {
        skip: true
      }
    })
  }
}
