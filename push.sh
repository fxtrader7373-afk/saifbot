#!/bin/bash

# Saha Sniper Git Push Script

# 1. Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Saha Sniper update process...${NC}"

# 2. Check if .gitignore exists, if not create it to protect sensitive data
if [ ! -f .gitignore ]; then
    echo "target/" > .gitignore
    echo ".env" >> .gitignore
    echo "**/*.rs.bk" >> .gitignore
    echo "Cargo.lock" >> .gitignore
    echo -e "${GREEN}Created .gitignore to protect your .env file.${NC}"
fi

# 3. Git commands
git add .

# Check if a commit message was provided
if [ -z "$1" ]; then
    COMMIT_MSG="Update Saha Sniper: $(date +'%Y-%m-%d %H:%M:%S')"
else
    COMMIT_MSG="$1"
fi

git commit -m "$COMMIT_MSG"

# 4. Push to current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo -e "${GREEN}Pushing to branch: $BRANCH...${NC}"
git push origin "$BRANCH"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Successfully pushed updates to Git!${NC}"
else
    echo -e "${RED}Push failed. Check your internet connection or repository permissions.${NC}"
fi
