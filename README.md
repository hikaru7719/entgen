# entgen

# TODO

## 準備

- [x] docker-compose.ymlを準備する

## 入力

- [ ] CLIのインターフェースを確定する
- [ ] CLIのライブラリを選定する

## モデル

- [ ] 必要なデータ項目を洗い出す
- [ ] 必要なデータを取得する
- [ ] information_schemaテーブルの項目を確認する
- [ ] information_schemaからデータを取得する
- [ ] 中間データ構造を確定する

## information_schema

ユーザ定義テーブル一覧

```
SELECT * FORM information_schema.tables WHERE table_schema = 'public';
```

テーブルカラム情報一覧

```
SELECT                                    
    *
FROM
    information_schema.columns
WHERE
    table_name = <テーブル名>
ORDER BY
        ordinal_position;
```

## 出力

- [ ] 出力のデータ構造を確定する
    - [ ] sqlxのフォーマットに対応する
    - [ ] uuidやtimestampなどに対応するRustのデータ型を確定する
- [ ] ファイル出力のためのテンプレートエンジンを検討する
- [ ] 多言語対応する
    - [ ] Rustに対応する
- [ ] 複数DBに対応する
    - 標準SQLのデータ型に対応する
    - PostgreSQL固有のデータ型に対応する
    - MySQL固有のデータ型に対応する
- [x] build.rsの仕様を確認する
    - build.rsは利用しない方針

## その他

- [ ] エラーハンドリングの方針を決定する
- [ ] ログ出力の方針を決定する


