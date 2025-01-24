# Nix

## 安装

> <https://zero-to-nix.com/start/install/>

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

## 新命令

<https://nix.dev/manual/nix/2.24/command-ref/experimental-commands>

## nix search

用于软件搜索：

```bash
# 常规搜索
nix search nixpkgs font

# 缩小范围
nix search nixpkgs font chinese

# 知道具体名称的 (正则表达式)
nix search nixpkgs ^qq$
```

## nix run

旧版命令是 nix-shell -p，传统发行版运行软件一般要管理员权限全局安装后运行，比如 debian：

```bash
sudo apt install yafetch
yafetch
```

nix 里也有等价的命令 nix profile install 但很少使用，nix 想用什么软件就直接：

```bash
nix run nixpkgs#yafetch
nix run nixpkgs#cbonsai -- -l  # with arguments
```

立即运行软件，而且无需 sudo 权限，非常方便。

## nil profile

旧版是 nix-env，这个基本就等价于传统发行版的包管理（例如 apt）。一般在 nixos 里很少使用，因为常用的软件直接加入配置固定了，不常用的直接 nix run 或者 nix shell。

不过在 wsl 里懒得配置就可以直接用 nix profile：

```bash
nix profile install nixpkgs#yafetch
nix profile remove yafetch
nix profile list
```

## nix-collect-garbage

垃圾回收：

```bash
sudo nix-collect-garbage -d
```

如果有安装 home-manager，那么垃圾回收需要执行的命令是：

```bash
sudo nix-collect-garbage -d && nix-collect-garbage -d
```

## nix-info

查询系统的环境：

```bash
nix-info -m
```

## nix config

查询 nix 环境：

```bash
nix config show
```
