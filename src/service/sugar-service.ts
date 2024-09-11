import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import {DictService} from "@/service/dict-service";

export class SugarService extends BaseService {
  constructor() {
    super('https://stock.baoflag.com/api/public/', '');
  }

  async loadData(isDemo = false, mask) {
    let resultData = null
    const dictService = new DictService()
    if (!isDemo) {
      const result = await dictService.getDict("carelinkData", mask)
      if (result) {
        const data = JSON.parse(result.val)
        resultData = {
          data: JSON.parse(data.data),
          status: data.status
        }
        console.log(resultData);
      }
    } else {
      const result = await dictService.getDemo()
      if (result) {
        resultData = {
          data: result,
          status: 200
        }
      }
    }
    return resultData
  }

  refreshCarelinkData() {
    return HttpClient.put(`${this.apiContext}refreshCarelinkData`, {
      external: true,
      mask: true
    })
  }
}
