// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![deny(warnings)]
use clap::{App, AppSettings, Arg, SubCommand};
use log::info;

use nitro_cli::common::commands_parser::{
    BuildEnclavesArgs, ConsoleArgs, DescribeEnclaveArgs, RunEnclavesArgs, TerminateEnclavesArgs,
};
use nitro_cli::common::enclave_proc_command_send_single;
use nitro_cli::common::logger;
use nitro_cli::common::{EnclaveProcessCommandType, ExitGracefully};
use nitro_cli::create_app;
use nitro_cli::enclave_proc_comm::enclave_proc_spawn;
#[cfg(feature = "power_user")]
use nitro_cli::testing_commands;
use nitro_cli::utils::handle_signals;
use nitro_cli::{
    build_enclaves, console_enclaves, describe_enclaves, run_enclaves, terminate_enclaves,
};

fn main() {
    // Command line specification for NitroEnclaves CLI.
    let logger = logger::init_logger();

    logger.update_logger_id(format!("nitro-cli:{}", std::process::id()).as_str());
    info!("Start Nitro CLI");

    let app = create_app!();
    #[cfg(feature = "power_user")]
    let app = testing_commands::initialize(app);
    let args = app.get_matches();

    #[cfg(feature = "power_user")]
    testing_commands::match_cmd(&args);

    match args.subcommand() {
        ("run-enclave", Some(args)) => {
            handle_signals();
            let run_args = RunEnclavesArgs::new_with(args).ok_or_exit(args.usage());
            let mut comm = enclave_proc_spawn(&logger).expect("Enclave process spawning failed.");
            enclave_proc_command_send_single(
                &EnclaveProcessCommandType::Run,
                Some(&run_args),
                &mut comm,
            )
            .expect("Failed to send single command.");
            // TODO: Move enclave run into the newly-spawned enclave process.
            run_enclaves(run_args).ok_or_exit(args.usage());
        }
        ("terminate-enclave", Some(args)) => {
            handle_signals();
            let terminate_args = TerminateEnclavesArgs::new_with(args).ok_or_exit(args.usage());
            terminate_enclaves(terminate_args).ok_or_exit(args.usage());
        }
        ("describe-enclaves", Some(args)) => {
            let describe_args = DescribeEnclaveArgs::new_with(args).ok_or_exit(args.usage());
            describe_enclaves(describe_args).ok_or_exit(args.usage());
        }
        ("build-enclave", Some(args)) => {
            let build_args = BuildEnclavesArgs::new_with(args).ok_or_exit(args.usage());
            build_enclaves(build_args).ok_or_exit(args.usage());
        }
        ("console", Some(args)) => {
            let console_args = ConsoleArgs::new_with(args).ok_or_exit(args.usage());
            console_enclaves(console_args).ok_or_exit(args.usage());
        }
        (&_, _) => {}
    }
}
