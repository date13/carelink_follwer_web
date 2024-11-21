import {each, keys} from 'lodash-es'
import {DATE_FORMAT, LastStatus, LocalStorageKey,} from '@/model/model-type'
import dayjs, {Dayjs} from 'dayjs'
import {ElLoading, ElMessage, ElMessageBox, ElNotification} from "element-plus";
import {RemovableRef, useStorage} from "@vueuse/core";
import User from "@/model/classes/User";
import {localStorage} from './storage'

let loadingInstance: any | null
let loadingCount = 0

export const isProd = import.meta.env.VITE_APP_NODE_ENV === 'production'

export function showLoading(msg?: string | undefined) {
  if (loadingCount == 0) {
    loadingInstance = ElLoading.service({
      lock: true,
      text: msg ? msg : '数据加载中',
      fullscreen: true,
      background: '#ffffff33'
    })
  }
  loadingCount++
}

export function hideLoading() {
  if (loadingInstance) {
    if (loadingCount > 0) {
      loadingCount--
      if (loadingCount === 0) {
        loadingInstance.close()
        //loadingcount为0了.强制close掉,去掉loading框,去掉body的class和loadNumer
        const el = document.querySelector("div.el-loading-mask.is-fullscreen")
        if (el) {
          el.remove()
        }
        const body = document.body
        const loadNumber = body.getAttribute('loading-number')
        if (loadNumber) {
          const number = parseInt(loadNumber)
          if (number > 0) {
            Tools.removeClass(document.body, 'el-loading-parent--relative')
            body.removeAttribute('loading-number')
          }
        }
      }
    }
  }
}


export class Msg {
  static alert(msg: string, func = () => {
  }, title = '') {
    ElMessageBox.close()
    ElMessageBox.alert(msg, title, {
      confirmButtonText: '确定',
      callback: () => {
        func()
      }
    })
  }

  static confirm(msg: string, sureFunc: any = () => {
  }, cancelFunc: any = () => {
  }, title = '') {
    ElMessageBox.confirm(msg, title, {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }).then(() => {
      sureFunc()
    }).catch(() => {
      cancelFunc()
    })
  }

  static prompt(msg: string, sureFunc: any = () => {
  }, cancelFunc: any = () => {
  }, defaultStr = '', errMsg = '输入值不得为空', title = '') {
    ElMessageBox.prompt(msg, title, {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      inputValue: defaultStr,
      inputPattern: /^.+$/,
      inputErrorMessage: errMsg,
      type: 'warning'
    }).then(({value}) => {
      sureFunc(value)
    }).catch(() => {
      cancelFunc()
    })
  }

  static success(message: string, title: string = '成功') {
    ElNotification.success({
      title,
      message
    })
  }

  static warn(message: string, title: string = '警告') {
    ElNotification.warning({
      title,
      message
    })
  }

  static info(message: string, title: string = '提示') {
    ElNotification.info({
      title,
      message
    })
  }

  static error(message: string, title: string = '错误') {
    ElNotification.error({
      title,
      message
    })
  }

  static infoMsg(message: string) {
    ElMessage.info(message)
  }

  static successMsg(message: string) {
    ElMessage.success(message)
  }

  static warnMsg(message: string) {
    ElMessage.warning(message)
  }

  static errorMsg(message: string) {
    ElMessage.error(message)
  }
}

const assets = import.meta.glob('/src/assets/**/**.*');
let id$2 = 0;

export class Tools {
  static setUser(user: User | null) {
    localStorage.set(LocalStorageKey.user, user)
  }

  static getUser() {
    return localStorage.get(LocalStorageKey.user)
  }
  static maskReplace(str, obj) {
    for (let key in obj) {
      str = str.replace(new RegExp('\\{' + key + '\\}', 'g'), obj[key])
    }
    return str
  }

  static getFirstPath(item, arr) {
    if (!item) return
    arr.push(item.path)
    if (item.children && item.children.length > 0) {
      //获取第一个有权限的路由
      let firstAutho = item.children.find(robj => robj.auth.show);
      Tools.getFirstPath(firstAutho, arr)
    }
    return arr
  }

  static jsonToUrlParam(json) {
    return new URLSearchParams(json).toString();
  }

  static urlParamToJson(url) {
    const index = url.indexOf('?')
    if (!url || index === -1) {
      return null;
    }

    let json = {};
    url.substring(index + 1)
        .trim()
        .split('&')
        .forEach(item => json[item.split('=')[0]] = item.split('=')[1]);

    return json;
  }

  static startTime(day: Dayjs | number) {
    return dayjs(day).hour(0).minute(0).second(0).millisecond(0)
  }

  static endTime(day: Dayjs | number) {
    return dayjs(day).hour(23).minute(59).second(59).millisecond(0)
  }


  static getTimeUUID() {
    return dayjs().format(DATE_FORMAT.datetime)
  }

  static uniqueId() {
    return Math.random().toString(36).substr(3, 3) + Number("".concat(Date.now().toString()).concat((++id$2).toString())).toString(36);
  }

  static removeChildNode(list, childProp) {
    list.forEach((item) => {
      let childs = item[childProp];
      if (childs) {
        if (childs.length <= 0) {
          delete item[childProp]; //去掉二级的childs属性
        } else {
          this.removeChildNode(childs, childProp);
        }
      }
    });
  }

  static debounce(func: any, wait: number, immediate = false) {
    let timeout: any

    return function executedFunction() {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      // eslint-disable-next-line @typescript-eslint/no-this-alias
      const context = this
      // eslint-disable-next-line prefer-rest-params
      const args = arguments

      const later = function () {
        timeout = undefined
        if (!immediate) func.apply(context, args)
      }

      const callNow = immediate && !timeout

      clearTimeout(timeout)

      timeout = setTimeout(later, wait)

      if (callNow) func.apply(context, args)
    }
  }

