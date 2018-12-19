# Contributing

First of all, thank you for your interest in contributing.
For now, this file doesn't provide very much information, just some basic notes
that come to my mind right now. If you have any suggestions to make it better,
please send me a Merge Reqest!

**Note: please make PRs on [GitLab], not on the GitHub mirror.**

If you experience any bugs or have feature requests, please [file an issue].

[GitLab]: https://gitlab.com/torkleyy/nitric
[file an issue]: https://gitlab.com/torkleyy/nitric/issues/new

## Dealing with upstream changes

Please use rebase over merge, since the latter is bad for the commit history.
If you're new to git, here's how to do that:

```sh
git fetch upstream
```

Assuming `upstream` is the upstream repo, this will fetch the latest changes.

Use the following with care if you're new to Git; better make a backup!

```sh
git rebase upstream/master
```

This will try to re-apply your commits on top of the upstream changes. If there
are conflicts, you'll be asked to fix them; once done, add the changes with
`git add -A` and use `git rebase --continue`. Repeat until there are no more
conflicts.

That should be it. Note that you'll have to force-push to your branch in case
you have pushed before.

## Squashing commits

If you created more commits then intended, it can be a good idea to combine some
of your commits. Note that this, again, should be used with care if you don't
know what you're doing; better create a backup before!

```sh
git rebase --interactive HEAD~$num_commits # replace this
```

You just need to replace `num_commits` with the number of commits you want to
edit (use `git log` if unsure).

Now you can simply change some commits to `s` or `f` to merge them into the
above commits. Once done, you'll be asked for the new commit messages.

That should be it. Note that you'll have to force-push to your branch in case
you have pushed before.
