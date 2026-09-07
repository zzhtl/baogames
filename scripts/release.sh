#!/usr/bin/env bash
#
# BaoGames · 发版脚本
# 对齐 Cargo.toml 版本号 → 打注解 tag → 推送，剩下的交给
# .github/workflows/release.yml 构建 linux / windows / macOS 三平台产物并建 Release。
#
# 用法：
#   bash scripts/release.sh v0.2.0             # 正式发版
#   bash scripts/release.sh v0.2.0 --dry-run   # 只打印要做什么，不改仓库
#   bash scripts/release.sh v0.2.0 --yes       # 跳过确认

set -euo pipefail

TAG=""
DRY_RUN=0
ASSUME_YES=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run) DRY_RUN=1; shift ;;
    --yes|-y)  ASSUME_YES=1; shift ;;
    -h|--help) sed -n '2,10p' "$0"; exit 0 ;;
    -*) echo "未知参数: $1（-h 查看用法）" >&2; exit 1 ;;
    *)  TAG="$1"; shift ;;
  esac
done

if [[ -z "$TAG" ]]; then
  echo "缺少版本号，例如: bash scripts/release.sh v0.2.0" >&2
  exit 1
fi
if [[ ! "$TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
  echo "版本号必须形如 v0.2.0 或 v0.2.0-rc.1，收到: $TAG" >&2
  exit 1
fi
VER="${TAG#v}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

# ---- 前置检查 ----
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "工作区有未提交改动，先提交或 stash 再发版。" >&2
  git status --short >&2
  exit 1
fi

if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  echo "本地已存在 tag ${TAG}。" >&2
  exit 1
fi
if git ls-remote --exit-code --tags origin "refs/tags/$TAG" >/dev/null 2>&1; then
  echo "远端已存在 tag ${TAG}。" >&2
  exit 1
fi

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
CUR_VER="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n1)"

echo "==> 发版计划"
echo "    分支      : $BRANCH"
echo "    当前版本  : $CUR_VER"
echo "    目标版本  : $VER  (tag $TAG)"
if [[ "$CUR_VER" != "$VER" ]]; then
  echo "    动作      : 改写 Cargo.toml/Cargo.lock 版本 → commit → tag → push"
else
  echo "    动作      : 版本已一致 → tag → push"
fi

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo "（--dry-run，未做任何改动）"
  exit 0
fi

if [[ "$ASSUME_YES" -ne 1 ]]; then
  read -r -p "确认执行？会推送 $BRANCH 与 $TAG 到 origin [y/N] " ans
  [[ "$ans" == "y" || "$ans" == "Y" ]] || { echo "已取消。"; exit 1; }
fi

# ---- 对齐版本号 ----
if [[ "$CUR_VER" != "$VER" ]]; then
  tmp="$(mktemp)"
  # 只改 [package] 段第一处 version，别碰依赖的 version。
  awk -v v="$VER" '!done && /^version = "/ { print "version = \"" v "\""; done=1; next } { print }' \
    Cargo.toml > "$tmp"
  mv "$tmp" Cargo.toml
  # 让 Cargo.lock 里本包的 version 跟着更新。
  cargo metadata --format-version 1 >/dev/null
  git add Cargo.toml Cargo.lock
  git commit -m "chore(release): $TAG"
fi

# ---- 打标签并推送 ----
git tag -a "$TAG" -m "BaoGames $TAG"
git push origin "$BRANCH"
git push origin "$TAG"

echo ""
echo "✅ 已推送 ${TAG}，CI 开始构建三平台产物："
REPO_URL="$(git remote get-url origin)"
REPO_PATH="${REPO_URL#*github.com[:/]}"
echo "   https://github.com/${REPO_PATH%.git}/actions"
