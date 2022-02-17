---
marp: true
theme: gaia
paginate: true
size: 4:3
---

# 色々なプログラミング言語たち~これまでの愛と出会い、その変遷~

TU 坂田誠也

---

## お品書き

- 出会い(R)

- 初恋とライバル(Python, Java)

- 仕事で関わるようになったあの子(C#, JavaScript)

- 新たな仕事の付き合い(PHP, TypeScript)

- エリート育ちでスマートなあの子(Go)

- 最近人気上昇中で高嶺の花なあの子(Rust)

---

## 内容について

- 僕とは違う世界線だけど同じ名前の誰かのプログラミング言語の変遷と各言語の紹介の面白おかしい（？）お話です。

- 擬人化して紹介してますが、特定の性別に寄らないようにしているのであしからず。

- エンジニア的には特に新しい内容はないです。（特定の言語しか知らないのであれば新しい発見はあるかも）

---

## Who am I?

- 名前: 坂田誠也(Seiya Sakata)

- 年齢: 28(1993/4/4生まれ)

- エンジニア歴: 4年目(2018年4月~)

- 領域: 大体全部やってる ~~（どうしてこうなった）~~

- 趣味: プログラミング、麻雀、料理

  - 最近はAWS/GCP/Terraformで個人インフラ環境を構築したりRustでAPIサーバを作ったりネット麻雀してる。

---

# 本題

---

## 出会い(R)

~~Fラン~~大学を卒業してもなお就職をしていないサカタ君。
行きたい経済学系の大学院があるがあるので、パチ屋のバイトをしながら勉強をしている。

![width:250px](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/study_night_boy.jpg)

---

## 出会い(R)

統計学・計量経済学の数値計算をするときに、手計算じゃなくてコンピュータでやった方が楽なのではと思い、Rと出会う。（正直勉強の息抜き）

![image](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/Rlogo.png)

---

### R（アール）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>

![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/Rlogo.png)

- パパがニュージーランド人

- 統計解析が得意だよ

- RStudioやJupyter Notebookなどが相性がいいよ

- Pythonよりフレンドリー

- ggplot2やdplyrなど便利な道具を持ってるよ
  - ggplot2はチャート描画
  - dplyrはデータフレーム操作

---

##  出会い(R)

Rと出会って...

- プログラミングって実は面白いのでは？

- チャート可視化とか面白い

- 色んなことができることできるじゃん

---

## 初恋とライバル(Python, Java)

Rと出会ってプログラミング自体に興味が出たサカタ君。

もっと色んなことができるプログラミング言語は無いか探すことになった。

---

## 初恋とライバル(Python, Java)

次の候補はフレンドリーと噂の以下の中から検討した。

- JavaScript

- Ruby

- Python

---

## 初恋とライバル(Python, Java)

Rのような数値解析との親和性からPython、君に決めた！

![image](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/python-logo%402x.png)

---

### Python（パイソン）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>

![image](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/python-logo%402x.png)

- パパがオランダ人

- 書く字のフォーマットが決まっているので綺麗
  - インデントのエラーが存在するので強制的に綺麗になる

- The Zen of Pythonという思想を持っていて、ファンはみんな好き

- 色々な便利ツールを持っている
  - たくさんのライブラリがあり、プログラミングをするならとりあえずPythonになりつつある。

- 近年のみんなからの人気が今も衰えない（https://www.tiobe.com/tiobe-index/）

---

### Python（パイソン）

```sh
$ python
Python 3.10.1 (main, Feb 13 2022, 23:41:57) [Clang 12.0.0 (clang-1200.0.32.21)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import this
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!
>>> 
```

---

### Python（パイソン）

```
Beautiful is better than ugly.
醜いより美しいほうがいい。

Explicit is better than implicit.
暗示するより明示するほうがいい。

Simple is better than complex.
複雑であるよりは平易であるほうがいい。

There should be one-- and preferably only one --obvious way to do it.
何かいいやり方があるはずだ。誰が見ても明らかな、たったひとつのやり方が。

Now is better than never.
ずっとやらないでいるよりは、今やれ。
```

---

### 初恋とライバル(Python, Java)

<style scoped>
  img {
    margin-left: auto;
    margin-right: auto;
    display: block;
  }
</style>

Pythonの汎用性の高さとわかりやすさと美しさを追求する考え方にサカタ君は大好きになりました！
（コーディング試験をするなら今でもPython）

![w:300px](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/valentine_honmei_chocolate.png)

---

## 初恋とライバル(Python, Java)

そして大学院の二次試験に落ちたサカタ君。受かっても落ちてもプログラミングをやっていくと決めていたのですぐに動きます。
専門の学校（無料のプログラミングスクール的なとこ）に入ってJavaと出会います。

---

### Java（ジャヴァ）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>
![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/Javalogo.png)

- アメリカで生まれた

- 世界中にたくさんの知り合いがいて今でのたくさんの需要がある
  - 開発元のOracleは3億のデバイスで動いていると言ってる

- 適応力が高いのでどこでも活動できる
  - JVM(Java Virtual Machine)によりどのコンピュータでも動く

---

## 初恋とライバル(Python, Java)

Pythonはゆるい雰囲気ですが、Javaはもっとお堅い雰囲気があります。サカタ君はPythonは好きですがJavaのお堅いながらも堅実な雰囲気にも惹かれるようになりました。

---

## 初恋とライバル(Python, Java)

この相反する二人の間でサカタ君は葛藤することとなります。

![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/pose_atama_kakaeru_man.png)

---

## 仕事で関わるようになったあの子(C#, JavaScript)

そして晴れて仕事をするようになってNEETを脱出したサカタ君。

そこに待ち受けていたのは初めて見るC#とJavaScriptでした。

---

### C#（シーシャープ）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>
![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/1920px-C_Sharp_wordmark.svg.png)

- パパはデンマーク人

- Microsoft育ちのエリート

- Javaが永遠のライバル

- 一番活躍してるのはゲーム
  - Web Assemblyも来てる(Blazor)

- 最近トレンドを取り入れつつある
  - 特にVer. 8.0が素晴らしい
  - 参照型もdefaultでnull非許容、switch式、再帰パターン、非同期ストリーム

---

### JavaScript（ジャバスクリプト）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>

![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/Unofficial_JavaScript_logo_2.svg)

- パパがアメリカ人

- マルチで色々動けるが問題児
  - ブラウザでもサーバでも動く
  - 過去の不可解な仕様を引きずっていたりよくない実装が残ってる（ex: `typeof null -> "object"`）

- 色々な人から ~~（愛と憎悪含め）~~ いつも注目される
  - Web Developerであればみんな使ったことある

---

## 新たな仕事の付き合い(PHP, TypeScript)

そして新しい会社に移って、また新しい出会いがありました。

そしてその子はサカタ君が食わず嫌いで若干避けていたあの子でした。

---

### PHP（ピーエイチピー）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>

![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/php-logo.svg)

- パパはデンマーク人
- 個人的前評判悪かった子だけど、意外とできる子
- 賢くなったのでちょっと評価上がった
  - type check
  - JITコンパイラで速度改善
  - match式とか名前付き引数とか
- なんだかんだで色々な人から頼りにされる
  - 正直PHPができれば職に困ることないと思ってる。

---

### TypeScript（タイプスクリプト）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 130px;
    left: auto;
    width: 200px;
  }
