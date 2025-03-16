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

### 两者的区别

#### 编译时生成的 Keypair（例如 program-keypair.json）

* 来源：在编译`Solana`程序（比如用`solana-program build`或`Anchor`的`anchor build`）时生成，通常位于 `target/deploy/` 目录下。
* 用途：
  * 这个`keypair`的公钥是程序的唯一标识`（Program ID）`，用于指定程序在链上的地址。
  * 私钥 用于在部署程序时签名交易，证明你有权将程序代码上传到这个地址。
* 特点：
  * 它是程序专属的，与程序的身份绑定。
  * 如果你重新生成一个新的 `keypair` 并用它部署，程序会有一个新的 `Program ID`。
* 文件示例：`target/deploy/my_program-keypair.json`。

#### 本地配置文件中的 Keypair（~/.config/solana/id.json）

* 来源：由`solana-keygen`或`solana config set`生成，通常是`Solana CLI`的默认钱包密钥对，存储在用户主目录下的 `~/.config/solana/id.json`。
* 用途：
  * 这是你的个人钱包密钥对，代表你的身份（类似于以太坊中的账户）。
  * 用于支付交易费用（比如部署程序时的`gas`费用）、签名普通交易等。
* 特点：
  * 它是你的“付款人”`（payer）`，与某个具体程序无关。
  * 这个 `keypair`控制你的`SOL`余额，用于支付程序部署的租金`（rent）`或交易费用。
* 文件示例：`~/.config/solana/id.json`。

#### 类比

* 程序 `Keypair（program-keypair.json）`：像是程序的“身份证”，定义了程序的地址和所有权。
* 本地 `Keypair（id.json）`：像是你的“银行卡”，用来支付费用和代表你的身份。

### 部署合约时用的是哪个？

部署`Solana`程序时，两个`keypair`都会用到，但作用不同：
* 程序`Keypair（program-keypair.json）`：
  * 指定程序的`Program ID`（公钥）。
  * 私钥用于签名程序部署交易，确保程序被正确写入链上的指定地址。
* 本地`Keypair（id.json）`：
  * 作为`payer`（付款人），支付部署程序所需的`SOL`费用（包括账户租金和交易费用）。
  * `Solana CLI`默认使用`~/.config/solana/id.json`作为`payer`，除非你通过`--keypair`参数指定其他钱包。

### Example
```
solana program deploy \
--program-id target/deploy/my_program-keypair.json \
target/deploy/my_program.so
```
* `--program-id target/deploy/my_program-keypair.json`：指定程序的 Keypair，决定`Program ID`。
* 默认使用`~/.config/solana/id.json`作为`payer`来支付费用。
* 如果你想用其他钱包支付，可以加参数
  ```
  solana program deploy \
  --program-id target/deploy/my_program-keypair.json \
  --keypair /path/to/custom_payer.json \
  target/deploy/my_program.so
  ```

### 具体流程

* 编译程序：生成`my_program.so`（BPF 字节码）和`my_program-keypair.json`。
* 部署程序：
  * `Solana CLI`使用`my_program-keypair.json`的公钥作为`Program ID`。
  * 使用`id.json`的私钥签名交易并支付费用。
* 结果：程序部署到链上，地址是`my_program-keypair.json`的公钥。


### 注意事项

* `program-keypair.json` 的私钥只用于部署和升级，泄露可能导致程序被恶意升级（如果可升级）。
* `id.json` 是你的钱包，泄露会导致资金丢失。
* 在测试网`（Devnet）`上，你可以随便生成新的程序`keypair`。
* 在主网上，建议固定程序`keypair`并妥善保存，因为`Program ID`是用户和客户端依赖的地址。


### 总结
* 编译生成的`Keypair（program-keypair.json）`：决定程序的身份`（Program ID）`。
* 本地`Keypair（id.json）`：作为你的钱包，支付部署费用。
* 部署时：两者都用，程序`Keypair`定义地址，本地`Keypair`支付费用。










