
# Getting information on Cairo developers

We will have a `struct`, `starker`, with items `login: String`, `contributions: u64`, `followers: u64`. 

We export to an excel file with this information in a matrix.

## Steps

### 1. Get all Cairo repos.

Use `https://api.github.com/search/repositories?q=language:cairo&order=desc` call to get all available repos.

We extract:
* `total_count`: total count of repos.
* `items`: repos.

### 2. Get the full name to each repo.

Make array with:
* `full_name`: e.g., `"sayajin-labs/kakarot"`.

### 3. Get all contributors to each Cairo repo.

Use `https://api.github.com/repos/{full_name}/contributors` call to get all contributors. E.g., `https://api.github.com/repos/xJonathanLEI/starknet-rs/contributors`.

Get:
* `login`: e.g., `"xJonathanLEI"`.
* `contributions`: e.g, `186`.
  
### 4. Get more information on the developer.

Use `https://api.github.com/users/{login}`. E.g.,  `https://api.github.com/users/espejelomar`.

Get:
* `followers`: e.g, `47`.


### 5. Others
1. link del repo, 
2. el nombre de quien lo creo, 
3. el numero de estrellas, y 
4. idealmente - la lista de todos los que hicieron un commit
