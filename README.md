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


## Solana
通过Sealevel技术实现并行交易处理，利用交易的读写集预先检测依赖，最大化并行度。`Solana`是一种高性能区块链，目标是实现每秒处理数万笔交易。
其核心创新之一是`Sealevel`，这是一个并行智能合约运行时，旨在最大化交易的并行执行能力。

### 机制：Sealevel的工作原理
* 交易的读写集`（Read-Write Set）`
  * 在`Solana`中，每笔交易在提交时需要明确声明它将访问和修改的账户（即“读写集”）。例如，一笔转账交易会声明它读取的账户余额和修改的目标账户。
  * 这种预先声明的机制允许系统在执行前分析交易之间的依赖关系。
* 依赖检测与并行调度
  * `Sealevel`会扫描所有待处理交易的读写集，识别哪些交易是独立的（即没有共享账户或状态），哪些交易存在冲突。
  * 无依赖的交易被分配到不同的线程或处理器核心并行执行，而有依赖的交易则按顺序执行。
* 多线程执行
  * `Solana`利用现代硬件的多核特性，将独立交易分发到多个线程，最大化硬件利用率。
  * 这与传统区块链（如以太坊）的单线程顺序执行形成鲜明对比。

### 解决的问题
* 数据竞争：通过预先声明读写集，`Sealevel`避免了并行执行中未经协调的账户状态修改。
* 状态冲突：依赖检测确保只有无冲突的交易并行运行，维护状态一致性。
* 性能瓶颈：利用多核`CPU/GPU`的并行能力，显著提升吞吐量（`Solana`宣称可达`50,000TPS`）。

### 优势
* 高吞吐量：通过最大化并行度，`Solana`能够在不牺牲去中心化的情况下处理大量交易。
* 确定性：依赖检测确保结果与顺序执行一致，满足区块链共识需求。

### 局限性
* 复杂性：要求开发者在交易中准确声明读写集，增加了开发难度。
* 跨账户依赖：如果大量交易涉及相同账户（如热门`DeFi`合约），并行度会受限。

### 总结
`Solana`的`Sealevel`通过交易的读写集和依赖检测，将并行执行的潜力发挥到极致，特别适用于高频交易场景，如去中心化交易所`（DEX）`。

## Ethereum 2.0
通过分片实现状态和交易的并行化，解决扩展性问题。 Ethereum 2.0（现已更名为以太坊共识层与执行层的结合）旨在通过分片`（Sharding）`解决以太坊主网
的扩展性瓶颈。分片允许网络并行处理交易和状态更新，从而提升整体性能。

### 机制：分片的工作原理
* 状态分片
  * 以太坊的全局状态（如账户余额、合约状态）被分成多个分片（计划为64个分片）。每个分片负责一部分状态和交易。
  * 例如，`分片1`可能处理地址以`0x00`开头的账户，`分片2`处理`0x01`开头的账户。
* 并行执行
  * 每个分片由不同的验证者节点组维护，交易在分片内顺序执行，但分片之间并行运行。
  * 无跨分片依赖的交易可以完全独立处理，从而实现并行化。
* 跨分片通信
  * 如果交易涉及多个分片（例如，从分片1的账户转账到分片2的账户），通过跨分片消息或信标链`（Beacon Chain）`协调。
  * 跨分片交易通常采用异步处理或两阶段提交机制，确保原子性和一致性。

### 解决的问题
* 状态冲突：分片将状态隔离到不同分区，减少了并行执行中的冲突。
* 性能瓶颈：通过将工作负载分配到多个分片，Ethereum 2.0的吞吐量从原来的 `15TPS` 提升到理论上的`数千TPS`。
* 执行顺序不确定性：分片内顺序执行，分片间通过信标链协调，确保全局一致性。

### 优势
* 扩展性：分片数量的增加直接提升网络容量，适应更高的交易需求。
* 去中心化：每个分片由独立验证者处理，保持了系统的分布式特性。

### 局限性
* 跨分片复杂性：跨分片交易的协调增加了延迟和实现难度。
* 数据可用性：验证者需要访问所有分片数据以验证全局状态，可能导致存储和带宽压力。

### 总结
Ethereum 2.0通过分片实现状态和交易的并行化，将扩展性问题分解为多个可并行处理的小问题。虽然跨分片通信仍需优化，但这种方法为以太坊的长期发展奠定了基础。

## EOS
使用多线程和并行执行模型，结合资源分配机制减少冲突。`EOS`是一种高性能区块链，采用委托权益证明`（DPoS）`共识机制，旨在通过并行执行和资源管理实现高吞吐量（宣称可达数千TPS）。

### 机制：多线程与资源分配
* 多线程执行
  * `EOS`的节点（如区块生产者）利用多核`CPU`，将交易分配到多个线程并行处理。
  * 交易被分组为独立的任务，无依赖关系的任务可以同时执行。
* 资源分配机制（CPU、NET、RAM）
  * `EOS`用户需要质押代币`（EOS）`来获取计算资源（`CPU`、时间、网络带宽、内存）。
  * 每个账户的交易执行受其可用资源限制，避免单一账户的交易占用过多系统资源。
  * 这种机制天然减少了并行执行中的资源竞争和冲突。
* 区块生产者优化：
  * 由21个经过选举的区块生产者负责验证和打包交易，他们可以根据硬件能力优化并行调度。
  * 交易依赖性通过区块生产者的调度算法处理，通常基于静态分析或预执行。

### 解决的问题
* 数据竞争：资源分配限制了每个账户的操作范围，减少了并行执行中的状态冲突。
* 性能瓶颈：多线程模型充分利用硬件资源，提升吞吐量。
* 状态冲突：通过预调度和资源隔离，确保并行交易不会意外覆盖状态。

### 优势
* 高性能：多线程和资源分配结合，使`EOS`适合企业级应用和高频交易。
* 灵活性：区块生产者可以根据负载动态调整并行策略。

### 局限性
* 中心化倾向：`DPoS`和少量区块生产者可能削弱去中心化特性。
* 资源成本：用户需要购买或质押`EOS`获取资源，增加了使用门槛。

### 总结
`EOS`通过多线程并行执行和资源分配机制，将交易处理能力提升到新的高度。其设计更偏向实用性和性能，但在去中心化方面有所妥协，适合对速度要求较高的商业场景。

## 对比与总结
| 特性     | Solana (Sealevel) | Ethereum 2.0 (分片) | EOS (多线程)  |
|--------|-------------------|-------------------|------------|
| 并行核心   | 读写集依赖检测           | 状态分片              | 多线程与资源分配   |
| 吞吐量    | 高（50,000 TPS）     | 中高（数千 TPS）        | 高（数千 TPS）  |
| 去中心化程度 | 高                 | 高                 | 中低（DPoS）   |
| 复杂性    | 开发者需声明读写集         | 跨分片通信复杂           | 资源管理增加用户成本 |
| 适用场景   | 高频交易、DeFi         | 通用区块链应用           | 企业级应用、DApp |











