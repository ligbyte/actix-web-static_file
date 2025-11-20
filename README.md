# actix-web-static_file
================

Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust

# References

* Actix Web: https://actix.rs/
* Build an API in Rust with JWT Authentication: https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/


当本地开启的服务（比如 Web 服务、API 服务等）在本机可以访问，但同一局域网下的其他设备无法访问时，通常有以下几个常见原因和解决方法：

---

### ✅ 1. **服务绑定的地址问题（最常见）**

很多开发服务器默认只绑定 `127.0.0.1`（即 localhost），这意味着它**只接受来自本机的连接请求**。

#### 🔧 解决方法：
启动服务时，将监听地址改为 `0.0.0.0`（表示监听所有网络接口）。

- **Node.js / Express 示例：**
  ```js
  app.listen(3000, '0.0.0.0', () => {
    console.log('Server running on http://0.0.0.0:3000');
  });
  ```

- **Python Flask 示例：**
  ```python
  app.run(host='0.0.0.0', port=5000)
  ```

- **React/Vite / Vue CLI 等开发服务器：**
  启动时加上 `--host 0.0.0.0` 参数，例如：
  ```bash
  vite --host 0.0.0.0
  # 或
  npm run dev -- --host 0.0.0.0
  ```

> 💡 注意：不要写成 `localhost` 或 `127.0.0.1`，必须是 `0.0.0.0`。

---

### ✅ 2. **防火墙阻止了外部访问**

即使服务监听了 `0.0.0.0`，如果操作系统防火墙（Windows Defender 防火墙、macOS 防火墙、Linux 的 iptables/ufw 等）没有放行对应端口，其他设备也无法访问。

#### 🔧 解决方法：
- **Windows**：
  - 打开“Windows 安全中心” → “防火墙和网络保护” → “允许应用通过防火墙”。
  - 或临时关闭防火墙测试（不推荐长期使用）。
  - 也可以通过 PowerShell 添加入站规则：
    ```powershell
    New-NetFirewallRule -DisplayName "Allow Port 3000" -Direction Inbound -Protocol TCP -LocalPort 3000 -Action Allow
    ```

- **macOS**：
  - 系统设置 → 隐私与安全性 → 防火墙 → 选项 → 确保允许你的程序通信。
  - 或临时关闭防火墙测试。

- **Linux (Ubuntu/Debian)**：
  ```bash
  sudo ufw allow 3000/tcp
  ```

---

### ✅ 3. **确认本机 IP 地址是否正确**

局域网其他设备需要通过你本机的**局域网 IP**（如 `192.168.x.x` 或 `10.x.x.x`）来访问，而不是 `127.0.0.1`。

#### 🔍 查看本机局域网 IP：
- **Windows**：
  ```cmd
  ipconfig
  ```
  找到 “无线局域网适配器” 或 “以太网适配器” 下的 IPv4 地址。

- **macOS / Linux**：
  ```bash
  ifconfig
  # 或
  ip a
  ```

然后在其他设备浏览器中访问：`http://<你的IP>:端口号`，例如：
```
http://192.168.1.100:3000
```

---

### ✅ 4. **路由器或网络隔离设置（较少见）**

某些企业或学校网络启用了“客户端隔离”（AP Isolation），禁止局域网设备之间互相通信。

#### 🔧 解决方法：
- 尝试用手机连同一个 Wi-Fi，看是否能访问。
- 如果不能，可能是网络策略限制，需联系网络管理员。
- 家用路由器一般默认允许局域网互访。

---

### ✅ 5. **服务本身限制了 CORS 或 Referer（仅影响前端调用，不影响直接访问）**

如果你是在浏览器中通过 AJAX 调用该服务，可能会因 CORS 被拦截，但这和“无法访问”不同——服务其实可达，只是浏览器阻止了响应。可通过浏览器开发者工具 Network 标签查看具体错误。

---

### 🛠️ 快速排查步骤总结：

1. 确认服务监听的是 `0.0.0.0:端口`（不是 `127.0.0.1`）。
2. 在本机执行 `netstat -an | grep <端口>`（Linux/macOS）或 `netstat -ano | findstr :<端口>`（Windows），确认监听状态。
3. 关闭防火墙临时测试（确认是否是防火墙问题）。
4. 在另一台设备上 `ping 你的IP`，确认网络连通。
5. 在另一台设备浏览器访问 `http://你的IP:端口`。

---

如果你告诉我你用的是什么语言/框架（比如 Flask、Express、Vite、Django 等）以及操作系统，我可以给出更具体的命令或配置示例。