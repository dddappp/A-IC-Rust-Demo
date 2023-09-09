# a_ic_rust_demo

This is a proof of concept for developing IC contracts using low-code tools.

```shell
dfx new --type=rust a_ic_rust_demo
cd a_ic_rust_demo
mkdir dddml
```

Add dependencies to the file `src/a_ic_rust_demo_backend/Cargo.toml`:

```toml
[dependencies]
candid = "0.8"
ic-cdk = "0.7"
ic-cdk-timers = "0.1" # Feel free to remove this dependency if you don't need timers
# Add the following two lines
ic-stable-structures = "0.5.4"
serde = "1.0"
```

## Programming

### Write DDDML Model File

In the `dddml` directory in the root of the repository, create a DDDML file `artilce.yaml` like this:

```yaml
aggregates:
  Article:
    id:
      name: ArticleId
      type: u128
    properties:
      Title:
        type: String
        length: 200
      Body:
        type: String
        length: 1500

    methods:
      Create:
        isCreationCommand: true
        event:
          name: ArticleCreated
        parameters:
          Title:
            type: String
          Body:
            type: String

      Update:
        event:
          name: ArticleUpdated
        parameters:
          Title:
            type: String
          Body:
            type: String
```

### Run dddappp Project Creation Tool

In repository root directory, run:

```shell
docker run \
-v .:/myapp \
wubuku/dddappp:0.0.1 \
--dddmlDirectoryPath /myapp/dddml \
--boundedContextName Test.AICRustDemo \
--icRustProjectDirectoryPath /myapp/src/a_ic_rust_demo_backend \
--icRustCanisterName a_ic_rust_demo_backend
```


The command parameters above are straightforward:

* The first line indicates mounting your local directory into the `/myapp` directory inside the container.
* `dddmlDirectoryPath` is the directory where DDDML model files are located. It should be a readable directory path in the container.
* Interpret the value of parameter `boundedContextName` as the name of your application you want to develop. For this example, this parameter is not currently useful, but must be provided to the CLI. When there are multiple parts in your name, separate them with dots and use PascalCase naming style for each part. Bounded-context is a term in Domain-driven design (DDD) that refers to a specific problem domain scope that contains specific business boundaries, constraints, and language. If you don't understand this concept for now, it's not a big deal.
* `icRustProjectDirectoryPath` is directory path where IC Rust on-chain contract code is placed. It should be a readable and writable directory path in container.
* `icRustCanisterName` is name of IC Rust backend canister. It's recommended to use snake_case naming style. Currently the tool will also use it as the name of the `.did` file.


### Implementing Business Logic

It **should be noted** that in the Move version of our tool, the CURD business logic can be generated automatically and does **not** require you to write it manually as below.

Open file `src/a_ic_rust_demo_backend/src/article_create_logic.rs`, and implement the business logic of the `Create` method.
What you need to do is actually fill in the contents of the `verify` and `mutate` functions.:

```rust
pub(crate) fn verify(
    article_id: u128,
    title: String,
    body: String,
) -> ArticleCreated {
    ArticleCreated { article_id, title, body }
}

pub(crate) fn mutate(
    article_created: &ArticleCreated,
) -> Article {
    Article {
        article_id: article_created.article_id,
        version: 0,
        title: article_created.title.clone(),
        body: article_created.body.clone(),
    }
}
```

Open file `src/a_ic_rust_demo_backend/src/article_update_logic.rs`, and implement the business logic of the `Update` method.

```rust

pub(crate) fn verify(
    title: String,
    body: String,
    article: &Article,
) -> ArticleUpdated {
    ArticleUpdated {
        article_id: article.article_id,
        version: article.version,
        title,
        body,
    }
}

pub(crate) fn mutate(
    article_updated: &ArticleUpdated,
    article: &Article,
) -> Article {
    let mut article = article.clone();
    article.title = article_updated.title.clone();
    article.body = article_updated.body.clone();
    article
}
```

Now you can build and deploy your backend canister.


## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

dfx deploy a_ic_rust_demo_backend
```

### Tests

Now you can use the dfx command line tool for testing:

```shell
dfx canister call a_ic_rust_demo_backend create '(1, "Hello", "World")'
dfx canister call a_ic_rust_demo_backend create '(2, "Foo", "Bar")'
dfx canister call a_ic_rust_demo_backend update '(1, "Hello", "World!")'
dfx canister call a_ic_rust_demo_backend get '(1)'
dfx canister call a_ic_rust_demo_backend get '(2)'
dfx canister call a_ic_rust_demo_backend getEvent '(0)'
dfx canister call a_ic_rust_demo_backend getEvent '(2)'
```

## Tips

This example is simple and has very limited support for the DDDML specification.

But with it, we can believe that if the IC version of the tool has the features that the Move version already has, 
it can be an amazing improvement in development efficiency when developing certain applications.

### Update dddappp Docker Image

Since the dddappp v0.0.1 image is updated frequently, you may be required to manually delete the image and pull it again before `docker run`.

```shell
# If you have already run it, you may need to Clean Up Exited Docker Containers first
docker rm $(docker ps -aq --filter "ancestor=wubuku/dddappp:0.0.1")
# remove the image
docker image rm wubuku/dddappp:0.0.1
# pull the image
git pull wubuku/dddappp:0.0.1
```


