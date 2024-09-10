import {each} from "lodash-es";

function arrToObj(arr: Array<any>, key = 'val') {
  const obj = Object.create(null)
  each(arr, (item) => {
    obj[item[key]] = item
  })
  return obj
}

export const enum LocalStorageKey {
  user = 'user',
  lastStatus = "lastStatus",
}

export class LastStatus {
  resGroupId: string = ''
  pageSize: Number = import.meta.env.VITE_APP_PAGE_SIZE
  tagAutoRefresh: boolean = true
}

export const enum MODE {
  list,
  edit,
  info,
  add,
  view,
  recommend,
  appointment,
  staff,
  position,
  system,
}

export const CONTENT_TYPE = {
  json: 'application/json',
  form: 'application/x-www-form-urlencoded;',
  html: 'text/html;',
  file: 'multipart/form-data',
  excel: 'msexcel',
  xml: 'application/xml',
  javascript: 'application/x-javascript;charset=utf-8'
}

export const RESPONSE_TYPE = {
  arraybuffer: 'arraybuffer',
  blob: 'blob',
  document: 'document',
  json: 'json',
  text: 'text',
  stream: 'stream',
}
export const REQ_HEADERS = {
  lang: 'Accept-Language',
  contentType: 'Content-Type'
}

export const enum DATE_FORMAT {
  datetime = "YYYY-MM-DD HH:mm:ss",
  dateminutes = "YYYY-MM-DD HH:mm",
  date = "YYYY-MM-DD",
  time = "HH:mm:ss"
}

export const STATUS_MAP = {
  1: "有效",
  0: "无效"
}

export const EXCHANGE = [
  {
    name: '上海交易所',
    val: 'sh',
  }, {
    name: '深圳交易所',
    val: 'sz',
  }, {
    name: '北京交易所',
    val: 'bj',
  },
]

export const EXCHANGE_MAP = arrToObj(EXCHANGE)


export const RECOMMEND = [
  {
    name: '是',
    val: 1,
  }, {
    name: '否',
    val: -1,
  }
]
export const RECOMMEND_MAP = arrToObj(RECOMMEND)
