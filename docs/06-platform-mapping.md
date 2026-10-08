# 06 · 各端映射

## 1. 总表

| 端 | 产物 | 引入方式 | 关键注意点 |
|---|---|---|---|
| Web / CSS | `gen/css/danqing.css` | `<link>` 引入，`<html data-theme data-brand>` | 选择器顺序有讲究，见 §2 |
| Sass | `gen/scss/_danqing.scss` | `@use "danqing"` | 输出 CSS 变量 + 嵌套 map + `dq-color()` |
| TypeScript | `gen/ts/danqing.ts` | `import { light, brands } from …` | 字面量常量，`as const`，可用于 canvas / 邮件模板等无法用 CSS 变量的场景 |
| DTCG | `gen/json/danqing.tokens.json` | Style Dictionary / Figma Tokens / Tokens Studio | W3C DTCG 格式，`$value` / `$type` / `$description` |
| Tailwind | `gen/tailwind/danqing.preset.cjs` | `presets: [require(…)]` | 颜色指向 CSS 变量，主题切换仍由 `data-theme` 控制 |
| Avalonia | `gen/avalonia/Tokens.axaml` | `ResourceInclude` 合并 | 语义刷子放 `ThemeDictionaries` 的 Light/Dark |
| SwiftUI | `gen/swift/DesignTokens.swift` | 加入 target | 建议同时导出 `Assets.xcassets` 颜色集以获自动 Appearance |
| Jetpack Compose | `gen/compose/DesignTokens.kt` | 复制到源码树 | 用其构造自定义 `ColorScheme`，**不要**用动态取色 |
| Flutter | `gen/flutter/design_tokens.dart` | 复制到源码树 | 常量类，无 `ThemeData` 依赖 |
| Android XML | `gen/android/colors.xml` + `dimens.xml` | `res/values` | 浅色名 `dq_*`，深色名 `dq_*_dark`，放 `values-night` |

## 2. Web

```html
<!DOCTYPE html>
<html lang="zh-CN" data-theme="light" data-brand="qing">
<head>
  <meta name="color-scheme" content="light dark">
  <link rel="stylesheet" href="/gen/css/danqing.css">
  <style>
    body { background: var(--dq-bg-base); color: var(--dq-text-primary);
           font-family: var(--dq-family-ui); font-size: var(--dq-font-body);
           line-height: var(--dq-lh-body); }
    .card { background: var(--dq-surface-default); border: var(--dq-bw-hairline) solid var(--dq-border-default);
            border-radius: var(--dq-radius-lg); box-shadow: var(--dq-elev-1); padding: var(--dq-space-4); }
    .btn { background: var(--dq-primary-default); color: var(--dq-primary-on);   /* ← on 可能是墨色，别写 #fff */
           height: var(--dq-touch-min); border-radius: var(--dq-radius-sm);
           transition: background var(--dq-dur-fast) var(--dq-ease-standard); }
    .btn:hover { background: var(--dq-primary-hover); }
    .btn:active { background: var(--dq-primary-active); }
  </style>
</head>
```

### 2.1 主题与品牌切换

```js
const root = document.documentElement;
root.dataset.theme = matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
root.dataset.brand = localStorage.getItem("brand") ?? "qing";
matchMedia("(prefers-color-scheme: dark)").addEventListener("change", e => {
  if (!localStorage.getItem("theme")) root.dataset.theme = e.matches ? "dark" : "light";
});
```

**约定**：`data-theme` 只允许 `light` / `dark`；`data-brand` 取值为品牌 id。
不要用 `class="dark"` 这类自定义开关，也不要让 `prefers-color-scheme` 直接参与样式计算。

### 2.2 生成的 CSS 结构（勿改顺序）

```
:root                               浅色基座 + 默认品牌
[data-brand="zhu"] { … }            浅色品牌覆盖（0,1,0）
[data-theme="dark"] { … }           深色基座（0,1,0，必须在浅色品牌之后）
[data-theme="dark"][data-brand="zhu"] { … }   深色品牌覆盖（0,2,0）
@media (prefers-reduced-motion)     动效归零
```