  static hasClass(ele: Element, cls: string) {
    return !!ele.className.match(new RegExp('(\\s|^)' + cls + '(\\s|$)'))
  }

  /**
   * Add class to element
   * @param {HTMLElement} elm
   * @param {string} cls
   */
  static addClass(ele: Element, cls: string) {
    if (!Tools.hasClass(ele, cls)) ele.className += ' ' + cls
  }

  /**
   * Remove class from element
   * @param {HTMLElement} elm
   * @param {string} cls
   */
  static removeClass(ele: Element, cls: string) {
    if (Tools.hasClass(ele, cls)) {
      const reg = new RegExp('(\\s|^)' + cls + '(\\s|$)')
      ele.className = ele.className.replace(reg, ' ')
    }
  }

  static importAll(r: any, func = (v: any) => {
  }) {
    r.keys().forEach((key: string) => {
      const mod = r(key)
      func(mod.__esModule && mod.default ? mod.default : mod)
    })
  }

  static lazyRender(func: any, delay = 100) {
    setTimeout(func, delay)
  }

  static getPageCount(totalCount: number, pageSize = 10) {
    let pageCount = 0
    if (totalCount === 0) {
      pageCount = 0
    } else {
      if (totalCount < pageSize) {
        pageCount = 1
      } else {
        pageCount = Math.floor(totalCount / pageSize)
        //console.log(pageCount);
        if (totalCount % pageSize !== 0) {
          pageCount++
        }
      }
    }
    return pageCount
  }

  static removeItem(list: Array<any>, targetKey: any, key = 'id') {
    const index = list.findIndex((item) => {
      return targetKey === item[key]
    })
    list.splice(index, 1)
  }

  static download(url: string) {
    let a = document.createElement('a')
    let event = new MouseEvent('click')
    a.href = url
    a.target = "_blank"
    a.dispatchEvent(event)
    a.remove()
  }

  static changeStatus(list: Array<any>, targetKey: string, status: any, key = 'id') {
    const index = list.findIndex((item) => {
      return targetKey === item[key]
    })
    list[index].status = status
  }

  static strMapToObj(strMap: any) {
    const obj = Object.create(null)
    for (const [k, v] of strMap) {
      obj[k] = v
    }
    return obj
  }

  static objToStrMap(obj: any) {
    const strMap = new Map()
    for (const k of keys(obj)) {
      if (obj[k]) strMap.set(k, obj[k])
    }
    return strMap
  }

  static objToArr(obj: any) {
    const arr: any[] = []
    for (const k of keys(obj)) {
      if (obj[k]) {
        arr.push({
          key: k,
          val: obj[k]
        })
      }
    }
    return arr
  }

  static objToArrFlat(obj: any) {
    return Object.entries(obj).map(([key, val]) => ({key, val}))
  }

  static arrToObj(arr: Array<any>, key = 'val') {
    const obj = Object.create(null)
    each(arr, (item) => {
      obj[item[key]] = item
    })
    return obj
  }

  static flatArray(list: Array<any>, key: string, resArr: Array<any> | null = null) {
    if (!resArr) {
      resArr = []
    }
    for (const item of list) {
      resArr.push(item)
      if (item[key] && item[key].length > 0) {
        Tools.flatArray(item[key], key, resArr)
      }
    }
    return resArr
  }

  static getLastStatus(defaultKey: string = '', defaultValue: any = null): RemovableRef<LastStatus> {
    const lastStatus = useStorage(LocalStorageKey.lastStatus, new LastStatus())
    if (defaultKey && lastStatus.value[defaultKey] === undefined) {
      (lastStatus.value[defaultKey] as any) = defaultValue
    }
    return lastStatus
  }

  static getCurrentTimeRange(timeType: number) {
    let start, end
    if (timeType === 0) { //当前时间区间为前一天的21:00到今天的15:00, 如果是周一的话,向前推3天,到上周五 ALGOQI-629
      start = dayjs().subtract(dayjs().day() === 1 ? 3 : 1, 'day').hour(20).minute(0).second(0)
      end = dayjs().hour(23).minute(59).second(59)
    } else if (timeType === 1) {
      start = dayjs().subtract(6, 'day').hour(0).minute(0).second(0)
      end = dayjs().hour(23).minute(59).second(59)
    } else if (timeType === 2) {
      start = dayjs().subtract(1, 'month').hour(0).minute(0).second(0)
      end = dayjs().hour(23).minute(59).second(59)
    }

    return {
      start: start?.format(DATE_FORMAT.datetime),
      end: end?.format(DATE_FORMAT.datetime)
    }
  }

  static props<T extends Record<any, any>, K extends string>(obj: T, key: K) {
    return obj[key]
  }

  // 下划线转换驼峰
  static toHump(name) {
    return name.replace(/\_(\w)/g, function (all, letter) {
      return letter.toUpperCase();
    });
  }

// 驼峰转换下划线
  static toLine(name) {
    return name.replace(/([A-Z])/g, "_$1").toLowerCase();
  }
}

export const Base64 = {
  encode(str) {
    // first we use encodeURIComponent to get percent-encoded UTF-8,
    // then we convert the percent encodings into raw bytes which
    // can be fed into btoa.
    return btoa(encodeURIComponent(str).replace(/%([0-9A-F]{2})/g,
        function toSolidBytes(match, p1) {
          return String.fromCharCode(Number('0x' + p1));
        }));
  },
  decode(str) {
    // console.log(str);
    // Going backwards: from bytestream, to percent-encoding, to original string.
    return decodeURIComponent(atob(str).split('').map(function (c) {
      return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));
  }
};
