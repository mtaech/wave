import zh from './langs/zh.js'
import zhLocale from 'element-plus/lib/locale/lang/zh-cn'
import {createI18n} from "vue3-i18n";

const i18n = createI18n({
    locale: 'zh',
    message:{ ...zhLocale, ...zh }
});

export default i18n;