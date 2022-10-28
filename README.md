# Github Submodule Hook

This service provide an API to update submodules to a specific SHA on a repository.



### Configuration

**config.json**

```json
{
    "user_file": "users.txt",	// The file that contains the mapping "user = token"
    "token": "mytoken",			// The token to access the github API (need enough permission)
    "permissions": {			// Permisson tree: you give, for each user, access to different repository
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

```
user1 = abcd
```

You can define the tokens yourself or use the CLI to add them:

```bash
github_submodule_hook config user add user1
```



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

