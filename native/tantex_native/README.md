# Nif for Elixir.Tantex.Native

## To build the NIF module:

  * Make sure your projects `mix.exs` has the `:rustler` compiler listed in the `project` function: `compilers: [:rustler] ++ Mix.compilers` If there already is a `:compilers` list, you should append `:rustler` to it.
  * Add your crate to the `rustler_crates` attribute in the `project function. [See here](https://hexdocs.pm/rustler/basics.html#crate-configuration).
  * Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule Tantex.Native do
    use Rustler, otp_app: [otp app], crate: "tantex_native"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples
[This](https://github.com/hansihe/NifIo) is a complete example of a NIF written in Rust.


## TODOs

  + add lookup by term
  + add delete document by term
  + add update document by term
  + add insert document by unique term