</style>
![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/Typescript_logo_2020.svg.png)

- エリート教育を受けたJavaScript
  - Microsoftが開発

- 以前よりもだいぶお堅い雰囲気に
  - 型による実行前の静的解析ができるので前よりも格段にエラーの原因を見つけやすい

- この子なしでは生きていけない人が多数
  - もはや全人類待っていたのでは？

---

## エリート育ちでスマートなあの子(Go)

サカタ君はおしゃべりは面白くないが

---

### Go（ゴー）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:80px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>
![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/2880px-Go_Logo_Aqua.svg.png)

- パパはカナダ人とアメリカ人

- Google育ちでエリート教育を受けている
  - Rob PikeもKenneth Tompsonもプログラミング言語の専門家

- C++が嫌い
  - Goが生まれた理由はC++が嫌いだから

- ここ数年で大人気に
  - どこもかしこもGoだらけ

- 十分速いコンパイル速度と実行時間

---

### Go（ゴー）

- シンプルな機能と文法

- 充実している標準ライブラリ

- シンプルゆえに楽しさは少ない
  - 面白みに欠けるのはそれだけ誰でも使えるようにしているから

- 好きな子に近いものを感じる
  - どことなくPythonぽさがある（気がする）

---

## 最近人気上昇中で高嶺の花なあの子(Rust)

