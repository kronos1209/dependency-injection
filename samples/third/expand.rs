#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod action {
    use clap::ArgMatches;
    pub mod add_product {
        use crate::{args::add_product::*, domain::service::product::ProductService};
        use super::Action;
        pub struct AddProductAction<'this> {
            args: CreateProductArgs,
            service: &'this dyn ProductService,
        }
        impl<'this> AddProductAction<'this> {
            pub fn new(
                args: CreateProductArgs,
                product_service: &'this dyn ProductService,
            ) -> Self {
                Self {
                    args,
                    service: product_service,
                }
            }
        }
        impl<'this> Action for AddProductAction<'this> {
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
                        let product = __self
                            .service
                            .create_product(
                                __self.args.name().to_string(),
                                __self.args.price(),
                            )
                            .await?;
                        if let Ok(Some(re)) = __self
                            .service
                            .get_product(product.id().clone())
                            .await
                        {
                            {
                                ::std::io::_print(format_args!("{0:?}\n", re));
                            };
                            match (&product, &re) {
                                (left_val, right_val) => {
                                    if !(*left_val == *right_val) {
                                        let kind = ::core::panicking::AssertKind::Eq;
                                        ::core::panicking::assert_failed(
                                            kind,
                                            &*left_val,
                                            &*right_val,
                                            ::core::option::Option::None,
                                        );
                                    }
                                }
                            };
                        } else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("failed to add product"),
                                );
                            }
                        };
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod add_user {
        use super::Action;
        use crate::args::add_user::CreateUserArgs;
        use crate::domain::service::user::UserService;
        pub struct AddUserAction<'this> {
            args: CreateUserArgs,
            user_service: &'this dyn UserService,
        }
        impl<'this> AddUserAction<'this> {
            pub fn new(
                args: CreateUserArgs,
                user_service: &'this dyn UserService,
            ) -> Self {
                Self { args, user_service }
            }
        }
        impl<'this> Action for AddUserAction<'this> {
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
                        let user = __self
                            .user_service
                            .create_user(__self.args.name().to_string())
                            .await?;
                        if let Ok(Some(re)) = __self
                            .user_service
                            .get_user(user.id().clone())
                            .await
                        {
                            {
                                ::std::io::_print(format_args!("{0:?}\n", re));
                            };
                            match (&user, &re) {
                                (left_val, right_val) => {
                                    if !(*left_val == *right_val) {
                                        let kind = ::core::panicking::AssertKind::Eq;
                                        ::core::panicking::assert_failed(
                                            kind,
                                            &*left_val,
                                            &*right_val,
                                            ::core::option::Option::None,
                                        );
                                    }
                                }
                            };
                        } else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("failed to add product"),
                                );
                            }
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
    pub mod config {
        use std::env;
        #[allow(dead_code)]
        pub struct AppConfig {
            connection_string: String,
        }
        impl AppConfig {
            pub fn from_env() -> anyhow::Result<Self> {
                Ok(Self {
                    connection_string: env::var("CONNECTION_STRING")
                        .map_or(String::new(), |var| var.to_string()),
                })
            }
        }
    }
    pub mod sample {
        use std::sync::Arc;
        use clap::{ArgMatches, Command, FromArgMatches};
        use crate::{
            action::Action, cmd::{product::ProductCmd, user::UserCmd},
            domain::service::{product::ProductService, user::UserService},
            infrastracture::{
                repository::{
                    product::MemoryProductRepository, user::MemoryUserRepository,
                },
                service::{product::ProductServiceImpl, user::UserServiceImpl},
            },
        };
        use super::{config::AppConfig, ApplicationCLIBuilder, CLIParser};
        pub struct SampleApplication {
            service_container: ServficeContiner,
        }
        struct ServficeContiner {
            user_service: Arc<dyn UserService + Send + Sync>,
            product_service: Arc<dyn ProductService + Send + Sync>,
        }
        impl SampleApplication {
            pub fn new(config: AppConfig) -> Self {
                Self {
                    service_container: Self::create_object_map(config),
                }
            }
            pub async fn execute(&mut self) -> anyhow::Result<()> {
                let action = self.parse_arg()?;
                action.execute().await
            }
            fn create_object_map(_config: AppConfig) -> ServficeContiner {
                ServficeContiner {
                    user_service: Arc::new(
                        UserServiceImpl::new(MemoryUserRepository::new()),
                    ),
                    product_service: Arc::new(
                        ProductServiceImpl::new(MemoryProductRepository::new()),
                    ),
                }
            }
        }
        impl ApplicationCLIBuilder for SampleApplication {
            fn cli() -> clap::Command {
                Command::new("sample").about("sample application cli")
            }
            fn sub_clis() -> impl IntoIterator<Item = Command> {
                ::alloc::vec::Vec::new()
            }
        }
        impl CLIParser for SampleApplication {
            fn parse_arg(
                &mut self,
            ) -> anyhow::Result<Box<dyn crate::action::Action + '_>> {
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
            ) -> anyhow::Result<Box<dyn Action + '_>> {
                let action: Box<dyn Action> = match cmd_name {
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
        fn parse_arg(&mut self) -> anyhow::Result<Box<dyn Action + '_>>;
        fn action_map(
            &self,
            cmd_name: &str,
            matches: &ArgMatches,
        ) -> anyhow::Result<Box<dyn Action + '_>>;
        fn get_matches(&mut self) -> anyhow::Result<ArgMatches> {
            Ok(<Self as ApplicationCLIBuilder>::build().try_get_matches()?)
        }
        fn print_help() -> Help {
            Help::new(<Self as ApplicationCLIBuilder>::build())
        }
    }
}
pub mod args {
    pub mod add_product {
        use clap::Args;
        pub struct CreateProductArgs {
            name: String,
            price: f64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateProductArgs {
            #[inline]
            fn clone(&self) -> CreateProductArgs {
                CreateProductArgs {
                    name: ::core::clone::Clone::clone(&self.name),
                    price: ::core::clone::Clone::clone(&self.price),
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
        impl clap::FromArgMatches for CreateProductArgs {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = CreateProductArgs {
                    name: __clap_arg_matches
                        .remove_one::<String>("name")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: name",
                        ))?,
                    price: __clap_arg_matches
                        .remove_one::<f64>("price")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: price",
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
                if __clap_arg_matches.contains_id("name") {
                    #[allow(non_snake_case)]
                    let name = &mut self.name;
                    *name = __clap_arg_matches
                        .remove_one::<String>("name")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: name",
                        ))?;
                }
                if __clap_arg_matches.contains_id("price") {
                    #[allow(non_snake_case)]
                    let price = &mut self.price;
                    *price = __clap_arg_matches
                        .remove_one::<f64>("price")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: price",
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
        impl clap::Args for CreateProductArgs {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("CreateProductArgs"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("CreateProductArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 2usize] = [
                                        clap::Id::from("name"),
                                        clap::Id::from("price"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
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
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("price")
                                .value_name("PRICE")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        f64,
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
                            clap::ArgGroup::new("CreateProductArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 2usize] = [
                                        clap::Id::from("name"),
                                        clap::Id::from("price"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
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
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("price")
                                .value_name("PRICE")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::impl_prelude::*;
                                    let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                        f64,
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
        impl ::core::fmt::Debug for CreateProductArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CreateProductArgs",
                    "name",
                    &self.name,
                    "price",
                    &&self.price,
                )
            }
        }
        impl CreateProductArgs {
            pub fn name(&self) -> &String {
                &self.name
            }
            pub fn price(&self) -> f64 {
                self.price
            }
        }
    }
    pub mod add_user {
        use clap::Args;
        pub struct CreateUserArgs {
            name: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CreateUserArgs {
            #[inline]
            fn clone(&self) -> CreateUserArgs {
                CreateUserArgs {
                    name: ::core::clone::Clone::clone(&self.name),
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
        impl clap::FromArgMatches for CreateUserArgs {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = CreateUserArgs {
                    name: __clap_arg_matches
                        .remove_one::<String>("name")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: name",
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
                if __clap_arg_matches.contains_id("name") {
                    #[allow(non_snake_case)]
                    let name = &mut self.name;
                    *name = __clap_arg_matches
                        .remove_one::<String>("name")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: name",
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
        impl clap::Args for CreateUserArgs {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("CreateUserArgs"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("CreateUserArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("name")];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
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
                            clap::ArgGroup::new("CreateUserArgs")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 1usize] = [clap::Id::from("name")];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("name")
                                .value_name("NAME")
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
        impl ::core::fmt::Debug for CreateUserArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CreateUserArgs",
                    "name",
                    &&self.name,
                )
            }
        }
        impl CreateUserArgs {
            pub fn name(&self) -> &String {
                &self.name
            }
        }
    }
}
pub mod cmd {
    pub mod product {
        use crate::args::add_product::CreateProductArgs;
        pub enum ProductCmd {
            Create { #[command(flatten)] args: CreateProductArgs },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductCmd {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ProductCmd::Create { args: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Create",
                            "args",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProductCmd {
            #[inline]
            fn clone(&self) -> ProductCmd {
                match self {
                    ProductCmd::Create { args: __self_0 } => {
                        ProductCmd::Create {
                            args: ::core::clone::Clone::clone(__self_0),
                        }
                    }
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
        impl clap::FromArgMatches for ProductCmd {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                    .remove_subcommand()
                {
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    if __clap_name == "create" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::Create {
                            args: <CreateProductArgs as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        });
                    }
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::InvalidSubcommand,
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "The subcommand \'{0}\' wasn\'t recognized",
                                        __clap_name,
                                    ),
                                );
                                res
                            }),
                        ),
                    )
                } else {
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::MissingSubcommand,
                            "A subcommand is required but one was not provided.",
                        ),
                    )
                }
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut<'b>(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                    match self {
                        Self::Create { args } if "create" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {
                                {
                                    <CreateProductArgs as clap::FromArgMatches>::update_from_arg_matches_mut(
                                        args,
                                        __clap_arg_matches,
                                    )?;
                                }
                            }
                        }
                        s => {
                            *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?;
                        }
                    }
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
        impl clap::Subcommand for ProductCmd {
            fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Create")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 0] = [];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand;
                            let __clap_subcommand = <CreateProductArgs as clap::Args>::augment_args(
                                __clap_subcommand,
                            );
                            __clap_subcommand
                        }
                    });
                __clap_app
            }
            fn augment_subcommands_for_update<'b>(
                __clap_app: clap::Command,
            ) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Create")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 0] = [];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand;
                            let __clap_subcommand = <CreateProductArgs as clap::Args>::augment_args_for_update(
                                __clap_subcommand,
                            );
                            __clap_subcommand
                        }
                    });
                __clap_app
            }
            fn has_subcommand(__clap_name: &str) -> bool {
                if "create" == __clap_name {
                    return true;
                }
                false
            }
        }
    }
    pub mod user {
        use crate::args::add_user::CreateUserArgs;
        #[command(name = "user")]
        pub enum UserCmd {
            Create { #[command(flatten)] args: CreateUserArgs },
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
        impl clap::FromArgMatches for UserCmd {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                    .remove_subcommand()
                {
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    if __clap_name == "create" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::Create {
                            args: <CreateUserArgs as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        });
                    }
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::InvalidSubcommand,
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "The subcommand \'{0}\' wasn\'t recognized",
                                        __clap_name,
                                    ),
                                );
                                res
                            }),
                        ),
                    )
                } else {
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::MissingSubcommand,
                            "A subcommand is required but one was not provided.",
                        ),
                    )
                }
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut<'b>(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                    match self {
                        Self::Create { args } if "create" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {
                                {
                                    <CreateUserArgs as clap::FromArgMatches>::update_from_arg_matches_mut(
                                        args,
                                        __clap_arg_matches,
                                    )?;
                                }
                            }
                        }
                        s => {
                            *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?;
                        }
                    }
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
        impl clap::Subcommand for UserCmd {
            fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Create")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 0] = [];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand;
                            let __clap_subcommand = <CreateUserArgs as clap::Args>::augment_args(
                                __clap_subcommand,
                            );
                            __clap_subcommand
                        }
                    });
                __clap_app
            }
            fn augment_subcommands_for_update<'b>(
                __clap_app: clap::Command,
            ) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Create")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 0] = [];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand;
                            let __clap_subcommand = <CreateUserArgs as clap::Args>::augment_args_for_update(
                                __clap_subcommand,
                            );
                            __clap_subcommand
                        }
                    });
                __clap_app
            }
            fn has_subcommand(__clap_name: &str) -> bool {
                if "create" == __clap_name {
                    return true;
                }
                false
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserCmd {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    UserCmd::Create { args: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Create",
                            "args",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UserCmd {
            #[inline]
            fn clone(&self) -> UserCmd {
                match self {
                    UserCmd::Create { args: __self_0 } => {
                        UserCmd::Create {
                            args: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
    }
    pub mod root {
        use super::{product::ProductCmd, user::UserCmd};
        pub struct RootCommand {
            #[command(subcommand)]
            user_command: UserCmd,
        }
        #[automatically_derived]
        #[allow(unused_qualifications, clippy::redundant_locals)]
        impl clap::Parser for RootCommand {}
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
        impl clap::CommandFactory for RootCommand {
            fn command<'b>() -> clap::Command {
                let __clap_app = clap::Command::new("console-application");
                <Self as clap::Args>::augment_args(__clap_app)
            }
            fn command_for_update<'b>() -> clap::Command {
                let __clap_app = clap::Command::new("console-application");
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
        impl clap::FromArgMatches for RootCommand {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = RootCommand {
                    user_command: {
                        <UserCmd as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?
                    },
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
                    let user_command = &mut self.user_command;
                    <UserCmd as clap::FromArgMatches>::update_from_arg_matches_mut(
                        user_command,
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
        impl clap::Args for RootCommand {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("RootCommand"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("RootCommand")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_app = <UserCmd as clap::Subcommand>::augment_subcommands(
                        __clap_app,
                    );
                    let __clap_app = __clap_app
                        .subcommand_required(true)
                        .arg_required_else_help(true);
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("RootCommand")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_app = <UserCmd as clap::Subcommand>::augment_subcommands(
                        __clap_app,
                    );
                    let __clap_app = __clap_app
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommand_required(false)
                        .arg_required_else_help(false);
                    __clap_app
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RootCommand {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "RootCommand",
                    "user_command",
                    &&self.user_command,
                )
            }
        }
    }
}
pub mod domain {
    pub mod model {
        pub mod product {
            use uuid::Uuid;
            pub struct Product {
                id: String,
                name: String,
                price: f64,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Product {
                #[inline]
                fn clone(&self) -> Product {
                    Product {
                        id: ::core::clone::Clone::clone(&self.id),
                        name: ::core::clone::Clone::clone(&self.name),
                        price: ::core::clone::Clone::clone(&self.price),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Product {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Product",
                        "id",
                        &self.id,
                        "name",
                        &self.name,
                        "price",
                        &&self.price,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Product {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Product {
                #[inline]
                fn eq(&self, other: &Product) -> bool {
                    self.id == other.id && self.name == other.name
                        && self.price == other.price
                }
            }
            impl Product {
                pub fn new(name: String, price: f64) -> Self {
                    Self {
                        id: Uuid::default().to_string(),
                        name,
                        price,
                    }
                }
                pub fn id(&self) -> &String {
                    &self.id
                }
            }
        }
        pub mod user {
            use uuid::Uuid;
            pub struct User {
                id: String,
                name: String,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for User {
                #[inline]
                fn clone(&self) -> User {
                    User {
                        id: ::core::clone::Clone::clone(&self.id),
                        name: ::core::clone::Clone::clone(&self.name),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for User {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for User {
                #[inline]
                fn eq(&self, other: &User) -> bool {
                    self.id == other.id && self.name == other.name
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for User {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "User",
                        "id",
                        &self.id,
                        "name",
                        &&self.name,
                    )
                }
            }
            impl User {
                pub fn new(name: String) -> Self {
                    Self {
                        id: Uuid::default().to_string(),
                        name,
                    }
                }
                pub fn id(&self) -> &String {
                    &self.id
                }
            }
        }
    }
    pub mod repository {
        pub mod product {
            use crate::domain::model::product::Product;
            pub trait ProductRepository: Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn get_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<Product>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn create_product<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                    price: f64,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Product>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
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
        pub mod user {
            use crate::domain::model::user::User;
            pub trait UserRepository: Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn get_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<User>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn create_user<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<User>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
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
    }
    pub mod service {
        pub mod product {
            use std::sync::Arc;
            use crate::domain::model::product::Product;
            pub trait ProductService: Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn get_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<Product>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn create_product<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                    price: f64,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Product>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
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
            impl<T> ProductService for Arc<T>
            where
                T: ProductService,
            {
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
                fn get_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<Product>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Option<Product>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let product_id = product_id;
                        let __ret: anyhow::Result<Option<Product>> = {
                            <T as ProductService>::get_product(__self, product_id).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_product<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                    price: f64,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Product>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Product>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let price = price;
                        let __ret: anyhow::Result<Product> = {
                            <T as ProductService>::create_product(__self, name, price)
                                .await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
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
                        let product_id = product_id;
                        let __ret: anyhow::Result<()> = {
                            <T as ProductService>::delete_product(__self, product_id)
                                .await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub mod user {
            use std::sync::Arc;
            use crate::domain::model::user::User;
            pub trait UserService: Sync + Send {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn get_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<User>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn create_user<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<User>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
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
            impl<T> UserService for Arc<T>
            where
                T: UserService,
            {
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
                fn get_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Option<User>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Option<User>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let user_id = user_id;
                        let __ret: anyhow::Result<Option<User>> = {
                            <T as UserService>::get_user(__self, user_id).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_user<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<User>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<User>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let __ret: anyhow::Result<User> = {
                            <T as UserService>::create_user(__self, name).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
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
                        let user_id = user_id;
                        let __ret: anyhow::Result<()> = {
                            <T as UserService>::delete_user(__self, user_id).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
    }
}
pub mod infrastracture {
    pub mod repository {
        pub mod product {
            use std::{collections::HashMap, sync::Arc};
            use tokio::sync::Mutex;
            use crate::domain::model::product::Product;
            use crate::domain::repository::product::ProductRepository;
            pub struct MemoryProductRepository {
                inner: Arc<Mutex<HashMap<String, Product>>>,
            }
            impl MemoryProductRepository {
                pub fn new() -> Self {
                    Self {
                        inner: Arc::new(Mutex::new(HashMap::default())),
                    }
                }
            }
            impl ProductRepository for MemoryProductRepository {
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
                fn get_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<
                                Option<crate::domain::model::product::Product>,
                            >,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<
                                Option<crate::domain::model::product::Product>,
                            >,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let product_id = product_id;
                        let __ret: anyhow::Result<
                            Option<crate::domain::model::product::Product>,
                        > = {
                            let opt_product = {
                                let guard = __self.inner.lock().await;
                                guard.get(&product_id).cloned()
                            };
                            Ok(opt_product)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_product<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                    price: f64,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Product>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Product>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let price = price;
                        let __ret: anyhow::Result<Product> = {
                            let product = Product::new(name, price);
                            {
                                let mut guard = __self.inner.lock().await;
                                if guard.contains_key(product.id()) {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "already exist user [user_id: {0:?}]",
                                                        product.id(),
                                                    ),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                guard.insert(product.id().clone(), product.clone());
                            }
                            Ok(product)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
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
                        let product_id = product_id;
                        let __ret: anyhow::Result<()> = {
                            {
                                let mut guard = __self.inner.lock().await;
                                if let None = guard.remove(&product_id) {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "not found product [product_id: {0:?}]",
                                                        &product_id,
                                                    ),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                            }
                            Ok(())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub mod user {
            use std::{collections::HashMap, sync::Arc};
            use tokio::sync::Mutex;
            use crate::domain::{model::user::User, repository::user::UserRepository};
            pub struct MemoryUserRepository {
                inner: Arc<Mutex<HashMap<String, User>>>,
            }
            impl MemoryUserRepository {
                pub fn new() -> Self {
                    Self {
                        inner: Arc::new(Mutex::new(HashMap::default())),
                    }
                }
            }
            impl UserRepository for MemoryUserRepository {
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
                fn get_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<
                                Option<crate::domain::model::user::User>,
                            >,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Option<crate::domain::model::user::User>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let user_id = user_id;
                        let __ret: anyhow::Result<
                            Option<crate::domain::model::user::User>,
                        > = {
                            let opt_user = {
                                let guard = __self.inner.lock().await;
                                guard.get(&user_id).cloned()
                            };
                            Ok(opt_user)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_user<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<User>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<User>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let __ret: anyhow::Result<User> = {
                            let user = User::new(name);
                            {
                                let mut guard = __self.inner.lock().await;
                                if guard.contains_key(user.id()) {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "already exist user [user_id: {0:?}]",
                                                        user.id(),
                                                    ),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                guard.insert(user.id().clone(), user.clone());
                            }
                            Ok(user)
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
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
                        let user_id = user_id;
                        let __ret: anyhow::Result<()> = {
                            {
                                let mut guard = __self.inner.lock().await;
                                if let None = guard.remove(&user_id) {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("not found user [user_id: {0:?}]", &user_id),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                            }
                            Ok(())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
    }
    pub mod service {
        pub mod product {
            use crate::domain::{
                model::product::Product, repository::product::ProductRepository,
                service::product::ProductService,
            };
            pub struct ProductServiceImpl<T> {
                product_repository: T,
            }
            impl<T> ProductServiceImpl<T> {
                pub fn new(product_repository: T) -> Self {
                    Self { product_repository }
                }
            }
            impl<T> ProductService for ProductServiceImpl<T>
            where
                T: ProductRepository,
            {
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
                fn get_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<
                                Option<crate::domain::model::product::Product>,
                            >,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<
                                Option<crate::domain::model::product::Product>,
                            >,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let product_id = product_id;
                        let __ret: anyhow::Result<
                            Option<crate::domain::model::product::Product>,
                        > = { __self.product_repository.get_product(product_id).await };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_product<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                    price: f64,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<Product>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Product>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let price = price;
                        let __ret: anyhow::Result<Product> = {
                            __self.product_repository.create_product(name, price).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_product<'life0, 'async_trait>(
                    &'life0 self,
                    product_id: String,
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
                        let product_id = product_id;
                        let __ret: anyhow::Result<()> = {
                            __self.product_repository.delete_product(product_id).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub mod user {
            use crate::domain::{
                model::user::User, repository::user::UserRepository,
                service::user::UserService,
            };
            pub struct UserServiceImpl<T> {
                user_repository: T,
            }
            impl<T> UserServiceImpl<T> {
                pub fn new(user_repository: T) -> Self {
                    Self { user_repository }
                }
            }
            impl<T> UserService for UserServiceImpl<T>
            where
                T: UserRepository,
            {
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
                fn get_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<
                                Option<crate::domain::model::user::User>,
                            >,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<Option<crate::domain::model::user::User>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let user_id = user_id;
                        let __ret: anyhow::Result<
                            Option<crate::domain::model::user::User>,
                        > = { __self.user_repository.get_user(user_id).await };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn create_user<'life0, 'async_trait>(
                    &'life0 self,
                    name: String,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = anyhow::Result<User>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            anyhow::Result<User>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let name = name;
                        let __ret: anyhow::Result<User> = {
                            __self.user_repository.create_user(name).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
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
                fn delete_user<'life0, 'async_trait>(
                    &'life0 self,
                    user_id: String,
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
                        let user_id = user_id;
                        let __ret: anyhow::Result<()> = {
                            __self.user_repository.delete_user(user_id).await
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
    }
}
