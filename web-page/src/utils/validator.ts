/* eslint-disable no-useless-escape */

import {isArray} from "lodash-es";

export const Reg = {
  digits: /^\d+$/,
  digitsNE: /^-?\d+$/,
  number: /^(?:\d+|\d{1,3}(?:,\d{3})+)(?:\.\d+)?$/,
  numberNE: /^-?(?:\d+|\d{1,3}(?:,\d{3})+)(?:\.\d+)?$/,
  numberDE: /^-?(?:\d+|\d{1,3}(?:\.\d{3})+)(?:,\d+)?$/,
  //最多两位小数
  numberMax1: /^\d+(\.\d)?$/,
  numberMax2: /^\d+(\.\d{1,2})?$/,
  url: /^(https?|ftp):\/\/(((([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(%[\da-f]{2})|[!\$&'\(\)\*\+,;=]|:)*@)?(((\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5])\.(\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5])\.(\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5])\.(\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5]))|((([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|\d|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])))\.)+(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])*([a-z]|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])))\.?)(:\d*)?)(\/((([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(%[\da-f]{2})|[!\$&'\(\)\*\+,;=]|:|@)+(\/(([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(%[\da-f]{2})|[!\$&'\(\)\*\+,;=]|:|@)*)*)?)?(\?((([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(%[\da-f]{2})|[!\$&'\(\)\*\+,;=]|:|@)|[\uE000-\uF8FF]|\/|\?)*)?(\#((([a-z]|\d|-|\.|_|~|[\u00A0-\uD7FF\uF900-\uFDCF\uFDF0-\uFFEF])|(%[\da-f]{2})|[!\$&'\(\)\*\+,;=]|:|@)|\/|\?)*)?$/i,
  signature: /^【.+】$/,
  mobile: /^((13[0-9])|(147)|(15[^4,\\D])|(18[0-9])|(17[0-9])|(16[0-9]))\d{8}$/,
  string: /[_a-zA-Z]/,
  pic: /^[^.]+\.(jpg|png)$/,
  datetime: /^\d{4}-\d{2}-\d{2} ([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/,
  timeNoS: /^([01]\d|2[0-3]):([0-5]\d)$/,
  time: /^([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/,
  pwd: /^(?![0-9]+$)(?![a-zA-Z]+$)[0-9A-Za-z]{8,20}$/,
  QQ: /^[1-9][0-9]{4,9}$/,
  password: /^[a-zA-Z]\w{5,17}$/,
  email: /^(\w)+(\.\w+)*@(\w)+((\.\w{2,3}){1,3})$/,
  // email2: /^(?!\.)[0-9A-Za-z\-_]+(\.[0-9A-Za-z\-_]+)?@(?!_)([^/,*?&.@\s])+(\.([^/,*?&.@\s])+)+(?<!_)$/,
  phone: /^(0?(13[0-9]|15[012356789]|17[013678]|18[0-9]|14[57])[0-9]{8})|(400|800)([0-9\\-]{7,10})|(([0-9]{4}|[0-9]{3})(-| )?)?([0-9]{7,8})((-| |转)*([0-9]{1,4}))?$/,
  phone2: /(\(\d{3,4}\)|\d{3,4}-|\s)?\d{7,14}/,
  shortName: /^[A-Z]{4}$/,
  bankId: /^([1-9]{1})(\d{11}|\d{15}|\d{16}|\d{17}|\d{18})$/

}

const AcceptFileType = {
  file: 'application/msword,application/pdf,application/vnd.ms-excel,text/plain,application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  img: 'image/gif,image/jpg,image/jpeg,image/png'
}

export class RegFunc {
  /**
   * @param {string} path
   * @returns {Boolean}
   */
  static isExternal(path: string) {
    const isExternal = /^(https?:|http?:|mailto:|tel:)/.test(path)
    return isExternal
  }

  static number(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.number.test(value)) {
          callback(new Error(`${key}请输入正数`));
        } else {
          callback();
        }
      }
    }]
  }

  static numberNE(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.number.test(value)) {
          callback(new Error(`${key}请输入数字`));
        } else {
          callback();
        }
      }
    }]
  }

  static integer(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.digits.test(value)) {
          callback(new Error(`${key}请输入正整数`));
        } else {
          callback();
        }
      }
    }]
  }

  static integerNE(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.digitsNE.test(value)) {
          callback(new Error(`${key}请输入整数`));
        } else {
          callback();
        }
      }
    }]
  }

  static require(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!value || (isArray(value) && value.length === 0)) {
          callback(new Error(`${key}不得为空`));
        } else {
          callback();
        }
      }
    }]
  }

  static mobile(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!/^1\d{10}$/.test(value)) {
          callback(new Error(`请输入正确的${key}`));
        } else {
          callback();
        }
      }
    }]
  }

  static mobileOrEmail(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!/^1\d{10}$/.test(value) && !Reg.email.test(value)) {
          callback(new Error(`请输入正确的${key}`));
        } else {
          callback();
        }
      }
    }]
  }

  static reg(key: string, reg: any) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!reg.test(value)) {
          callback(new Error(`请输入正确的${key}`));
        } else {
          callback();
        }
      }
    }]
  }

  static validator(func: any) {
    return [{required: true, trigger: "blur", validator: func}]
  }

  static json(key: string, required = true) {
    return [{
      required, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        try {
          if (value && required) {
            eval('(' + value + ')')
          } else {
            callback()
          }
        } catch (e) {
          callback(new Error(`${key}不是合法JSON字符串`));
        }
        callback();
      }
    }]
  }


  static email(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.email.test(value)) {
          callback(new Error(`请输入正确的${key}`));
        } else {
          callback();
        }
      }
    }]
  }

  static url(key: string) {
    return [{
      required: true, trigger: "blur", validator: (rule: any, value: any, callback: any) => {
        if (!Reg.url.test(value)) {
          callback(new Error(`请输入正确的${key}`));
        } else {
          callback();
        }
      }
    }]
  }
}
