<template>
  <el-container
      style="width:100%; height:100%; background-image: url(../../src/assets/images/bg.png);background-size: 100% 100%;">
    <el-header height="60px" style="background-color:#dedfe0;">
      <el-row align="middle" class="mt-1" style="height:60px">
        <img alt="" class="mb-2" src="../../src/assets/images/logo.jpg" width="140">
      </el-row>
    </el-header>
    <el-main>
      <div class="login-container">
        <!--密码登录-->
        <el-form v-if="mode === MODE.info" ref="loginFormRef" :model="loginForm" :rules="loginRules" auto-complete="on"
                 class="login-form" label-position="left">
          <div class="title-container text-center">
            <h1 class="title">Carelink Follower Platform</h1><br/>
          </div>
          <el-form-item prop="name">
            <div class="flex">
              <span class="svg-container">
              <ep-User></ep-User>
            </span>
              <el-input ref="username" v-model="loginForm.name" auto-complete="on" name="name" placeholder="用户名"
                        tabindex="1" type="text"/>
            </div>

          </el-form-item>
          <el-tooltip :disabled="capslockTooltipDisabled" content="Caps lock is On" placement="right">
            <el-form-item prop="password">
              <div class="flex">
                <span class="svg-container">
                  <ep-Lock></ep-Lock>
                </span>
                <el-input :key="passwordType" ref="passwordRef" v-model="loginForm.password" :type="passwordType"
                          auto-complete="on" name="password" placeholder="Password" tabindex="2"
                          @blur="capslockTooltipDisabled = true" @keyup="checkCapslock" @keyup.enter="handleLogin"/>
                <span class="show-pwd" @click="showPwd">
                  <ep-View v-if="passwordType !== 'password'"></ep-View>
                  <ep-Hide v-else></ep-Hide>
                </span>
              </div>
            </el-form-item>
          </el-tooltip>
          <el-button :loading="loading" class="w-full mb-2" size="large" type="primary"
                     @click.prevent="handleLogin">登&nbsp;&nbsp;&nbsp;&nbsp;录
          </el-button>
        </el-form>
      </div>
    </el-main>
    <el-footer height="60px" style="background-color:#dedfe0;font-size: 14px;">
      <el-row align="middle" justify="center" style="height:60px">
        date13 版权所有
      </el-row>
    </el-footer>
  </el-container>
</template>

<script lang="ts" setup>
// 组件依赖
import {ElForm, ElInput} from "element-plus";

// 状态管理依赖
import {useRoute} from "vue-router";
import {Tools} from "@/utils/tools";
import SparkMD5 from "spark-md5";
import router from "@/router";
import {RegFunc} from "@/utils/validator";
import {MODE} from "@/model/model-type";
import User from "@/model/classes/User";
import {UserService} from "@/service/user-service";

const route = useRoute();
const loginFormRef = ref(ElForm);
const service = new UserService()
const passwordRef = ref(ElInput);
const state: any = reactive({
  redirect: "",
  mode: MODE.info,
  loginForm: {
    name: "",
    password: "",
  },
  passwordType: "password",
  loginRules: {
    name: RegFunc.require('用户名'),
    password: RegFunc.validator(validatePassword),
  },
  loading: false,
  // 大写提示禁用
  capslockTooltipDisabled: true,
  otherQuery: {},
  clientHeight: document.documentElement.clientHeight,
  showCopyright: true,
});

function validatePassword(rule: any, value: any, callback: any) {
  if (value.length < 6) {
    callback(new Error("密码不得小于6位"));
  } else {
    callback();
  }
}

const {
  loginForm,
  loginRules,
  loading,
  passwordType,
  mode,
  capslockTooltipDisabled,
} = toRefs(state);

function checkCapslock(e: any) {
  const {key} = e;
  state.capslockTooltipDisabled =
      key && key.length === 1 && key >= "A" && key <= "Z";
}

