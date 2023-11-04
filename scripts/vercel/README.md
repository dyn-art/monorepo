# `.sh` scripts for Vercel deployment


## 🔴 Issues
- [unexpected ERR_PNPM_OUTDATED_LOCKFILE error](https://github.com/vercel/vercel/issues/8272)
- [Shell Script Execution Issue in PNPM Monorepo on Vercel](https://github.com/vercel/vercel/discussions/10797)
- [Can a shell script set environment variables of the calling shell?](https://stackoverflow.com/questions/496702/can-a-shell-script-set-environment-variables-of-the-calling-shell)

## 👨‍🏫 Learnings
- [Pass all variables from one shell script to another?](https://stackoverflow.com/questions/9772036/pass-all-variables-from-one-shell-script-to-another)
- [How can I set the current working directory to the directory of the script in Bash?](https://stackoverflow.com/questions/3349105/how-can-i-set-the-current-working-directory-to-the-directory-of-the-script-in-ba)

### Environment variables
Setting environment variables within a script will only make them available to that script and its child processes during that session. Once the script terminates, any environment variables it set will disappear unless they were exported to a parent shell.
