#!/usr/bin/env bash
set -euo pipefail

ENV_GLOB="/home/mhugo/.env*"
ENV_FILE="/home/mhugo/.env"
BACKUP_DIR="/home/mhugo/.env_backups_$(date +%s)"
REPO_ROOT="$(pwd)"

echo "Backing up any existing files matching ${ENV_GLOB} to ${BACKUP_DIR}"
mkdir -p "${BACKUP_DIR}"
shopt -s nullglob
for f in ${ENV_GLOB}; do
  cp -- "$f" "${BACKUP_DIR}/$(basename "$f")"
done
shopt -u nullglob

echo
echo "Current GITHUB_TOKEN entries (files:line):"
grep -Hn '^GITHUB_TOKEN=' /home/mhugo/.env* 2>/dev/null || echo "  (none found)"
echo

echo "NOTE: If your home dotfiles are synced to a gist or backup service, DO NOT write tokens to files that will be uploaded."
if [ -L "$ENV_FILE" ]; then
  echo "- Warning: ${ENV_FILE} is a symlink (possible linked backup)."
fi
if realpath "$ENV_FILE" 2>/dev/null | grep -q "${REPO_ROOT}"; then
  echo "- Warning: ${ENV_FILE} appears inside the repository root (${REPO_ROOT}). This may be backed up to gist."
fi
echo

read -rp "Remove existing GITHUB_TOKEN lines from all /home/mhugo/.env* files? [y/N]: " remove
remove=${remove:-N}
if [[ "${remove,,}" == "y" ]]; then
  shopt -s nullglob
  for f in ${ENV_GLOB}; do
    [ -e "$f" ] || continue
    sed -i.bak '/^GITHUB_TOKEN=/d' -- "$f" || true
  done
  shopt -u nullglob
  echo "Removed GITHUB_TOKEN lines; original files backed up with .bak alongside originals."
fi

read -rp "Choose how to proceed - write token to file (w), use gh interactive login without saving (g), or skip (s)? [s/w/g]: " action
action=${action:-s}
if [[ "${action,,}" == "w" ]]; then
  echo
  echo "IMPORTANT: Paste the token when prompted. It will not be shown on screen."
  read -rsp "Enter new token: " token
  echo
  if [[ -z "${token// /}" ]]; then
    echo "No token entered; aborting write."
    exit 1
  fi
  # If ENV_FILE appears to be under the repo (likely backed up), warn and confirm
  if realpath "$ENV_FILE" 2>/dev/null | grep -q "${REPO_ROOT}"; then
    read -rp "${ENV_FILE} appears to be inside the repo (may be backed up). Are you sure you want to write there? [y/N]: " confirm
    confirm=${confirm:-N}
    if [[ "${confirm,,}" != "y" ]]; then
      echo "Aborting write to avoid leaking token to backups."
      exit 1
    fi
  fi
  printf 'GITHUB_TOKEN=%s\n' "$token" > "$ENV_FILE"
  chmod 600 "$ENV_FILE"
  echo "Wrote token to ${ENV_FILE} (permissions set to 600)."
  echo "To load it in your current shell: run 'source ${ENV_FILE}' or 'export \\$(cat ${ENV_FILE})'"
elif [[ "${action,,}" == "g" ]]; then
  echo
  echo "You chose gh interactive login without saving the token to disk. Paste the token when prompted."
  read -rsp "Enter token to use for 'gh auth login --with-token': " token
  echo
  if [[ -z "${token// /}" ]]; then
    echo "No token entered; aborting."
    exit 1
  fi
  # Use gh to login with token via stdin
  if ! command -v gh >/dev/null 2>&1; then
    echo "gh CLI not found in PATH. Install gh before using this option."
    exit 1
  fi
  echo "$token" | gh auth login --with-token
  echo "Logged in with gh using provided token (token not saved to ${ENV_FILE})."
else
  echo "No token written or set."
fi

echo
echo "Next steps:"
echo "- Verify with: gh auth status"
echo "- If you prefer to use your interactive gh login instead of a token, run: unset GITHUB_TOKEN"
echo
echo "Backups saved in: ${BACKUP_DIR}"

exit 0
