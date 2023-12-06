// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    locked_token_pos_creator
    (
        init => init
        upgrade => upgrade
        createEnergyPosition => create_energy_position
        setEnergyFactoryAddress => set_energy_factory_address
        getEnergyFactoryAddress => energy_factory_address
        createPairPosFromSingleToken => create_pair_pos_from_single_token_endpoint
        createPairPosFromTwoTokens => create_pair_pos_from_two_tokens_endpoint
        createFarmPosFromSingleToken => create_farm_pos_from_single_token
        createFarmPosFromTwoTokens => create_farm_pos_from_two_tokens
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
