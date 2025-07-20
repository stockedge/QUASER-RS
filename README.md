# QUASAR-RS

**A Fast, Reliable, and Secure Programming Language for LLM Agents with Code Actions**

QUASARは、LLMエージェントのコードアクションに特化したプログラミング言語の実装です。論文「A Fast, Reliable, and Secure Programming Language for LLM Agents with Code Actions」に基づき、Rustで実装されています。

## 🌟 概要

QUASARは以下の特徴を持つプログラミング言語です：

- **高速性**: 並列実行可能な外部呼び出しの自動検出
- **信頼性**: 関数型セマンティクスによる明確な実行モデル
- **セキュリティ**: 外部関数呼び出しにユーザー承認が必要
- **不確実性の扱い**: コンフォーマルセマンティクスによる値の集合の表現

## 📋 言語仕様

### 基本構文

QUASARプログラムは以下の形式で構成されます：

```
P ::= stmt₁; ...; stmtₙ; return x
```

### データ型

```rust
// プリミティブ値
Boolean(true/false)
Integer(i64)
Float(f64)
String("text")
Null

// 複合型
List([value1, value2, ...])
Tuple((value1, value2, ...))

// コンフォーマル値（値の集合）
ConformValue { possibilities: {value1, value2, ...} }
```

### 文（Statement）

すべての文は `変数 = 操作` の形式：

```rust
stmt ::= x = op

op ::=
    | prim c              // プリミティブ値
    | x                   // 変数参照
    | (x₁, ..., xₙ)       // タプル構築
    | f x                 // 外部関数呼び出し
    | proj i x            // タプルの射影
    | fold w x block      // リストの畳み込み
    | if x block₁ block₂  // 条件分岐
    | ?S                  // 外部呼び出しプレースホルダー
    | join {x₁, ..., xₙ}  // 値の集合の結合
```

### ブロック（Block）

```rust
block ::= {x ⇒ P}
```

パラメータを受け取るプログラムブロック。主に`fold`や`if`で使用。

### 外部関数

QUASARでは外部関数呼び出しが特別に扱われます：

```rust
// 使用可能な外部関数
find(object) -> [patch1, patch2, ...]    // オブジェクト検索
simple_query(object) -> "yes"/"no"       // 簡単なクエリ
exists(object) -> true/false             // 存在確認
```

## 🏗️ アーキテクチャ

### モジュール構成

```
src/
├── ast/                 # 抽象構文木の定義
│   ├── value.rs        # 値とコンフォーマル値
│   ├── expression.rs   # 式の定義
│   ├── statement.rs    # 文の定義
│   └── program.rs      # プログラム全体
├── runtime/            # 実行時システム
│   ├── error.rs        # エラー型定義
│   ├── state.rs        # 実行状態管理
│   └── external.rs     # 外部関数実装
├── interpreter/        # インタープリター
│   ├── rewriter.rs     # 書き換えルール
│   ├── dispatcher.rs   # 外部呼び出しディスパッチ
│   ├── evaluator.rs    # 式評価
│   └── executor.rs     # メイン実行ループ
└── main.rs            # エントリーポイント
```

### 実行モデル

QUASARの実行は以下のサイクルで行われます：

1. **ディスパッチフェーズ**: 実行可能な外部呼び出しを検出
2. **承認フェーズ**: ユーザーに外部呼び出しの実行許可を求める
3. **非同期実行**: 承認された外部呼び出しを並列実行
4. **内部書き換え**: プログラムの内部ルールを適用
   - 変数の別名解決 (`alias`)
   - タプルの射影 (`proj`)
   - 条件分岐の解決 (`if-t`, `if-f`)
   - ループの展開 (`fold`)
   - 完了した外部呼び出しの結果代入 (`ext`)

### セマンティクス

#### 内部ルール（R_int）

| ルール | 説明 | 変換例 |
|--------|------|--------|
| **alias** | 変数のエイリアス解決 | `y = x` → `y`を`x`に置換 |
| **proj** | タプルの射影 | `y = proj 0 (a, b)` → `y = a` |
| **if-t/if-f** | 条件分岐の単純化 | `if true then A else B` → `A` |
| **fold** | ループの展開 | `fold [1,2] acc {...}` → 展開されたステップ |

#### 外部ルール（R_ext）

| ルール | 説明 |
|--------|------|
| **disp** | 外部関数呼び出しのディスパッチ |
| **ext** | 完了した外部呼び出しの結果代入 |

## 🚀 使用方法

### 依存関係

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ordered-float = { version = "4.0", features = ["serde"] }
futures = "0.3"
```

### 実行

```bash
# ビルド
cargo build

