#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod action {
    use clap::ArgMatches;
    pub mod add_user {
        use clap::FromArgMatches;
        use super::{Action, ActionFactory};
        use crate::args::add_user::AddUserArgs;
        use crate::cmd::add_user::AddUserCmd;
        use crate::cmd::CommandArgExt;
        pub struct AddUserAction {
            args: AddUserArgs,
        }
        impl AddUserAction {
            pub fn new(args: AddUserArgs) -> Self {
                Self { args }
            }
        }
        impl ActionFactory for AddUserCmd {
            type Dependency = ();
            fn from_arg_matches(
                matches: &clap::ArgMatches,
                _dependency: Self::Dependency,
            ) -> anyhow::Result<Box<dyn Action>> {
                let cmd = <Self as FromArgMatches>::from_arg_matches(matches)?;
                Ok(cmd.action(_dependency))
            }
            fn action(
                &self,
                _dependency: Self::Dependency,
            ) -> Box<dyn crate::action::Action> {
                Box::new(AddUserAction::new(self.args()))
            }
        }
        impl Action for AddUserAction {
            #[allow(
                elided_named_lifetimes,
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::needless_arbitrary_self_type,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn execute<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        #[allow(unreachable_code)] return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        {
                            ::std::io::_print(
                                format_args!("call add-user cmd: args {0:?}\n", __self.args),
                            );
                        };
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod help {
        use crate::action::Action;
        use clap::Command;
        pub struct Help {
            cmd: Command,
        }
        impl Help {
            pub fn new(cmd: Command) -> Self {
                Self { cmd }
            }
        }
        impl Action for Help {
            #[allow(
                elided_named_lifetimes,
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::needless_arbitrary_self_type,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn execute<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        #[allow(unreachable_code)] return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        __self.cmd.clone().print_long_help()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub trait ActionFactory {
        type Dependency;
        fn from_arg_matches(
            matches: &ArgMatches,
            dependency: Self::Dependency,
        ) -> anyhow::Result<Box<dyn Action>>;
        fn action(&self, dependency: Self::Dependency) -> Box<dyn Action>;
    }
    pub trait Action {
        #[must_use]
        #[allow(
            elided_named_lifetimes,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds
        )]
        fn execute<'life0, 'async_trait>(
            &'life0 self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
}
pub mod app {
    use crate::action::{help::Help, Action};
    use clap::{ArgMatches, Command};
    pub mod sample {
        use clap::{ArgMatches, Command, CommandFactory};
        use crate::{
            action::{Action, ActionFactory},
            cmd::add_user::AddUserCmd,
        };
        use super::{ApplicationCLIBuilder, CLIParser};
        pub struct SampleApplication {}
        impl SampleApplication {
            pub fn new() -> Self {
                Self {}
            }
            pub async fn execute(&mut self) -> anyhow::Result<()> {
                let action = self.parse_arg()?;
                action.execute().await
            }
        }
        impl ApplicationCLIBuilder for SampleApplication {
            fn cli() -> clap::Command {
                Command::new("sample").about("sample application cli")
            }
            fn sub_clis() -> impl IntoIterator<Item = Command> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([AddUserCmd::command()]),
                )
            }
        }
        impl CLIParser for SampleApplication {
            fn parse_arg(&mut self) -> anyhow::Result<Box<dyn crate::action::Action>> {
                let matches = self.get_matches()?;
                let Some((cmd_name, sub_matches)) = matches.subcommand() else {
                    return Ok(Box::new(<Self as CLIParser>::print_help()));
                };
                let action = self.action_map(cmd_name, sub_matches)?;
                Ok(action)
            }
            fn action_map(
                &self,
                cmd_name: &str,
                matches: &ArgMatches,
            ) -> anyhow::Result<Box<dyn Action>> {
                let action = match cmd_name {
                    "adduser" => {
                        <AddUserCmd as ActionFactory>::from_arg_matches(matches, ())?
                    }
                    _ => {
                        return ::anyhow::__private::Err(
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("unknow subcommand: {0:?}", cmd_name),
                                    );
                                    res
                                }),
                            ),
                        );
                    }
                };
                Ok(action)
            }
        }
    }
    /// ApplicationCLIBuilder
    ///
    /// Application のコマンドをビルドするためのトレイト
    ///
    /// ## サンプル
    ///
    /// ```ignore
    /// struct SampleApp
    ///
    /// impl ApplicationCLIBuilder for SampleApp {
    ///     fn build() -> Command {
    ///         // ビルドメソッドの実装を上書きしたい場合はここで実装する
    ///     }
    ///     fn cli() -> Command {
    ///         // 以下は最低限の実装
    ///         // 必要であれば情報を追加する
    ///         clap::Command::new("sample")
    ///     }
    ///     fn sub_clis() -> impl IntoIterator<Item = Command> {
    ///         // SubCommand1, SubCommand2 は clap::CommandFactory トレイトを実装している
    ///         vec![
    ///             SubCommand1::command(),
    ///             SubCommand2::command(),
    ///         ]
    ///     }
    /// }
    /// ```
    pub trait ApplicationCLIBuilder {
        fn build() -> Command {
            Self::cli().subcommands(Self::sub_clis())
        }
        /// アプリケーションのメインコマンドを定義する場所
        fn cli() -> Command;
        /// サブコマンドを定義する
        fn sub_clis() -> impl IntoIterator<Item = Command>;
    }
    pub trait CLIParser: ApplicationCLIBuilder {
        fn parse_arg(&mut self) -> anyhow::Result<Box<dyn Action>>;
        fn action_map(
            &self,
            cmd_name: &str,
            matches: &ArgMatches,
        ) -> anyhow::Result<Box<dyn Action>>;
        fn get_matches(&mut self) -> anyhow::Result<ArgMatches> {
            Ok(<Self as ApplicationCLIBuilder>::build().try_get_matches()?)
        }
        fn print_help() -> Help {
            Help::new(<Self as ApplicationCLIBuilder>::build())
        }
    }
}
pub mod args {
    pub mod add_user {
        use clap::Args;
        pub struct AddUserArgs {
            hogehoge: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AddUserArgs {
            #[inline]
            fn clone(&self) -> AddUserArgs {
                AddUserArgs {
                    hogehoge: ::core::clone::Clone::clone(&self.hogehoge),
                }
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for AddUserArgs {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = AddUserArgs {
                    hogehoge: __clap_arg_matches
                        .remove_one::<String>("hogehoge")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: hogehoge",
                        ))?,
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("hogehoge") {
                    #[allow(non_snake_case)]
                    let hogehoge = &mut self.hogehoge;
                    *hogehoge = __clap_arg_matches
                        .remove_one::<String>("hogehoge")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: hogehoge",
                        ))?;
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for AddUserArgs {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("AddUserArgs"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("AddUserArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [
                                        clap::Id::from("hogehoge"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("hogehoge")
                                .value_name("HOGEHOGE")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("AddUserArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [
                                        clap::Id::from("hogehoge"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("hogehoge")
                                .value_name("HOGEHOGE")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg;
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddUserArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AddUserArgs",
                    "hogehoge",
                    &&self.hogehoge,
                )
            }
        }
    }
}
pub mod cmd {
    pub mod add_user {
        use crate::args::add_user::AddUserArgs;
        use super::CommandArgExt;
        #[command(name = "adduser")]
        pub struct AddUserCmd {
            #[command(flatten)]
            args: AddUserArgs,
        }
        #[automatically_derived]
        #[allow(unused_qualifications, clippy::redundant_locals)]
        impl clap::Parser for AddUserCmd {}
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::CommandFactory for AddUserCmd {
            fn command<'b>() -> clap::Command {
                let __clap_app = clap::Command::new("adduser");
                <Self as clap::Args>::augment_args(__clap_app)
            }
            fn command_for_update<'b>() -> clap::Command {
                let __clap_app = clap::Command::new("adduser");
                <Self as clap::Args>::augment_args_for_update(__clap_app)
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for AddUserCmd {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = AddUserCmd {
                    args: <AddUserArgs as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?,
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                {
                    #[allow(non_snake_case)]
                    let args = &mut self.args;
                    <AddUserArgs as clap::FromArgMatches>::update_from_arg_matches_mut(
                        args,
                        __clap_arg_matches,
                    )?;
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for AddUserCmd {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("AddUserCmd"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("AddUserCmd")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0] = [];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app;
                    let __clap_app = <AddUserArgs as clap::Args>::augment_args(
                        __clap_app,
                    );
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("AddUserCmd")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0] = [];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app;
                    let __clap_app = <AddUserArgs as clap::Args>::augment_args_for_update(
                        __clap_app,
                    );
                    __clap_app
                }
            }
        }
        impl CommandArgExt for AddUserCmd {
            type Args = AddUserArgs;
            fn args(&self) -> Self::Args {
                self.args.clone()
            }
        }
    }
    pub trait CommandArgExt {
        type Args;
        fn args(&self) -> Self::Args;
    }
}
