import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {CARELINK_DICT_KEY, CONST_VAR} from "@/views/const";

export class DictService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  getDict(key: string, mask = true) {
    if (!CONST_VAR.isDemo && key !== CARELINK_DICT_KEY.carelinkMyData) {
      return HttpClient.get(`${this.apiContext}dict/${key}`, {
        mask
      })
    } else {
      return "{\"lastConduitTime\":\"2024-09-16 20:35:08\"}"
    }
  }

  updateDict(params: any) {
    return HttpClient.postBody(`${this.apiContext}dict`, {
      body: params
    })
  }

  getDemo() {
    return HttpClient.get(`https://localhost:8006/data.json`, {
      headers: {
        skip: true
      }
    })
  }
}