---

### Rust（ラスト）

<style scoped>
  img {
    position:absolute;
    display:block;
    right:60px;
    top: 100px;
    left: auto;
    width: 200px;
  }
</style>
![images](https://raw.githubusercontent.com/kanade0404/ltslide/master/programming-languages/rust-logo-blk.svg)

- 他の子にはできない技術を持っている
  - GCなしでメモリの確保・開放を行う
  （メモリ: プログラムを実行するためのコンピュータ上の領域のこと）

- トレンドを知り尽くしている
  - null safe, Pattern Match, immutable by default, traitなどモダンな仕組みが一通り整っている

- twitter元CEO「rust is a perfect programming language」

---

### Rust（ラスト）

- これまでの子たちの誰よりも堅い
  - 強力な型推論とそれを可能にするエコシステムが備わっている。

- 誰よりも素早い
  - Cという最速選手には及ばないが、次ぐらいに速い。速さは正義。

---

### Rust（ラスト）

サカタ君「この言語やばくね...？」

こうしてサカタ君はRustに一本釣りされました。

---

## 現在の関係

そんなこんなで今のサカタ君の関係はこちら！

---

### R

昔の友達。ごめん、最近遊べてないね...また遊ぼ

### Python

元恋人でプライベートでも遊ぶ、仕事はたまに一緒に関わるけどズッ友。 ~~（死語？）~~

### Java

ScalaかKotlinの方が好きだけど勝てる？（ScalaとKotlinはJavaの親戚あるいか兄弟みたいな感じ）

---

## 現在の関係

### C#

昔の仕事仲間。まだ少し興味はある。

### PHP

仕事で関わるだけの関係の同僚。

### TypeScript

基本仕事だけの関係だけど最近プライベートでも遊ぶことが増えてきた同僚。

---

## 現在の関係

### Go

仕事でも関わるしプライベートでも遊ぶようになった同僚。元恋人に近いものを感じるので好感度は高い。

### Rust

プライベートでよく声をかけて遊んでもらっている友達。一方的に好きだけど僕がまだ相手を理解し切れていない...

---

## その他のプログラミング言語たち

- Ruby（ルビー）
  - 元恋人（Python）と逆を行くので合わない。

- Scala（スカラ）
  - かっこいい！けど分かり合えてない。

- Kotlin（コトリン）
  - 君も良いと思うよ！でもまだあまり分かってないんだごめんね。

- Ocaml（オキャムル）、F#（エフシャープ）
  - 半年か1年後ぐらいになったら...

---

## 番外編：各プログラミング言語への様々なお気持ち

- JavaScript
  - 多くの負債を抱えた言語がブラウザを支配しているのは独占禁止法で取り締まってくれ。
  - TypeScriptなしは無駄に脆弱性を増やしているだけなので論外。

---

## 番外編：各プログラミング言語への様々なお気持ち

- JavaScript
  - いわゆる標準的なプログラミング言語とは別だと思っているので、初心者に最初にJavaScriptを教えるのは良くないと思ってる。
    - 本気でエンジニアになるつもりならCかGoかJava、とりあえずプログラミングやりたいぐらいならGASかPython（Jupyter Notebook実行）

---

## 番外編：各プログラミング言語の様々なお気持ち

- Visual Basic
  - これ使わされるの犯罪なので取り締まってほしい

- Ruby
  - 型が再評価されている時代に逆行していつまでもRubyという感じがあるので二の足を踏む。
  - Web開発になるとRails一択になってしまうのが多様性なさすぎて厳しい

---

## 番外編：各プログラミング言語の様々なお気持ち

- Rust
  - collect()が辛い

- Python
  - そろそろconst欲しい...欲しくない？
  - 速度は3.11で改善されるらしいがどうなの？

---

# Thank you for listening!
