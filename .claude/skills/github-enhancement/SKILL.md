---
name: github-enhancement
description: Automates the end-to-end process of pulling a GitHub issue and implementing a solution.
allowed-tools:
  - Write
  - Read
  - Edit
  - Bash
arguments:
  - issue
argument-hint: "<url-to-github-issue> or <github-issue-number>"
---

# Instructions
Attempt to resolve GitHub issue: $issue.

## Step 0: Check GitHub connection
1. **Authenticate:**
  ```bash
  gh auth status
  ```
2. **Guard condition:** If **not** authenticated:
  - Print a message describing the issue.
  - Stop execution immediately and do not proceed to subsequent steps.

## Step 1: Validate issue type then extract components
1. **Fetch the issue:** Fetch the issue details:
  ```bash
  gh issue view $issue
  ```
2. **Check issue type:** Check if the label corresponds to an enhancement (i.e. `Labels: enhancement`)
3. **Guard condition:** If the issue is **not** of type `enhancement`:
  - Print a clear message to the user indicating that the issue is not an enhancement.
  - Stop execution immediately and do not proceed to subsequent steps.
4. **Extract issue components:** The issue may contain a **Background**, **Task**, and **Where to look** section.

## Step 2: Implement the solution
1. **Isolate development:** Create a dedicated feature branch using a semantic naming convention:
  ```bash
  git checkout -b feature/<issue-number>-<short-description>
  ```
2. **Understand the task:**
  - The issue may contain a **Background** section. Use this as contextual information, **and**,
  - The issue may contain a **Task** section. Use this to determine what needs to be done, **otherwise**,
  - Infer what needs to be done from the issue description.
3. **Locate code:** Try the following:
  - The issue may contain a **Where to look** section. This is where you should operate.
  - If the **Where to look** section does not exist, you will need to decide where to implement the solution.
3. **Implement fix:** Apply the minimum required changes to implement the solution. Avoid unrelated code refactoring or cosmetic modifications.
4. **Commit changes:** Stage your minimal, relevant edits and commit with a clear, concise `git` message matching the project convention:
  ```bash
  git add .
  git commit -m "claude: solution for issue #<issue-number>"
  ```
