# Ubuntu

## 便利なコマンド集

### アップデート

```bash
sudo apt update
sudo apt upgrade
```

### 現在のユーザーグループに権限を付与したいとき

```bash
sudo chown $USER:$USER -R * 
```

### 現在のportを調べてkillしたいとき

```bash
sudo lsof -i :80
sudo kill -9 8080
```