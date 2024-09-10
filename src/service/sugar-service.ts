import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";

export class SugarService extends BaseService {
  constructor() {
    super('https://localhost:8081/api/public/', '');
  }

  refreshCarelinkData() {
    return HttpClient.put(`${this.apiContext}refreshCarelinkData`, {
      external: true,
      mask: true
    })
  }
}
