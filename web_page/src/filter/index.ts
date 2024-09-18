import {App} from "@vue/runtime-core";
import {DATE_FORMAT, STATUS_MAP} from "@/model/model-type";
import dayjs from "dayjs";

function PadZero(str) {
  //补零
  return new RegExp(/^\d$/g).test(str) ? `0${str}` : str;
}

export default (app: App) => {
  app.config.globalProperties.$filters = {
    date(day, format = DATE_FORMAT.date) {
      if (!day) return '';
      return dayjs(day).format(format);
    },
    dateminuties(day, format = DATE_FORMAT.dateminutes) {
      if (!day) return '';
      return dayjs(day).format(format);
    },
    datetime(day, format = DATE_FORMAT.datetime) {
      if (!day) return '';
      return dayjs(day).format(format);
    },
    status(status) {
      return STATUS_MAP[status];
    },
    duration(_seconds) {
      if (!_seconds) return "";
      _seconds = parseInt(_seconds);
      let hours, mins, seconds;
      let result = '';
      seconds = parseInt(String(_seconds % 60));
      mins = parseInt(String(_seconds % 3600 / 60))
      hours = parseInt(String(_seconds / 3600));

      if (hours)
        result = `${PadZero(hours)}时${PadZero(mins)}分${PadZero(seconds)}秒`
      else if (mins)
        result = `${PadZero(mins)}分${PadZero(seconds)}秒`
      else
        result = `${seconds}秒`
      return result;
    },
    fileSize(size, pointLength = 2, units = ['B', 'K', 'M', 'G', 'TB']) {
      if (!size) return '0B';
      let unit;
      while ((unit = units.shift()) && size > 1024) {
        size = size / 1024;
      }
      return `${unit === 'B' ? size : size.toFixed(pointLength || 2)}${unit}`
    },
    money(value, isFen = false) {
      if (!value) return '0.00'
      if (isFen) value = value / 100
      let intPart = Number(value) - (Number(value) % 1)
      let intPartFormat = intPart
          .toString()
          .replace(/(\d)(?=(?:\d{3})+$)/g, '$1,')
      let floatPart = '.00'
      let value2Array = value.toString().split('.')
      if (value2Array.length == 2) {
        floatPart = value2Array[1].toString()
        if (floatPart.length == 1) {
          return intPartFormat + '.' + floatPart + '0'
        } else {
          return intPartFormat + '.' + floatPart
        }
      } else {
        return intPartFormat + floatPart
      }
    },
  }
}
