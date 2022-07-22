# gmsv_dot_env

A not-so-true-to-life implementation of a .env file for gmod. Place a .env at `./garrysmod/.env` and any variables set in it will be accessible from within Lua through the `env()` function. The file will require you to run `require("dot_env")` at the top of a config file.

This does not respect ENVIRONMENT_VARIABLES defined outside of the .env. My intentions were (And always will be) to keep the context of the variables tight and strict, so they are only creatable through the .env file.

Also, this will only work on server side. So if you require it inside a shared file, make sure to account for that.

*Also, idk wtf I'm doing with Rust and this is probably a low key mess*

## Install

1. Place the dll in `./garrysmod/lua/bin`.
2. Create a `.env` file at `./garrysmod/.env`.
3. Populate the .env file with an appropriate config.
4. Go to a config file and require the DLL at the top of the file: `if SERVER then require('dot_env') end`
5. Set the config value using the `env()` function: `MyAddon.Config.SQL.Password = env('MY_ADDON_SQL_PASSWORD') or "password"`.
6. If you are using this in a shared context, you may need to account for that like so: `MyAddon.Config.SharedValue = SERVER and (env('MY_ADDON_SHARED_VAL') or "Fallback") or "Client Fallback"`, but it is unlikely you would be using this in a shared context.

## Credits

- [WilliamVenner](https://github.com/WilliamVenner) - Making gmod-rs. Digging through his endless Gmod Rust DLL repos in order to understand how gmod-rs works.
- [dotenv-parser](https://github.com/rubik/dotenv-parser) - Parsing the .env file. Because why make something yourself when someone has already done it better?
