# Github Submodule Hook

This service provide an API to update submodules to a specific SHA on a repository.



### Configuration

The whole configuration is defined in the configuration file.
The file is in JSON-format.

It can be passed to the program with the `-c` option, otherwise, it will check the following places:

* local file named `config.json`
* (TODO) Environment variable `GITHUB_SUBMODULE_HOOK_CONFIG`
* (TODO) `~/.github_submodule_hook/config.json`
* (TODO) `/etc/github_submodule_hook`
* (TODO) file `config.json` in the same directory as the executable





**config.json**

```json
{
    "user_file": "users.txt",   // Optional: The file that contains the mapping "user = token"
    "token": "mytoken",         // The token to access the github API (need enough permission)
    "permissions": {            // Permisson tree: you give, for each user, access to different repository
        "user1": {
            "owner": {
                "repo": {
                    "branch": [
                        "submodule1"
                    ]
                }
            }
        }
    } 
}
```

I choose to use a tree `owner -> repo -> branch -> submodule` for simplicity when we have for example only 1 owner but many repositories.
I also wanted a file that can be manually edited



**users.txt** (or the name you choose to use)

use the CLI to add them:

```bash
github_submodule_hook config user add user1
```

Nb: the file contains 1 entry by line in the following format

```yaml
{username} = {base64(sha512(token))}
```

You could generate your own token if you want but this is strongly discouraged.



### Usage

user1 can now do the following query

```bash
curl -X POST localhost:8000/update/<owner>/<repo>/<branch>/<submodule>/<hash>?token?abcd
```



### Choices

### Token

* UUID4: This is random and non deterministic, the size is great too.
* SHA512: We don't need any password-specialized hash algorithm:
  * The entropy of the token is good (which is not the case for human password), we don't need salt
  * We don't need slow-by-design algorithm because of the number of possible values.



### Misc

For the CLI, I used `clap` with declaration. I needed to configure cargo

```bash
cargo add clap --features derive
```

