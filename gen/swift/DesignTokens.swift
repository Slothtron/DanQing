// 丹青 —— SwiftUI 令牌（生成物，勿手改）
// 建议：把语义色导出为 Assets.xcassets 颜色集以获得自动 Appearance 切换；
// 下方语义枚举值即颜色集的字面值。
import SwiftUI

public enum Dq {

  // MARK: tier1 原语 · 中国传统色族
  /// 青 Qīng · 锚点 群青 #2E59A7
  public enum Qing {
    public static let s50 = Color(hex: "#F2F7FF")  // 山矾
    public static let s100 = Color(hex: "#D1E2FF")  // 月白
    public static let s200 = Color(hex: "#B1CAF6")  // 碧落
    public static let s300 = Color(hex: "#92B2EB")  // 窃蓝
    public static let s400 = Color(hex: "#759BDC")  // 监德
    public static let s500 = Color(hex: "#5A83CC")  // 紫苑
    public static let s600 = Color(hex: "#416CBA")  // 青冥
    public static let s700 = Color(hex: "#2E59A7")  // 群青
    public static let s800 = Color(hex: "#1A4188")  // 绀宇
    public static let s900 = Color(hex: "#092964")  // 帝释青
    public static let s950 = Color(hex: "#011541")  // 獭见
  }
  /// 朱 Zhū · 锚点 银朱 #D12920
  public enum Zhu {
    public static let s50 = Color(hex: "#FFF4F2")  // 山矾
    public static let s100 = Color(hex: "#FFD6D0")  // 盈盈
    public static let s200 = Color(hex: "#FFB3A7")  // 海天霞
    public static let s300 = Color(hex: "#FF8C7C")  // 朱颜酡
    public static let s400 = Color(hex: "#F56757")  // 朱柿
    public static let s500 = Color(hex: "#E24437")  // 檎丹
    public static let s600 = Color(hex: "#D12920")  // 银朱
    public static let s700 = Color(hex: "#AB0003")  // 胭脂虫
    public static let s800 = Color(hex: "#870001")  // 顺圣
    public static let s900 = Color(hex: "#5D0001")  // 爵头
    public static let s950 = Color(hex: "#390000")  // 油紫
  }
  /// 缃 Xiāng · 锚点 柘黄 #C67915
  public enum Xiang {
    public static let s50 = Color(hex: "#FFF5EB")  // 凝脂
    public static let s100 = Color(hex: "#FEDAB8")  // 弗肯红
    public static let s200 = Color(hex: "#F3BD89")  // 扶光
    public static let s300 = Color(hex: "#E5A15C")  // 椒房
    public static let s400 = Color(hex: "#D38731")  // 郁金裙
    public static let s500 = Color(hex: "#C67915")  // 柘黄
    public static let s600 = Color(hex: "#9E5D00")  // 黄流
    public static let s700 = Color(hex: "#814B00")  // 紫瓯
    public static let s800 = Color(hex: "#653A00")  // 枣褐
    public static let s900 = Color(hex: "#442500")  // 青骊
    public static let s950 = Color(hex: "#281400")  // 獭见
  }
  /// 绿 Lǜ · 锚点 官绿 #2A6E3F
  public enum Lv {
    public static let s50 = Color(hex: "#EFFAF1")  // 酂白
    public static let s100 = Color(hex: "#D2E8D6")  // 卵色
    public static let s200 = Color(hex: "#B2D3B8")  // 山岚
    public static let s300 = Color(hex: "#93BE9C")  // 渌波
    public static let s400 = Color(hex: "#75A981")  // 青楸
    public static let s500 = Color(hex: "#599468")  // 庭芜绿
    public static let s600 = Color(hex: "#3E7F50")  // 翠微
    public static let s700 = Color(hex: "#2A6E3F")  // 官绿
    public static let s800 = Color(hex: "#115329")  // 螺青
    public static let s900 = Color(hex: "#003917")  // 螺子黛
    public static let s950 = Color(hex: "#00210A")  // 獭见
  }
  /// 绛 Jiàng · 锚点 朱樱 #8F1D22
  public enum Jiang {
    public static let s50 = Color(hex: "#FFF4F3")  // 山矾
    public static let s100 = Color(hex: "#FFD6D2")  // 盈盈
    public static let s200 = Color(hex: "#F4B8B3")  // 桃夭
    public static let s300 = Color(hex: "#E79B95")  // 檀唇
    public static let s400 = Color(hex: "#D77E78")  // 琼琚
    public static let s500 = Color(hex: "#C5635E")  // 牙绯
    public static let s600 = Color(hex: "#B14745")  // 鞓红
    public static let s700 = Color(hex: "#9B2C2D")  // 朱湛
    public static let s800 = Color(hex: "#8F1D22")  // 朱樱
    public static let s900 = Color(hex: "#5C000A")  // 爵头
    public static let s950 = Color(hex: "#380004")  // 油紫
  }
  /// 黛 Dài · 锚点 青黛 #45465E
  public enum Dai {
    public static let s50 = Color(hex: "#F6F6FB")  // 山矾
    public static let s100 = Color(hex: "#E0E0EA")  // 月白
    public static let s200 = Color(hex: "#C7C8D5")  // 影青
    public static let s300 = Color(hex: "#AFB0C1")  // 月魄
    public static let s400 = Color(hex: "#9899AD")  // 紫菂
    public static let s500 = Color(hex: "#818298")  // 迷楼灰
    public static let s600 = Color(hex: "#6B6C84")  // 菘蓝
    public static let s700 = Color(hex: "#565770")  // 曾青
    public static let s800 = Color(hex: "#45465E")  // 青黛
    public static let s900 = Color(hex: "#2B2C40")  // 绀蝶
    public static let s950 = Color(hex: "#171827")  // 獭见
  }
  /// 墨 Mò · 锚点 瑾瑜 #1E2732
  public enum Mo {
    public static let s50 = Color(hex: "#F5F7F9")  // 山矾
    public static let s100 = Color(hex: "#DEE2E6")  // 月白
    public static let s200 = Color(hex: "#C5CAD0")  // 影青
    public static let s300 = Color(hex: "#ACB2BA")  // 月魄
    public static let s400 = Color(hex: "#949BA4")  // 竹月
    public static let s500 = Color(hex: "#7D858F")  // 迷楼灰
    public static let s600 = Color(hex: "#67707A")  // 石涅
    public static let s700 = Color(hex: "#525B66")  // 育阳染
    public static let s800 = Color(hex: "#3D4652")  // 霁蓝
    public static let s900 = Color(hex: "#1E2732")  // 瑾瑜
    public static let s950 = Color(hex: "#121A24")  // 獭见
  }
  /// 素 Sù · 锚点 凝脂 #F5F2E9
  public enum Su {
    public static let s50 = Color(hex: "#F5F2E9")  // 凝脂
    public static let s100 = Color(hex: "#E4E1D8")  // 二目鱼
    public static let s200 = Color(hex: "#CCC9C1")  // 藕丝秋半
    public static let s300 = Color(hex: "#B4B2AA")  // 葭灰
    public static let s400 = Color(hex: "#9D9B93")  // 绍衣
    public static let s500 = Color(hex: "#86847E")  // 石莲褐
    public static let s600 = Color(hex: "#716F69")  // 石涅
    public static let s700 = Color(hex: "#5B5A54")  // 结绿
    public static let s800 = Color(hex: "#474541")  // 驖骊
    public static let s900 = Color(hex: "#2F2E2A")  // 京元
    public static let s950 = Color(hex: "#1A1917")  // 獭见
  }
  /// 苍 Cāng · 锚点 铜青 #3D8E86
  public enum Cang {
    public static let s50 = Color(hex: "#ECFAF8")  // 山矾
    public static let s100 = Color(hex: "#CBE9E4")  // 天缥
    public static let s200 = Color(hex: "#A6D4CE")  // 沧浪
    public static let s300 = Color(hex: "#84BFB8")  // 繱犗
    public static let s400 = Color(hex: "#63AAA2")  // 二绿
    public static let s500 = Color(hex: "#3D8E86")  // 铜青
    public static let s600 = Color(hex: "#2D7D76")  // 鱼师青
    public static let s700 = Color(hex: "#186760")  // 石绿
    public static let s800 = Color(hex: "#04514B")  // 青緺
    public static let s900 = Color(hex: "#003632")  // 螺子黛
    public static let s950 = Color(hex: "#001F1C")  // 獭见
  }
  /// 紫 Zǐ · 锚点 紫紶 #7D5284
  public enum Zi {
    public static let s50 = Color(hex: "#FCF3FD")  // 山矾
    public static let s100 = Color(hex: "#ECDBEE")  // 盈盈
    public static let s200 = Color(hex: "#D9BFDD")  // 昌荣
    public static let s300 = Color(hex: "#C5A5CA")  // 紫薄汗
    public static let s400 = Color(hex: "#B18CB7")  // 紫菂
    public static let s500 = Color(hex: "#9C73A3")  // 茈藐
    public static let s600 = Color(hex: "#7D5284")  // 紫紶
    public static let s700 = Color(hex: "#714778")  // 三公子
    public static let s800 = Color(hex: "#5A3560")  // 三公子
    public static let s900 = Color(hex: "#3E2043")  // 凝夜紫
    public static let s950 = Color(hex: "#250F29")  // 獭见
  }
  /// 檀 Tán · 锚点 檀色 #B26D5D
  public enum Tan {
    public static let s50 = Color(hex: "#FFF4F1")  // 山矾
    public static let s100 = Color(hex: "#F9D9D1")  // 弗肯红
    public static let s200 = Color(hex: "#ECBDB1")  // 肉红
    public static let s300 = Color(hex: "#DCA193")  // 檀唇
    public static let s400 = Color(hex: "#CA8778")  // 琼琚
    public static let s500 = Color(hex: "#B26D5D")  // 檀色
    public static let s600 = Color(hex: "#9C5A4B")  // 棠梨褐
    public static let s700 = Color(hex: "#82473A")  // 朱石栗
    public static let s800 = Color(hex: "#683529")  // 蜜褐
    public static let s900 = Color(hex: "#492017")  // 麒麟竭
    public static let s950 = Color(hex: "#2D0F09")  // 獭见
  }
  // MARK: tier2 语义 · light
  public enum SemanticLight {
    public static let bgCanvas = Color(hex: "#E4E1D8")
    public static let bgBase = Color(hex: "#F5F2E9")
    public static let bgSunken = Color(hex: "#CCC9C1")
    public static let bgInverse = Color(hex: "#1E2732")
    public static let surfaceDefault = Color(hex: "#FFFFFF")
    public static let surfaceRaised = Color(hex: "#FFFFFF")
    public static let surfaceOverlay = Color(hex: "#FFFFFF")
    public static let surfaceSunken = Color(hex: "#E4E1D8")
    public static let surfaceVariant = Color(hex: "#CCC9C1")
    public static let surfaceInverse = Color(hex: "#3D4652")
    public static let borderSubtle = Color(hex: "#141E2732")
    public static let borderDefault = Color(hex: "#241E2732")
    public static let borderStrong = Color(hex: "#471E2732")
    public static let borderInverse = Color(hex: "#3DFFFFFF")
    public static let textPrimary = Color(hex: "#1E2732")
    public static let textSecondary = Color(hex: "#AD1E2732")
    public static let textTertiary = Color(hex: "#8C1E2732")
    public static let textDisabled = Color(hex: "#571E2732")
    public static let textInverse = Color(hex: "#F5F2E9")
    public static let textLink = Color(hex: "#2E59A7")
    public static let primaryDefault = Color(hex: "#2E59A7")
    public static let primaryHover = Color(hex: "#1A4188")
    public static let primaryActive = Color(hex: "#092964")
    public static let primarySubtle = Color(hex: "#D1E2FF")
    public static let primaryBorder = Color(hex: "#2E59A7")
    public static let primaryText = Color(hex: "#2E59A7")
    public static let primaryOn = Color(hex: "#FFFFFF")
    public static let accentDefault = Color(hex: "#7D5284")
    public static let accentSubtle = Color(hex: "#ECDBEE")
    public static let accentBorder = Color(hex: "#7D5284")
    public static let accentText = Color(hex: "#7D5284")
    public static let accentOn = Color(hex: "#FFFFFF")
    public static let successDefault = Color(hex: "#2A6E3F")
    public static let successSubtle = Color(hex: "#D2E8D6")
    public static let successBorder = Color(hex: "#2A6E3F")
    public static let successText = Color(hex: "#2A6E3F")
    public static let successOn = Color(hex: "#FFFFFF")
    public static let warningDefault = Color(hex: "#C67915")
    public static let warningSubtle = Color(hex: "#FEDAB8")
    public static let warningBorder = Color(hex: "#C67915")
    public static let warningText = Color(hex: "#814B00")
    public static let warningOn = Color(hex: "#0E1116")
    public static let dangerDefault = Color(hex: "#8F1D22")
    public static let dangerSubtle = Color(hex: "#FFD6D2")
    public static let dangerBorder = Color(hex: "#8F1D22")
    public static let dangerText = Color(hex: "#8F1D22")
    public static let dangerOn = Color(hex: "#FFFFFF")
    public static let infoDefault = Color(hex: "#3D8E86")
    public static let infoSubtle = Color(hex: "#CBE9E4")
    public static let infoBorder = Color(hex: "#3D8E86")
    public static let infoText = Color(hex: "#186760")
    public static let infoOn = Color(hex: "#0E1116")
    public static let stateHover = Color(hex: "#0A1E2732")
    public static let statePressed = Color(hex: "#141E2732")
    public static let stateSelected = Color(hex: "#1F1E2732")
    public static let stateDrag = Color(hex: "#291E2732")
    public static let stateDisabled = Color(hex: "#0D1E2732")
    public static let overlayScrim = Color(hex: "#7314171C")
    public static let overlayScrimstrong = Color(hex: "#AD14171C")
    public static let overlayGlass = Color(hex: "#B8FFFFFF")
    public static let focusRing = Color(hex: "#2E59A7")
    public static let focusOffset = Color(hex: "#FFFFFF")
    public static let skeletonBase = Color(hex: "#121E2732")
    public static let skeletonSheen = Color(hex: "#081E2732")
    public static let chartC1 = Color(hex: "#2E59A7")
    public static let chartC2 = Color(hex: "#3D8E86")
    public static let chartC3 = Color(hex: "#2A6E3F")
    public static let chartC4 = Color(hex: "#A9862E")
    public static let chartC5 = Color(hex: "#C67915")
    public static let chartC6 = Color(hex: "#D12920")
    public static let chartC7 = Color(hex: "#B83570")
    public static let chartC8 = Color(hex: "#7D5284")
    public static let chartC9 = Color(hex: "#B26D5D")
    public static let chartC10 = Color(hex: "#945635")
    public static let chartGrid = Color(hex: "#1A1E2732")
    public static let chartAxis = Color(hex: "#731E2732")
    public static let chartLabel = Color(hex: "#AD1E2732")
    public static let chartPositive = Color(hex: "#2A6E3F")
    public static let chartNegative = Color(hex: "#8F1D22")
    public static let chartNeutral = Color(hex: "#731E2732")
  }
  // MARK: tier2 语义 · dark
  public enum SemanticDark {
    public static let bgCanvas = Color(hex: "#101318")
    public static let bgBase = Color(hex: "#14171C")
    public static let bgSunken = Color(hex: "#0E1116")
    public static let bgInverse = Color(hex: "#F5F2E9")
    public static let surfaceDefault = Color(hex: "#1E2732")
    public static let surfaceRaised = Color(hex: "#3D4652")
    public static let surfaceOverlay = Color(hex: "#3D4652")
    public static let surfaceSunken = Color(hex: "#0E1116")
    public static let surfaceVariant = Color(hex: "#3D4652")
    public static let surfaceInverse = Color(hex: "#F5F2E9")
    public static let borderSubtle = Color(hex: "#17D4E5EF")
    public static let borderDefault = Color(hex: "#29D4E5EF")
    public static let borderStrong = Color(hex: "#4CD4E5EF")
    public static let borderInverse = Color(hex: "#3D1E2732")
    public static let textPrimary = Color(hex: "#F5F2E9")
    public static let textSecondary = Color(hex: "#B8D4E5EF")
    public static let textTertiary = Color(hex: "#8FD4E5EF")
    public static let textDisabled = Color(hex: "#66D4E5EF")
    public static let textInverse = Color(hex: "#1E2732")
    public static let textLink = Color(hex: "#759BDC")
    public static let primaryDefault = Color(hex: "#416CBA")
    public static let primaryHover = Color(hex: "#5A83CC")
    public static let primaryActive = Color(hex: "#759BDC")
    public static let primarySubtle = Color(hex: "#092964")
    public static let primaryBorder = Color(hex: "#416CBA")
    public static let primaryText = Color(hex: "#759BDC")
    public static let primaryOn = Color(hex: "#FFFFFF")
    public static let accentDefault = Color(hex: "#9C73A3")
    public static let accentSubtle = Color(hex: "#3E2043")
    public static let accentBorder = Color(hex: "#9C73A3")
    public static let accentText = Color(hex: "#B18CB7")
    public static let accentOn = Color(hex: "#0E1116")
    public static let successDefault = Color(hex: "#3E7F50")
    public static let successSubtle = Color(hex: "#003917")
    public static let successBorder = Color(hex: "#3E7F50")
    public static let successText = Color(hex: "#75A981")
    public static let successOn = Color(hex: "#FFFFFF")
    public static let warningDefault = Color(hex: "#C67915")
    public static let warningSubtle = Color(hex: "#442500")
    public static let warningBorder = Color(hex: "#C67915")
    public static let warningText = Color(hex: "#D38731")
    public static let warningOn = Color(hex: "#0E1116")
    public static let dangerDefault = Color(hex: "#B14745")
    public static let dangerSubtle = Color(hex: "#5C000A")
    public static let dangerBorder = Color(hex: "#B14745")
    public static let dangerText = Color(hex: "#D77E78")
    public static let dangerOn = Color(hex: "#FFFFFF")
    public static let infoDefault = Color(hex: "#3D8E86")
    public static let infoSubtle = Color(hex: "#003632")
    public static let infoBorder = Color(hex: "#3D8E86")
    public static let infoText = Color(hex: "#63AAA2")
    public static let infoOn = Color(hex: "#0E1116")
    public static let stateHover = Color(hex: "#0FD4E5EF")
    public static let statePressed = Color(hex: "#1AD4E5EF")
    public static let stateSelected = Color(hex: "#24D4E5EF")
    public static let stateDrag = Color(hex: "#2ED4E5EF")
    public static let stateDisabled = Color(hex: "#0FD4E5EF")
    public static let overlayScrim = Color(hex: "#9E000000")
    public static let overlayScrimstrong = Color(hex: "#C7000000")
    public static let overlayGlass = Color(hex: "#B814171C")
    public static let focusRing = Color(hex: "#416CBA")
    public static let focusOffset = Color(hex: "#14171C")
    public static let skeletonBase = Color(hex: "#17D4E5EF")
    public static let skeletonSheen = Color(hex: "#0AD4E5EF")
    public static let chartC1 = Color(hex: "#6B8FD4")
    public static let chartC2 = Color(hex: "#4FB3A6")
    public static let chartC3 = Color(hex: "#4E9C63")
    public static let chartC4 = Color(hex: "#D4B36A")
    public static let chartC5 = Color(hex: "#DE9A45")
    public static let chartC6 = Color(hex: "#E4665C")
    public static let chartC7 = Color(hex: "#D9739E")
    public static let chartC8 = Color(hex: "#A98BC0")
    public static let chartC9 = Color(hex: "#D69A86")
    public static let chartC10 = Color(hex: "#C08A6B")
    public static let chartGrid = Color(hex: "#1FD4E5EF")
    public static let chartAxis = Color(hex: "#73D4E5EF")
    public static let chartLabel = Color(hex: "#B8D4E5EF")
    public static let chartPositive = Color(hex: "#75A981")
    public static let chartNegative = Color(hex: "#D77E78")
    public static let chartNeutral = Color(hex: "#73D4E5EF")
  }
  // MARK: 品牌主色
  public enum Brand {
    public enum Qing {
      public static let lightDefault = Color(hex: "#2E59A7")
      public static let lightHover = Color(hex: "#1A4188")
      public static let lightActive = Color(hex: "#092964")
      public static let lightSubtle = Color(hex: "#D1E2FF")
      public static let lightBorder = Color(hex: "#2E59A7")
      public static let lightText = Color(hex: "#2E59A7")
      public static let lightOn = Color(hex: "#FFFFFF")
      public static let darkDefault = Color(hex: "#416CBA")
      public static let darkHover = Color(hex: "#5A83CC")
      public static let darkActive = Color(hex: "#759BDC")
      public static let darkSubtle = Color(hex: "#092964")
      public static let darkBorder = Color(hex: "#416CBA")
      public static let darkText = Color(hex: "#759BDC")
      public static let darkOn = Color(hex: "#FFFFFF")
    }
    public enum Zhu {
      public static let lightDefault = Color(hex: "#D12920")
      public static let lightHover = Color(hex: "#AB0003")
      public static let lightActive = Color(hex: "#870001")
      public static let lightSubtle = Color(hex: "#FFD6D0")
      public static let lightBorder = Color(hex: "#D12920")
      public static let lightText = Color(hex: "#AB0003")
      public static let lightOn = Color(hex: "#FFFFFF")
      public static let darkDefault = Color(hex: "#D12920")
      public static let darkHover = Color(hex: "#E24437")
      public static let darkActive = Color(hex: "#F56757")
      public static let darkSubtle = Color(hex: "#5D0001")
      public static let darkBorder = Color(hex: "#D12920")
      public static let darkText = Color(hex: "#F56757")
      public static let darkOn = Color(hex: "#FFFFFF")
    }
    public enum Dai {
      public static let lightDefault = Color(hex: "#45465E")
      public static let lightHover = Color(hex: "#2B2C40")
      public static let lightActive = Color(hex: "#171827")
      public static let lightSubtle = Color(hex: "#E0E0EA")
      public static let lightBorder = Color(hex: "#45465E")
      public static let lightText = Color(hex: "#45465E")
      public static let lightOn = Color(hex: "#FFFFFF")
      public static let darkDefault = Color(hex: "#6B6C84")
      public static let darkHover = Color(hex: "#818298")
      public static let darkActive = Color(hex: "#9899AD")
      public static let darkSubtle = Color(hex: "#2B2C40")
      public static let darkBorder = Color(hex: "#6B6C84")
      public static let darkText = Color(hex: "#9899AD")
      public static let darkOn = Color(hex: "#FFFFFF")
    }
    public enum Tan {
      public static let lightDefault = Color(hex: "#B26D5D")
      public static let lightHover = Color(hex: "#9C5A4B")
      public static let lightActive = Color(hex: "#82473A")
      public static let lightSubtle = Color(hex: "#F9D9D1")
      public static let lightBorder = Color(hex: "#B26D5D")
      public static let lightText = Color(hex: "#82473A")
      public static let lightOn = Color(hex: "#0E1116")
      public static let darkDefault = Color(hex: "#B26D5D")
      public static let darkHover = Color(hex: "#CA8778")
      public static let darkActive = Color(hex: "#DCA193")
      public static let darkSubtle = Color(hex: "#492017")
      public static let darkBorder = Color(hex: "#B26D5D")
      public static let darkText = Color(hex: "#CA8778")
      public static let darkOn = Color(hex: "#0E1116")
    }
  }
  // MARK: 尺度
  public static let space0: CGFloat = 0
  public static let space1: CGFloat = 4
  public static let space2: CGFloat = 8
  public static let space3: CGFloat = 12
  public static let space4: CGFloat = 16
  public static let space5: CGFloat = 20
  public static let space6: CGFloat = 24
  public static let space8: CGFloat = 32
  public static let space10: CGFloat = 40
  public static let space12: CGFloat = 48
  public static let space16: CGFloat = 64
  public static let space20: CGFloat = 80
  public static let space24: CGFloat = 96
  public static let radiusNone: CGFloat = 0
  public static let radiusXs: CGFloat = 4
  public static let radiusSm: CGFloat = 8
  public static let radiusMd: CGFloat = 12
  public static let radiusLg: CGFloat = 16
  public static let radiusXl: CGFloat = 20
  public static let radius2xl: CGFloat = 28
  public static let radiusPill: CGFloat = 999
  public static let iconXs: CGFloat = 12
  public static let iconSm: CGFloat = 16
  public static let iconMd: CGFloat = 20
  public static let iconLg: CGFloat = 24
  public static let iconXl: CGFloat = 32
  public static let icon2xl: CGFloat = 40
  public static let touchMin: CGFloat = 44
  public static let durationInstant: Double = 0.08
  public static let durationFast: Double = 0.14
  public static let durationNormal: Double = 0.22
  public static let durationSlow: Double = 0.32
  public static let durationDeliberate: Double = 0.42
}

public extension Color {
  init(hex: String) {
    let s = hex.hasPrefix("#") ? String(hex.dropFirst()) : hex
    var v: UInt64 = 0
    Scanner(string: s).scanHexInt64(&v)
    let a = s.count == 8 ? Double((v >> 24) & 0xFF) / 255 : 1
    let r = Double((v >> 16) & 0xFF) / 255
    let g = Double((v >> 8) & 0xFF) / 255
    let b = Double(v & 0xFF) / 255
    self.init(.sRGB, red: r, green: g, blue: b, opacity: a)
  }
}
