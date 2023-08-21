# Git

## よく使うコマンド

### ローカルで特定のブランチ(mainとdevelop)以外を消したいとき 

```bash
git branch| egrep -v "\*|main|develop" | xargs git branch -D
```