# 実行（ユーザー承認あり）
cargo run

# 実行（自動承認 - 未実装）
# cargo run -- --auto-approve
```

### サンプルプログラム

```rust
// 画像からドリンクを検索し、存在するもののみを収集
let statements = vec![
    // image_patch = "image_patch_object"
    Statement {
        variable: "image_patch".to_string(),
        expression: Expression::Primitive(Value::Primitive(
            PrimitiveValue::String("image_patch_object".to_string())
        )),
    },
    
    // drinks = find(image_patch)
    Statement {
        variable: "drinks".to_string(),
        expression: Expression::ExternalCall {
            function: "find".to_string(),
            argument: "image_patch".to_string(),
        },
    },
    
    // final_patches = fold drinks [] {...}
    Statement {
        variable: "final_patches".to_string(),
        expression: Expression::Fold {
            list: "drinks".to_string(),
            initial: "drink_patches".to_string(),
            block: Block { /* ... */ },
        },
    },
];
```

## ⚠️ 現在の状態と制限事項

### 実装済み機能

✅ 基本的なAST構造  
✅ 内部書き換えルール  
✅ 外部関数呼び出しのディスパッチ  
✅ 非同期実行管理  
✅ ユーザー承認システム  
✅ エラーハンドリング  

### 既知の問題

⚠️ **無限ループ**: 現在のサンプルプログラムでディスパッチループが発生  
⚠️ **入力処理**: 非同期入力処理に問題がある可能性  
⚠️ **終了条件**: プログラム終了の判定ロジックに改善が必要  

### 未実装機能

🔄 完全なコンフォーマルセマンティクス  
🔄 LSPサーバー  
🔄 構文解析器（現在は手動でASTを構築）  
🔄 自動承認モード  
🔄 デバッグ機能  

## 🔮 今後の拡張予定

### フェーズ1: 基本機能の安定化
- [ ] 無限ループ問題の修正
- [ ] 入力処理の改善
- [ ] テストスイートの追加
- [ ] エラーメッセージの改善

### フェーズ2: コンフォーマルセマンティクス
- [ ] 値の集合の完全サポート
- [ ] 不確実性の伝播
- [ ] 条件分岐での集合値処理
- [ ] ループでの集合値処理

### フェーズ3: 開発者体験の向上
- [ ] 構文解析器の実装
- [ ] LSPサーバーの実装
- [ ] VS Code拡張
- [ ] デバッガーの実装

### フェーズ4: 高度な機能
- [ ] 並列化の最適化
- [ ] セキュリティポリシーエンジン
- [ ] パフォーマンス監視
- [ ] 分散実行

## 📚 技術詳細

### コンフォーマルセマンティクス

QUASARの特徴的な機能として、値が「集合」である可能性を表現できます：

```rust
// 確実な値
ConformValue::certain(Value::Primitive(PrimitiveValue::Boolean(true)))

// 不確実な値（集合）
ConformValue::uncertain(vec![
    Value::Primitive(PrimitiveValue::Boolean(true)),
    Value::Primitive(PrimitiveValue::Boolean(false)),
])
```

### 書き換えルールの実装

書き換えルールはパターンマッチングを使用して実装：

```rust
match &stmt.expression {
    Expression::Variable(src_var) => {
        // alias ルール
        if let Some(value) = state.lookup_var(src_var) {
            state.set_var(stmt.variable.clone(), value.clone());
            changed = true;
        }
    }
    Expression::If { condition, then_block, else_block } => {
        // if-t/if-f ルール
        let cond_value = state.lookup_var(condition);
        // 条件に応じて分岐を処理
    }
    // ...
}
```

### 非同期実行管理

外部関数呼び出しは`tokio::spawn`で非同期実行：

```rust
let handle = task::spawn(async move {
    if let Some(func) = get_external_function(&function_name) {
        func.call(&argument).await
    } else {
        Err(QuasarError::ExternalFunctionError(format!("Unknown function: {}", function_name)))
    }
});

state.pending_calls.push(PendingCall {
    id: call_id.clone(),
    assignment_var: call.assignment_var.clone(),
    handle,
});
```

## 🤝 貢献

このプロジェクトは論文の概念実装として作成されています。改善提案やバグ報告をお待ちしています。

## 📄 ライセンス

MIT License

## 📖 参考文献

- 論文: "A Fast, Reliable, and Secure Programming Language for LLM Agents with Code Actions"
- Rust公式ドキュメント: https://doc.rust-lang.org/
- Tokio非同期ランタイム: https://tokio.rs/