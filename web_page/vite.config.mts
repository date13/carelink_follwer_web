import {defineConfig, loadEnv} from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import VueSetupExtend from 'vite-plugin-vue-setup-extend'
import {ElementPlusResolver} from 'unplugin-vue-components/resolvers'
import {resolve} from 'path'
import {createSvgIconsPlugin} from 'vite-plugin-svg-icons'
import basicSsl from '@vitejs/plugin-basic-ssl'
import Unocss from 'unocss/vite'
import {presetAttributify, presetUno} from 'unocss'
import {compression} from 'vite-plugin-compression2'
import {visualizer} from "rollup-plugin-visualizer";
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'


/*// @ts-ignore
import dns from 'dns'

dns.setDefaultResultOrder('verbatim')*/

// https://vitejs.dev/config/
// @ts-ignore
export default defineConfig(({mode}) => {
  // Load env file based on `mode` in the current working directory
  const env = loadEnv(mode, process.cwd())
  console.log("mode:", mode);
  console.log("cur env:", env);
  const isProd = mode === "production"
  const isHttps = env.VITE_APP_HTTPS === 'true'
  return {
    // build specific config
    resolve: {
      // Vite路径别名配置
      alias: {
        "@": resolve(__dirname, "src"),  // @代替src
      },
    },
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `@use "@/styles/element/index.scss" as *;`,
          silenceDeprecations: ["mixed-decls", 'legacy-js-api', 'color-functions']
        },
      },
    },
    base: env.VITE_APP_DOMAIN_CONTEXT,
    mode,
    // 本地反向代理解决浏览器跨域限制
    server: {
      host: '0.0.0.0',
      port: Number(env.VITE_APP_PORT),
      https: isHttps,
      open: false, // 运行自动打开浏览器
      watch: {
        ignored: ['**/auto-imports.d.ts', '**/components.d.ts', '**/__uno.css'],
      },
    },
    esbuild: {
      pure: ['console.log'],
      drop: ['debugger'],
      minify: true,
    },
    build: {
      /* minify: 'terser',
       terserOptions: {
         compress: {
           //生产环境时移除console
           drop_console: true,
           drop_debugger: true,
         },
       },*/
      minify: true,
      chunkSizeWarningLimit: 1500,
      reportCompressedSize: false,
      //   关闭生成map文件 可以达到缩小打包体积
      sourcemap: false,
      rollupOptions: {
        output: {
          /*manualChunks: {
            vue: ['vue', 'vue-router', 'pinia'],
            echarts: ['echarts'],
            codemirror: ['codemirror', 'vue-codemirror', '@codemirror/lang-javascript', '@codemirror/lang-json', '@codemirror/state'],
            wangeditor: ['@wangeditor/editor', '@wangeditor/editor-for-vue']
          },*/
          manualChunks(id) {
            if (id.includes('node_modules')) {
              return id.toString()?.split('node_modules/')[2]?.split('/')[0].toString();
            }
          }
        }
      }
    },
    plugins: [
      isHttps ? basicSsl() : {},
      compression(),
      visualizer({
        open: true,
        gzipSize: true,
        brotliSize: true,
        emitFile: true,
        filename: "stats.html",
      }), // 自动开启分析页面
      vue(),
      VueSetupExtend(),
      Unocss({
        presets: [
          presetAttributify({ /* preset options */}),
          presetUno(),
        ]
      }),
      createSvgIconsPlugin({
        // 指定需要缓存的图标文件夹
        iconDirs: [resolve(process.cwd(), 'src/assets/icons')],
        // 指定symbolId格式
        symbolId: 'icon-[dir]-[name]'
      }),
      AutoImport({
        resolvers: [
          ElementPlusResolver(),
          IconsResolver({
            prefix: 'Icon',
          }),
        ],
        dts: true,
        vueTemplate: true,
        eslintrc: {
          enabled: true, // <-- this
        },
        imports: ['vue', 'vue-router', 'pinia'],
      }),
      Components({
        dts: true,
        dirs: ['src'],
        directoryAsNamespace: true,
        resolvers: [
          ElementPlusResolver({
            importStyle: "sass",
          }),
          IconsResolver({
            prefix: false,
            enabledCollections: ['ep'],
          }),
        ],
      }),
      Icons({
        autoInstall: true,
      }),
    ]
  }
})
