import {BaseService} from './base-service'
import {HttpClient} from "@/utils/http-client";
import qs from "qs";

export class TestService extends BaseService {
  constructor() {
    super('/public/', '');
  }

  //手机密码登录
  testHelloWorld() {
    return HttpClient.get(`${this.apiContext}test`, {});
  }

  luck() {
    return HttpClient.get(`${this.apiContext}luck`, {})
  }

  luckPush(params) {
    return HttpClient.postBody(`${this.apiContext}luck_push`, {
      body: params
    })
  }

  getUser(id) {
    return HttpClient.get(`${this.apiContext}user/${id}`, {})
  }

  getUserById(params) {
    return HttpClient.get(`${this.apiContext}get_user_by_id?${qs.stringify(params)}`, {})
  }

  getUserByIdFromUser(params) {
    return HttpClient.get(`/test/get_user_info?${qs.stringify(params)}`, {})
  }

  getInfoFromRedis() {
    return HttpClient.get(`/test/get_json_from_redis`, {})
  }

  getInfoHashFromRedis() {
    return HttpClient.get(`/test/get_json_hash_from_redis`, {})
  }

  set_json_handler(params) {
    return HttpClient.postBody(`/test/set_json_handler`, {
      body: params
    })
  }

  set_hash_handler(params) {
    return HttpClient.postBody(`/test/set_hash_handler`, {
      body: params
    })
  }

  del_hash_handler(params) {
    return HttpClient.postBody(`/test/del_hash_handler`, {
      body: params
    })
  }

  createUser(params) {
    return HttpClient.postBody(`${this.apiContext}create_user`, {
      body: params
    })
  }

  login(params) {
    return HttpClient.postBody(`/user/login`, {
      body: params
    })
  }

  getUserSetting() {
    return HttpClient.get(`/user/get_user_setting`, {})
  }

  test() {
    return HttpClient.get(`/sugar/test`, {});
  }

  loadTask() {
    return HttpClient.get(`/system/tasks`, {});
  }

  stopTask(taskId) {
    return HttpClient.get(`/system/task/${taskId}/stop`, {})
  }

  startTask(taskId) {
    return HttpClient.get(`/system/task/${taskId}/start`, {})
  }
}
