#![feature(c_unwind)]
use std::fs;
use dotenv_parser::parse_dotenv;

#[macro_use]
extern crate gmod;

unsafe extern "C-unwind" fn read_env(lua: gmod::lua::State) -> i32 {
    let contents = fs::read_to_string("./garrysmod/.env")
        .expect("./garrysmod/.env does not seem to exist");

    let env_vars = parse_dotenv(&String::from(contents)).unwrap();
    let lookup_var = String::from(lua.check_string(1));

    if env_vars.contains_key(&lookup_var) {
        let desired_var = &env_vars[&lookup_var];
        lua.push_string(desired_var);
    } else {
        lua.push_nil();
    }

    return 1;
}

#[gmod13_open]
pub unsafe extern "C-unwind" fn gmod13_open(lua: gmod::lua::State) -> i32 {
    lua.push_function(read_env);
    lua.set_global(lua_string!("env"));

    lua.pop();

    return 0;
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    return 0;
}
