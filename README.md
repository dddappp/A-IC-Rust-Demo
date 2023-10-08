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
    metadata:
      # The following two lines indicate that the tool should automatically generate the Create and Update methods,
      # but not the Delete method.
      Preprocessors: [ "MOVE_CRUD_IT" ]
      CRUD_IT_NO_DELETE: true
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
      UpdateBody:
        event:
          name: ArticleBodyUpdated
        parameters:
          Body:
            type: String

#      Create:
#        isCreationCommand: true
#        event:
#          name: ArticleCreated
#        parameters:
#          Title:
#            type: String
#          Body:
#            type: String
#      Update:
#        event:
#          name: ArticleUpdated
#        parameters:
#          Title:
#            type: String
#          Body:
#            type: String
```

You may have noticed the comments in the code above. 
If the "business logic" you need is CRUD, you don't actually need to write it. 
You can specify the tool to automatically generate it.

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

> **Tip**
> 
> Since the dddappp v0.0.1 image is updated frequently, 
> if you have run this image before, 
> you may be required to manually delete the image and pull it again before `docker run`.
> You can use the shell commands in the "Tips" section below to do this.
> 


### Implementing Business Logic

Open file `src/a_ic_rust_demo_backend/src/article_update_body_logic.rs`, and implement the business logic of the `UpdateBody` method.

What you need to do is actually fill in the contents of the `verify` and `mutate` functions.:

```rust
pub(crate) fn verify(
    body: String,
    article: &Article,
) -> ArticleBodyUpdated {
    ArticleBodyUpdated {
        article_id: article.article_id,
        version: article.version,
        body,
    }
}

pub(crate) fn mutate(
    article_body_updated: &ArticleBodyUpdated,
    article: Article,
) -> Article {
    let mut article = article.clone();
    article.body = article_body_updated.body.clone();
    article
}
```

If you open the files `src/a_ic_rust_demo_backend/src/article_create_logic.rs` and `src/a_ic_rust_demo_backend/src/article_update_logic.rs`, 
you will find that the tool has generated the business logic for the `Create` and `Update` methods.

Now you can build and deploy your backend canister.


## Test the project locally

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

Use the following commands to clean up old containers re-pull the image.

```shell
# If you have already run it, you may need to Clean Up Exited Docker Containers first
docker rm $(docker ps -aq --filter "ancestor=wubuku/dddappp:0.0.1")
# remove the image
docker image rm wubuku/dddappp:0.0.1
# pull the image
git pull wubuku/dddappp:0.0.1
```

## Move Examples

### Blog Example for Rooch

Here is a Rooch version blog example: https://github.com/rooch-network/rooch/blob/main/examples/blog/README.md

Rooch's Getting Started article on developing a simple Blog (in a non-low-code/write-all-the-code-manually way):
https://rooch.network/zh-CN/docs/getting-started#41-创建-move-项目

The code to accompany the article is here: https://github.com/rooch-network/rooch/tree/main/examples/simple_blog

This example is actually a modification of our "Developing a blog using a low-code approach" example.
Specifically, they removed "Comment " which is an "Aggregate Internal Entity", and only kept "Article" which is the "Aggregate Root Entity ". (It doesn't matter if you can't understand the DDD concept of "aggregate " or something like that ...)
The reason for deleting the "Comments" entity, as I understand it, is probably because if they were to explain how to manually code the functions of adding comments/updating comments/deleting comments, then this "introductory" article would be too long and would scare away the "newbies". 

The code of our "Low-code developing a Rooch-based Blog Example" is also available in the Rooch official repository: https://github.com/rooch-network/rooch/blob/main/examples/blog

You may have noticed that the low-code version of the example project is "officially" called "blog", while the "Getting Started" article version is called "simple blog".

As you can see from the names, the version we developed using the low-code approach is more complex; however, the developer has a lot less to do.

### A More Complex Sui Demo

If you are interested, you can find a more complex Sui Demo here: ["A Sui Demo"](https://github.com/dddappp/A-Sui-Demo).
