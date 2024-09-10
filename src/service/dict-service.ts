import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";

export class DictService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  getDict(key: string, mask = true) {
    return HttpClient.get(`${this.apiContext}dict/${key}`, {
      mask
    })
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
