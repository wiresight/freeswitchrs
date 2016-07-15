#[macro_use]
extern crate freeswitchrs;

use freeswitchrs::raw as fsr;
use freeswitchrs::mods::*; // This will get replaced with a mods prelude
use freeswitchrs::Status;

fn my_load(mod_int: &ModInterface) -> Status {
    mod_int.add_raw_api("skelr", "Example doc", "skelr", skelr_api);
    Ok(())
}

static MOD_SKELR_DEF: ModDefinition = ModDefinition {
    name: "mod_skelr",
    load: my_load,
    shutdown: None,
    runtime: None,
};

freeswitch_export_mod!(mod_skelr_module_interface, MOD_SKELR_DEF);

#[allow(unused_variables)]
unsafe extern "C" fn skelr_api(cmd: *const std::os::raw::c_char,
                               session: *mut fsr::core_session,
                               stream: *mut fsr::stream_handle)
                               -> fsr::status {
    (*stream).write_function.unwrap()(stream, fsr::str_to_ptr("OK"));
    fsr::status::SUCCESS
}
