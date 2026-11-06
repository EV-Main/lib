#!/usr/bin/env bash
# Puts PR <n>'s review page at pr/<n>/index.html on the Pages branch, or removes
# it when no page is given. The branch is rewritten as one orphan commit each
# time, so the multi-MB pages never pile up in history.
set -euo pipefail
pr=$1
page=${2:-}
GIT_INDEX_FILE=$(mktemp -u)
export GIT_INDEX_FILE
trap 'rm -f "$GIT_INDEX_FILE"' EXIT
if git fetch -q origin gh-pages 2>/dev/null; then git read-tree FETCH_HEAD; else git read-tree --empty; fi
git rm -rq --cached --ignore-unmatch "pr/$pr"
put() { git update-index --add --cacheinfo "100644,$(git hash-object -w "$1"),$2"; }
put /dev/null .nojekyll
[[ -z $page ]] || put "$page" "pr/$pr/index.html"
git push -qf origin "$(git commit-tree "$(git write-tree)" -m "visual review pages")":refs/heads/gh-pages
