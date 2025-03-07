# Solana Program Tutorials
Solana智能合约入门教程


## 常用命令
```
solana --version
solana-keygen new

solana address
solana balance

solana config set -ud
solana config get

solana program deploy ./target/deploy/program.so

solana program show --programs
solana program show Bcs3x7JCccn5oWsQvVEpohGgVyYEZXGVJh8xns5yNRTk
```
## 编译
### cargo build
* 用途：用于构建标准的`Rust`应用或库。
* 目标（Target）：编译为本机架构（如`x86_64`或`aarch64`）。
* 不适用于`Solana`智能合约，因为`Solana`运行的是`BPF/SBF`目标架构，而非本机架构。
### cargo build-bpf（旧版命令，已被build-sbf取代）
### cargo build-sbf
* 用途：用于编译 `Solana` 智能合约（以前的`Solana`运行时使用`BPF`，现已迁移到`SBF`）。
* 目标（Target）：`bpfel-unknown-unknown`（`Solana`的`BPF`运行时）。
* 输出文件：会生成 `.so` 共享库，存放在`target/deploy/`。

| 命令                 | 目标架构                    | 适用场景            | 备注                |
|--------------------|-------------------------|-----------------|-------------------|
| `cargo build`	     | 本机架构（`x86_64/aarch64`）  | 	`Rust`应用程序、库	  | 不能用于`Solana`合约    |
| `cargo build-bpf`	 | `bpfel-unknown-unknown` | 	旧版`Solana`智能合约 | 	已废弃，用`build-sbf` |
| `cargo build-sbf`	 | `sbf-solana-solana`	    | 新版`Solana`智能合约	 | 推荐使用              |

###  Program编译后生成的keypair.json
* `keypair.json`代表程序账户`（Program Account）`的私钥，用于在部署程序时为其签名。
* 用途：
  * 仅用于部署该程序到`Solana`网络。
  * 不能直接用于签署交易或当作用户账户使用。

### solana-keygen new生成的keypair.json
* 是一个 普通`Solana`账户`（Wallet Account）`，可以用于持有`SOL`代币、签署交易等。
* 用途：
  * 作为普通钱包账户（PDA除外）。
  * 可用于执行和签署交易，如`solana transfer`。

| 特性   | Program编译后生成的`keypair.json`           | `solana-keygen new`生成的`keypair.json` |
|------|---------------------------------------|--------------------------------------|
| 用途   | 仅用于部署`Solana`程序                       | 普通`Solana`账户，可用于转账、签名等               |
| 权限   | 不能用于签署交易                              | 可签署交易                                |
| 生成方式 | 由`anchor build`或`cargo build-sbf`自动生成 | 手动运行`solana-keygen new`生成            |
| 作用   | 代表程序账户的私钥                             | 代表用户或合约的私钥                           |


* 如果你要部署`Solana`程序，你需要使用 编译后生成的`keypair.json`，而不是`solana-keygen new`生成的普通钱包`keypair`。
* 如果你要管理一个普通`Solana`账户，使用`solana-keygen new`生成的`keypair.json`才是合适的。


### Program ID计算的核心因素

* `Keypair`（私钥）
  * `Program ID`主要是由你为`Solana`合约部署时所使用的`keypair.json`（私钥）计算得出的。
  * `Solana`使用该私钥的公钥`（public key）`来作为`Program ID`。
  * 在`Solana`中，每个`keypair`都对应着一个唯一的公钥，而这个公钥就是该合约的`Program ID`。






