# Webstorm

## 最初に設定すること

### Node.jsとPackage Managerの設定

1. Node.jsとPackage Managerのバージョンを確認する
2. Preferences -> Languages & Frameworks -> Node.js
3. 使用しているNode.jsとPackage Managerを指定

### ESLintとPrettierの設定

1. Preferences -> Languages & Frameworks -> JavaScript -> Code Quality Tools -> ESLint
2. ESLintを有効化
3. Tools -> Actions on Save
4. Run eslint --fixとRun Prettierを有効化。それ以外は無効化