原因见 [`04-theming.md`](./04-theming.md#21-主题切换的实现约定web)。

## 3. TypeScript

```ts
import { cn, light, dark, brands, space, radius, duration, easing, BrandId } from "./gen/ts/danqing";

const canvas = document.querySelector("canvas")!;
const ctx = canvas.getContext("2d")!;
ctx.fillStyle = light.chart.c1;                     // 字面量，可用于 canvas
ctx.fillStyle = brands.qing.light.primary.default;   // 品牌感知
const gap = space["4"];                              // 16
const theme = (id: BrandId) => brands[id].light.primary.default;
```

`gen/json/danqing.tokens.json` 是**已解析快照**（含全部原语与语义），
适合作为构建期数据源；`gen/ts/danqing.ts` 是可直接 import 的代码。

## 4. DTCG 与设计工具

`gen/json/danqing.tokens.json` 遵循 W3C Design Tokens Community Group 草案：

```json
{
  "cn": { "qing": { "700": { "$value": "#2E59A7", "$type": "color", "$description": "群青 Qún Qīng　◀ 锚点" } } },
  "sys": { "light": { "text": { "primary": { "$value": "#1E2732", "$type": "color" } } } },
  "brand": { "qing": { "light": { "primary": { "default": { "$value": "#2E59A7", "$type": "color" } } } } }
}
```

用途：

- **Style Dictionary**：`npx style-dictionary build` 可再生成任意平台（iOS/Android/Compose/SCSS…）
- **Figma Tokens / Tokens Studio**：直接导入，设计稿与代码共用同一份真源
- **自定义校验**：可用它做对照，防止设计稿与代码走偏

`$description` 里带传统色名，导入 Figma 后设计师看到的是「青 · 700 · 群青」而不是 `qing-700`。

## 5. Avalonia

```xml
<Application.Resources>
  <ResourceDictionary>
    <ResourceDictionary.MergedDictionaries>
      <ResourceInclude Source="avares://MyApp/gen/avalonia/Tokens.axaml" />
    </ResourceDictionary.MergedDictionaries>
  </ResourceDictionary>
</Application.Resources>
```

```xml
<Border Background="{DynamicResource DqSurfaceDefault}"
        BorderBrush="{DynamicResource DqBorderDefault}"
        CornerRadius="{StaticResource DqRadiusLg}">
  <TextBlock Foreground="{DynamicResource DqTextPrimary}" Text="{Binding Title}" />
</Border>
```

要点：

- 语义刷子位于 `ThemeDictionaries` 的 `Light` / `Dark` 键下，随 `Application.RequestedThemeVariant` 自动切换；
- 使用 `DynamicResource`（而非 `StaticResource`）引用语义刷子，以便运行时切主题；
- 层级值（`DqSpace8` / `DqRadiusLg`）是 `x:Double`，可直接用于 `Thickness` 的四则运算；
- 品牌切换：把 `DqPrimaryDefault` / `DqPrimaryHover` / `DqPrimaryActive` / `DqPrimaryOn` 四个键
  指向 `DqBrand{Id}{Role}` 即可，或运行时替换这四个键的值。

## 6. SwiftUI

```swift
import SwiftUI

struct CardView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: Dq.space3) {
            Text("标题").font(.system(size: Dq.fontSizeTitleSm, weight: .semibold))
                        .foregroundStyle(Dq.SemanticLight.textPrimary)
            Text("说明").foregroundStyle(Dq.SemanticLight.textSecondary)
        }
        .padding(Dq.space4)
        .background(Dq.SemanticLight.surfaceDefault)
        .clipShape(RoundedRectangle(cornerRadius: Dq.radiusLg, style: .continuous))
    }
}
```

**推荐做法**：把语义色写入 `Assets.xcassets` 颜色集（命名为 `DqTextPrimary` 等），
在 xcassets 里分别配置 Any / Dark 外观值，然后用 `Color("DqTextPrimary")`。
这样系统会在切换外观时自动重解析，无需手动管理 `colorScheme` 分支。

`Color(hex:)` 扩展已生成，方便快速起步，但它**不随外观切换**——只在调试或图表绘制中直接使用字面量值时才用它。

## 7. Jetpack Compose

```kotlin
import design.danqing.tokens.Dq

@Composable
fun DanqingTheme(dark: Boolean = isSystemInDarkTheme(), content: @Composable () -> Unit) {
    val s = if (dark) Dq.SemanticDark else Dq.SemanticLight
    MaterialTheme(
        colorScheme = lightColorScheme(
            primary = s.PrimaryDefault, onPrimary = s.PrimaryOn,
            background = s.BgBase, surface = s.SurfaceDefault,
            onBackground = s.TextPrimary, onSurface = s.TextPrimary,
            outline = s.BorderDefault, error = s.DangerDefault, onError = s.DangerOn,
        ),
        content = content,
    )
}
```

要点：

- **不要**使用 Material You 的动态取色（`dynamicColorScheme`），它会覆盖丹青的品牌色；
- Android 12+ 的 `MaterialTheme` 会自动叠加 tonal elevation，若与丹青的层级冲突，
  在需要精确控制的地方显式指定 `surface` 色；
- 触控尺寸用 `Dq.touchMin`（44.dp）作为 `Modifier.sizeIn(minHeight = …)`。

## 8. Flutter

```dart
import 'design_tokens.dart';

ThemeData danqingTheme({bool dark = false}) => ThemeData(
  scaffoldBackgroundColor: dark ? DqDark.bgBase : DqLight.bgBase,
  colorScheme: ColorScheme(
    brightness: dark ? Brightness.dark : Brightness.light,
    primary: dark ? DqDark.primaryDefault : DqLight.primaryDefault,
    onPrimary: dark ? DqDark.primaryOn : DqLight.primaryOn,
    surface: dark ? DqDark.surfaceDefault : DqLight.surfaceDefault,
    onSurface: dark ? DqDark.textPrimary : DqLight.textPrimary,
    error: dark ? DqDark.dangerDefault : DqLight.dangerDefault,
    onError: dark ? DqDark.dangerOn : DqLight.dangerOn,
  ),
  visualDensity: VisualDensity.standard,
);
```

## 9. Android XML

```xml
<!-- res/values/colors.xml（浅色） -->
<color name="dq_bg_base">#F5F2E9</color>
<color name="dq_text_primary">#1E2732</color>
<color name="dq_primary_default">#2E59A7</color>

<!-- res/values-night/colors.xml（深色，生成文件里名为 dq_*_dark） -->
<color name="dq_bg_base">#14171C</color>
<color name="dq_text_primary">#F5F2E9</color>
<color name="dq_primary_default">#416CBA</color>
```

生成器输出的 `dq_*_dark` 是**平铺**版本（便于不拆 `values-night` 时手动引用）。
若要支持系统级深色切换，把深色项移入 `values-night/colors.xml` 并使用同名资源。

```xml
<dimen name="dq_space_4">16dp</dimen>
<dimen name="dq_radius_lg">16dp</dimen>
<dimen name="dq_touch_min">44dp</dimen>
```

## 10. 跨端一致性检查清单

同一份令牌在不同端落地时最容易走偏的四件事：

- [ ] **`on` 色不要写死为白色**——檀色品牌与深色模式下的强调色都以墨色为 `on`
- [ ] **半透明色的表现**——CSS/Android 支持 `#AARRGGBB`，iOS/Compose 需显式拆 alpha；不要在某端退化成实色
- [ ] **层级实现方式不同**——Web 用 `box-shadow`，Avalonia/Compose/iOS 用各自的 elevation 概念；
      深色模式下应改用描边+提亮，而不是「更暗的阴影」
- [ ] **触控尺寸单位不同**——pt / dp / px 的换算在高 DPI 下必须走平台自适应，不要乘固定系数
