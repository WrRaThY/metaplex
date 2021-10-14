#!/bin/bash

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
echo "GIT_BRANCH: $BRANCH"
echo "VERCEL_ENV: $VERCEL_ENV"

if [[ "$VERCEL_ENV" == "production" ]] ; then
  # Proceed with the build
  echo "✅ - Production build can proceed"
  exit 1;

elif [[ $BRANCH == *"preview"* ]]; then
  echo "✅ - build from $BRANCH can proceed"
  exit 1;

elif [[ $BRANCH == *"public"* ]]; then
  echo "✅ - build from $BRANCH can proceed"
  exit 1;
else
  # Don't build
  echo "🛑 - Build cancelled"
  exit 0;
fi
