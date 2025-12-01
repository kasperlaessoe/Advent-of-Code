# Push to GitLab (origin) with input.txt included
# Force add input.txt (it's in .gitignore)
git add -f src/01/input.txt
# Commit if there are changes
$status = git status --porcelain src/01/input.txt
if ($status) {
    git commit -m "Update input files" 2>$null
}
git push origin

