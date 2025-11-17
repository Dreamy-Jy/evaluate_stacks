# JJ Issue Write-up

JJ's does not show logs prior to a pushed remote bookmark. I want to change this.

This seems like intended behavior and I want to add an option to change that.

---

How do I start jj with a main bookmark, and have it automatically propagate with new commit?

- there are local and remote bookmarks
- there are tracked bookmarks, but this is not the default

When working with remote bookmarks jj, will not show commits below the committed bookmark

- read the code to figure this out
- submit a patch on github

Sources

- https://jj-vcs.github.io/jj/latest/bookmarks/#automatic-tracking-of-bookmarks-gitauto-local-bookmark-option
