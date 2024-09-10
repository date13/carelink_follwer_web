# carelink_follwer_web
## 初衷
    1.因为使用的是 780 的泵和闭环系统,要查看血糖就要用 carelink,就要有个iPhone(当然 Andorid 也有 carelink,但是在国内无法使用).
    之前自己搭建了一个nightsocut 作为查看的 web 工具,但是经过美敦力服务器几次崩溃和 IT 升级后(加入了 google 验证码)
    好多第三方的和 nightsocut 要想再看到数据就比较困难了
    
    2.后面也对 nightsocut 进过一番改造,勉勉强强能够看到数据,但是最近频繁的无法读取数据,不停的去登录,让我烦不胜烦.
    nightsocut 也基本停止维护了,我想看个数据就这么难吗.
    于是乎,经过一番研究,决定自己写个代码看数据,2 天后就完成了现在的这个 web 程序,进过几天测试下来,几乎不会断数据,现在拿出来分享给大家
    
    3.有一定开发能力的,正好有身边有糖的,又正好用美敦力 carelink 的朋友(好像这样的人也不多)可以自己改改用用

![image](https://github.com/user-attachments/assets/efc9a06b-9055-4301-a4fc-5ec7a3eb032a
)


## 技术栈
    1.因为赶时间,第一版基本上是在原有项目中开发的,所以后面会把代码给分离出来

    2.carelink 的数据抓取会去 carelink 的官方接口获取由于速度问题,会采取定时抓取然后存放到数据库的方式,方便页面查看

    3.后端采用 python,因为简单,采用 fastApi 框架,mysql 数据库,一张表足够了,主打就是一个简单(代码稍后会放到在另外一个项目)

    4.前端采用最流行的 vue3

    5.干了快 20 年 IT ,做了这一个自己的项目

# 如何运行

- 全局安装 pnpm, npm i pnpm -g
- 在项目根目录运行pnpm i
- 然后运行 pnpm run dev
- 访问 https://localhost:8006/
- 模拟数据在 /public/data.json里面
- 实际要用要配合后端服务,晚点把后台项目也放出来

# Vue 3 + TypeScript + Vite

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue
3 `<script setup>` SFCs, check out
the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [WebStorm](https://www.jetbrains.com/webstorm/)
- [VS Code](https://code.visualstudio.com/)
    + [Volar](https://marketplace.visualstudio.com/items?itemName=johnsoncodehk.volar)

- NodeJs 16+
- [包管理采用pnpm](https://cn.vitejs.dev/)

## element样式修改

[element-plus](https://element-plus.gitee.io/zh-CN/)

1. 全局样式变量修改
   修改文件 styles/element/index.scss
   参考变量名 packages/theme-chalk/src/common/var.scss

2. element变量修改以及默认样式覆盖
   修改文件 styles/element/element-plus.scss

3. 自定义css变量在variables.module.scss
4. 自定义全局样式index.scss

## element的按需引入

    不需要添加,自动会生成到components.d.ts里面

## ts模板setup语法糖

    <script setup name="test" lang="ts">
    通过'vite-plugin-vue-setup-extend'加上name,页面才能缓存cache来控制

## 图标采用unplugin-icons

#### https://github.com/antfu/unplugin-icons

#### https://icones.js.org/collection/ep

    预装设置在vite.config.ts 
    enabledCollections('ep'),可以在html中直接使用无需引入
    IconsResolver({
        prefix: false,
        enabledCollections: ['ep'],
    }),
    {prefix}-{collection}-{icon}
    <i-ep-CirclePlus></i-ep-CirclePlus>
    <i-ep-Check></i-ep-Check>
    prefix设为false可以不要前缀
    <ep-CirclePlus></ep-CirclePlus>
    <ep-Check></ep-Check>

## 样式

    unoss+tailwindcss 
    ## vuetify css

    已经完全去除掉vuetify.css的代码,直接使用tailwindcss.style目录下有个tailwindcss文件只是给编辑器语法提示用的,项目中没有使用
    直接使用uno-css + tailwindcss 模板

[tailwindcss](https://www.tailwindcss.cn/docs/text-color)

## 发布优化

    经过chunk分包,nginx打开gzip,图标自动导入,压缩compression后
    首页js加载大小从11m减少到6m一下,打开速度提高一大截
    
    lodash换成了lodash-es,使用lodash的包不要采用
    import _ from 'lodash-es'
    应该使用
    import {forEach} from 'lodash-es'
    减少打包体积

## 开发规范

- 页面统一在views目录下
- 同样service和model目录也是相应划分,系统级别共用的的service和model放到根目录下即可
- 业务模型放入到model/classes目录下,最好每个业务模块采用class的方式,提交数据重写convert方法,获取编辑数据重写parseField方法
- 基本上不用再开发新的css,要用的uno-css都找得到,采用minix方式
- 共用功能采用混入式函数式编程,参考composition下的
