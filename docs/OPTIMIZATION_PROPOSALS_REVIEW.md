# 优化提案评估（2026-05-20）

基于 `v0.2.2` 与提案文档的对照结论。**本次已落地**：P0（#1 #2）、P2（#7）、任务统一（#9）、xcbeautify 回退提示（补充项）、MCP `inputSchema`（#5 精简版）。

## 总表

| # | 优先级 | 建议 | 评估 | 本次 |
|---|--------|------|------|------|
| 1 | P0 | 项目检测去重 | ✅ 合理，零行为变化 | 已做 |
| 2 | P0 | 单元测试 | ✅ 必要，防 destination 回归 | 已做 |
| 3 | P1 | 增量构建跳过 | ⚠️ 风险高，见下 | 未做 |
| 4 | P1 | 实时构建输出 | ❌ 当前已 `Stdio::inherit`，收益有限 | 未做 |
| 5 | P1 | MCP 参数化 | ✅ 合理 | 已做 |
| 6 | P1 | 构建诊断 + 日志持久化 | ✅ 合理，可分期 | 未做 |
| 7 | P2 | xcbeautify 缓存 | ✅ 合理 | 已做 |
| 8 | P2 | 全局配置 TTL 缓存 | ⚠️ 多进程/写入后易脏读 | 未做 |
| 9 | P2 | 任务定义统一 | ✅ 合理 | 已做 |
| 10 | P3 | switch destination | ✅ 产品向，独立 PR | 未做 |
| — | P1 补充 | xcbeautify 未安装警告 | ✅ 与 #7 互补 | 已做 |

## 分项说明

### #3 增量构建 — 建议暂缓或改为 opt-in

- `xcactivitylog` 时间与源码 mtime 比较**不能**覆盖：改 Build Settings、换 scheme、Pods、SPM、签名、资源仅改 Contents.json 等。
- 误跳过会导致 Zed 里「改了代码却不编译」的难查问题。
- 若要做：默认关闭，仅 `IOS_RUNNER_SKIP_IF_FRESH=1` 或 `build --skip-if-unchanged`，并记录上次成功 build 的 scheme/configuration 哈希。

### #4 实时输出 — 不必改

`run_command` 已使用 `stdout/stderr: inherit()`，无 xcbeautify 时本就是行缓冲实时输出。提案中的 pipe + 线程反而可能乱序、丢 exit 关联。

### #6 构建诊断 — 建议下一版

与现有真机签名 `bail!` 提示合并即可；日志目录 `~/.ios-runner/logs/` 有价值，宜在捕获 stderr 的 piped 路径（xcbeautify）里统一落盘。

### #8 配置缓存 — 谨慎

MCP 同进程连续调用才受益；Zed 多任务并行 + 文件锁写入后，5s TTL 可能读到旧 destination。更稳妥：`save_config` 后进程内 invalidate，或不做缓存。

### #10 switch — 独立功能

`recent_destinations` + CLI 交互合理，但牵涉 `global_store` schema、Zed 任务、文档，适合单独发版。

## 验证

```bash
cd crates && cargo test --workspace && cargo clippy --workspace -- -D warnings
ios-runner doctor
ios-runner install-zed-tasks   # 任务列表应与改前一致
```
