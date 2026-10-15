---
name: github-resolve-issue
description: Automates the end-to-end process of pulling a GitHub issue, implementing a targeted solution, and opening a pull request.
allowed-tools:
  - Write
  - Read
  - Edit
  - Bash
argument-hints:
  issue_url: "The full URL or issue number of the GitHub issue to solve"
---

# Resolve GitHub Issue Skill

Use this skill whenever the user asks to "resolve issue #X," or provides a GitHub issue link. Follow this strict, professional, end-to-end software engineering workflow.

## Step 0: Validate issue type
1. **Authenticate & Fetch**: Ensure the GitHub CLI (`gh`) is authenticated. Fetch the issue details:
   ```bash
   gh issue view <issue_url>
   ```
2. **Check issue type:** Check if the label corresponds to an enhancement (i.e. `Labels: enhancement`)
3. **Guard Condition:** If the issue is **not** of type `enhancement`:
   - Print a clear message to the user: *"Aborting: Issue #<number> is not an enhancement (found type/label: <actual-type>)."*
   - Stop execution immediately and do not proceed to subsequent steps.

## Step 1: Analysis & environment setup
1. **Confirm Scope**: Parse the issue description and comments.
2. **Isolate Development**: Create a dedicated feature branch using a semantic naming convention (i.e. `claude/issue-<number>-<name>`):
   ```bash
   git checkout -b claude/issue-<number>-<name>
   ```

## Step 2: Fix implementation
1. **Locate Code**: Map out the files where the solution should be implemented if they are not specified in the issue description.
2. **Implement Fix**: Apply the minimum required changes to implement the solution. Avoid unrelated code refactoring or cosmetic modifications.
3. **Commit Changes**: Stage your minimal, relevant edits and commit with a clear, concise Git message matching project convention:
   ```bash
   git add .
   git commit -m "fix #<number>: <description>"
   ```
4. **Push Branch**: Push the local branch up to the remote repository.
5. **Submit PR**: Open a new Pull Request using the GitHub CLI, ensuring the body explicitly references the issue to close it automatically:
   ```bash
   gh pr create --title "fix: resolve issue #<number>" --body "Closes #<number>.\n"
   ```