function showPwd() {
  if (state.passwordType === "password") {
    state.passwordType = "";
  } else {
    state.passwordType = "password";
  }
  nextTick(() => {
    passwordRef.value.focus();
  });
}

function handleLogin() {
  loginFormRef.value.validate(async (valid: boolean) => {
    if (valid) {
      state.loading = true;
      const result = await service.login({
        name: state.loginForm.name,
        password: SparkMD5.hash(state.loginForm.password),
      });
      if (result) {
        dealLogin(result)
      }
      state.loading = false;
    }
  });
}

function dealLogin(result) {
  const user = new User(result)
  Tools.setUser(user)
  router.push({path: state.redirect || "/", query: state.otherQuery});
}


watch(
    route,
    () => {
      const query = route.query;
      if (query) {
        state.redirect = query.redirect as string;
        state.otherQuery = getOtherQuery(query);
      }
    },
    {
      immediate: true,
    }
);

function getOtherQuery(query: any) {
  return Object.keys(query).reduce((acc: any, cur: any) => {
    if (cur !== "redirect") {
      acc[cur] = query[cur];
    }
    return acc;
  }, {});
}

onMounted(() => {
  window.onresize = () => {
    state.showCopyright = state.clientHeight <= document.documentElement.clientHeight;
  };
});
</script>

<style lang="scss">
/* 修复input 背景不协调 和光标变色 */
/* Detail see https://github.com/PanJiaChen/vue-element-admin/pull/927 */

$bg: #283443;
$light_gray: #000;
$cursor: #000;

/* reset element-ui css */
.login-container {

  .el-input {
    display: inline-block;
    height: 47px;
    width: 85%;
    font-size: 14px;

    .el-input__wrapper {
      padding: 0;
      background: transparent;
      box-shadow: none;

      .el-input__inner {
        background: transparent;
        border: 0;
        -webkit-appearance: none;
        border-radius: 0;
        padding: 12px 5px 12px 15px;
        color: $light_gray;
        height: 47px;
        caret-color: $cursor;

        &:-webkit-autofill {
          box-shadow: 0 0 0 1000px #e5e5e5 inset !important;
          -webkit-text-fill-color: $cursor !important;
        }
      }
    }

  }

  .el-input__inner {
    box-shadow: none;

    &:hover {
      border-color: var(--el-input-hover-border, var(--el-border-color-hover));
      box-shadow: none;
    }
  }

  .el-form-item {
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(47, 47, 47, 0.1);
    border-radius: 0px;
    color: #454545;
  }

  .copyright {
    width: 100%;
    position: absolute;
    bottom: 0;
    font-size: 12px;
    text-align: center;
    color: #cccccc;
  }
}
</style>

<style lang="scss" scoped>
$bg: #f8f8f8;
$dark_gray: #5f8093;
$light_gray: #eee;

.login-container {

  .login-form {
    position: relative;
    width: 420px;
    max-width: 100%;
    padding: 100px 35px 0;
    margin: 0 auto;
    overflow: hidden;
  }

  .tips {
    font-size: 14px;
    color: #fff;
    margin-bottom: 10px;

    span {
      &:first-of-type {
        margin-right: 16px;
      }
    }
  }

  .svg-container {
    padding: 6px 5px 6px 15px;
    color: $dark_gray;
    vertical-align: middle;
    width: 40px;
    display: inline-block;
  }

  .title-container {
    position: relative;

    .title {
      color: black;
      margin: 0 auto 20px auto;
      text-align: center;
      font-weight: bold;
      font-family: '微软雅黑';
    }
  }

  .show-pwd {
    position: absolute;
    right: 10px;
    top: 7px;
    font-size: 16px;
    color: $dark_gray;
    cursor: pointer;
    user-select: none;
  }

  .register-form {
    position: relative;
    width: 420px;
    max-width: 100%;
    padding: 100px 35px 0;
    margin: 0 auto;
    overflow: hidden;
  }

  .captcha {
    img {
      height: 47px;
      cursor: pointer;
      vertical-align: top;
    }
  }
}
</style>
