#!/bin/bash
# Push to GitLab (origin) with input.txt included
# Force add input.txt (it's in .gitignore)
git add -f src/01/input.txt
# Commit if there are changes
if [ -n "$(git status --porcelain src/01/input.txt)" ]; then
    git commit -m "Update input files" 2>/dev/null || true
fi
git push origin

