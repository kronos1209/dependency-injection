# Application

アプリケーションは以下の情報を持っている

- CLIアプリケーションのコマンドリスト
  - 今回はアプリケーションを構築した時点でコマンドのリストは変更できないこととする
- コマンド名から対応するアクションへのマップ
  - やりたいこととしては clap を利用してコマンド引数をパースして、
  コマンド名を取得して実行するアクションを返したい

```rust
struct Application {
    cmd: clap::Command;
}

impl Application {
    /// ここでCLIアプリケーションのコマンドリストを構築する
    fn clis() -> impl IntoIterator<Item=Command> {
        vec![
            SubCmd1::cli(),
            SubCmd2::cli(),
        ]
    }
    fn action_map() -> HashMap<TypeId, Box<dyn
}

struct ActionActivator {}
trait Acticator {
    fn activate(&self,args: ArgMatch) -> Box<dyn Action>
}

trait Action {
    fn execute() -> impl Future<Output = ()>;
}
```
